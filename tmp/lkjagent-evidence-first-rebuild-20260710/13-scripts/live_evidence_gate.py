#!/usr/bin/env python3
from __future__ import annotations

import csv
import hashlib
import math
import sqlite3
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from live_db_checks import check_database
from live_source_checks import validate_run_binding
from scenario_policy import (
    ANCHORED_CHECKS,
    ANCHORED_EVENT_KINDS,
    MINIMUM_DECISION_SPAN_SECONDS,
    MINIMUM_DURATION_SECONDS,
    MINIMUM_OWNER_SPAN_SECONDS,
    MINIMUM_OWNER_TURNS,
)


def fields(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def rows(path: Path) -> list[dict[str, str]]:
    return list(csv.DictReader(path.open(encoding="utf-8"), delimiter="\t"))


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def check_source_contract(
    run: Path,
    result: dict[str, str],
    scenario_path: Path,
    errors: list[str],
) -> tuple[dict[str, str], Path]:
    expected_matters = scenario_path.parent / "matters.tsv"
    source_schedule = scenario_path.parent / "owner-schedule.tsv"
    for path in (scenario_path, expected_matters, source_schedule):
        if not path.is_file():
            errors.append(f"source scenario file missing: {path.name}")
    if errors:
        return {}, expected_matters
    scenario = fields(scenario_path)
    scenario_id = scenario.get("scenario_id", "")
    if scenario_id not in ANCHORED_CHECKS:
        errors.append("scenario is not an anchored required scenario")
    if result.get("scenario_id") != scenario.get("scenario_id"):
        errors.append("scenario ID mismatch")
    if result.get("scenario_fingerprint") != digest(scenario_path):
        errors.append("scenario fingerprint mismatch")
    run_schedule = run / "owner-schedule.tsv"
    if not run_schedule.is_file() or digest(run_schedule) != digest(source_schedule):
        errors.append("owner schedule differs from source scenario")
    if any(row["expected_terminal"].lower() != "completed" for row in rows(expected_matters)):
        errors.append("required final scenario declares a non-completed terminal")
    return scenario, expected_matters


def check_timeline(
    run: Path, scenario: dict[str, str], errors: list[str]
) -> dict[str, tuple[str, str, str, int]]:
    event_path = run / "events.tsv"
    schedule_path = run / "owner-schedule.tsv"
    if not event_path.is_file() or not schedule_path.is_file():
        errors.append("events.tsv or owner-schedule.tsv missing")
        return {}
    events = rows(event_path)
    schedule = rows(schedule_path)
    minimum_turns = max(MINIMUM_OWNER_TURNS, int(scenario.get("minimum_owner_turns", "3")))
    if len(schedule) < minimum_turns:
        errors.append("owner schedule below scenario minimum")
    moments = [float(row["monotonic_seconds"]) for row in events]
    if any(not math.isfinite(moment) for moment in moments) or moments != sorted(moments):
        errors.append("event times are not finite and monotonic")
    span = max(moments) - min(moments) if moments else 0
    if span < max(MINIMUM_DURATION_SECONDS, int(scenario.get("minimum_duration_seconds", "840"))):
        errors.append("raw event span below scenario minimum")
    offsets = [float(row["offset_seconds"]) for row in schedule]
    minimum_owner_span = max(
        MINIMUM_OWNER_SPAN_SECONDS,
        int(scenario.get("minimum_owner_span_seconds", "600")),
    )
    if not offsets or max(offsets) - min(offsets) < minimum_owner_span:
        errors.append("owner goals do not span the session")
    kinds = [row["kind"] for row in events]
    missing_kinds = ANCHORED_EVENT_KINDS.get(scenario.get("scenario_id", ""), set()) - set(kinds)
    if missing_kinds:
        errors.append(f"anchored scenario events missing: {sorted(missing_kinds)}")
    if kinds.count("session.start") != 1 or kinds.count("session.end") != 1:
        errors.append("session boundary events missing")
    elif kinds[0] != "session.start" or kinds[-1] != "session.end":
        errors.append("session boundaries are not first and last")
    if kinds.count("owner.turn") != len(schedule):
        errors.append("schedule and raw owner events disagree")
    owner_events = [row for row in events if row["kind"] == "owner.turn"]
    start = moments[0] if moments else 0
    for scheduled, actual in zip(schedule, owner_events, strict=False):
        timing = float(actual["monotonic_seconds"]) - start
        if abs(timing - float(scheduled["offset_seconds"])) > 5:
            errors.append("scheduled owner turn timing mismatch")
        if actual.get("ref") != scheduled.get("text_fingerprint"):
            errors.append("scheduled owner turn fingerprint mismatch")
    ids = [row.get("event_id", "") for row in events]
    if not all(ids) or len(ids) != len(set(ids)):
        errors.append("event IDs are empty or duplicated")
    return {
        row["event_id"]: (
            row["kind"],
            row.get("matter_id", ""),
            row.get("ref", ""),
            round(float(row["monotonic_seconds"]) * 1000),
        )
        for row in events
    }


def main() -> int:
    if len(sys.argv) != 4:
        print(
            "usage: live_evidence_gate.py RUN_DIR SOURCE_COMMIT SCENARIO.tsv",
            file=sys.stderr,
        )
        return 2
    run = Path(sys.argv[1])
    source_commit = sys.argv[2]
    scenario_path = Path(sys.argv[3])
    result_path = run / "result.tsv"
    database = run / "run.sqlite3"
    errors: list[str] = []
    if not result_path.is_file() or not database.is_file():
        errors.append("result.tsv or run.sqlite3 missing")
    else:
        result = fields(result_path)
        validate_run_binding(run, result, scenario_path, errors)
        for path, key in (
            (database, "database_fingerprint"),
            (run / "events.tsv", "events_fingerprint"),
        ):
            if path.is_file() and result.get(key) != digest(path):
                errors.append(f"{key} mismatch")
        if result.get("source_commit") != source_commit:
            errors.append("run source commit mismatch")
        if result.get("snapshot_method") != "sqlite-online-backup":
            errors.append("run did not declare the required SQLite backup method")
        manifest = run / "workspace-manifest.tsv"
        if not manifest.is_file():
            errors.append("workspace manifest missing")
        elif result.get("workspace_manifest_fingerprint") != digest(manifest):
            errors.append("workspace manifest fingerprint mismatch")
        scenario, expected = check_source_contract(
            run, result, scenario_path, errors
        )
        if scenario:
            trace_ids = check_timeline(run, scenario, errors)
            required_checks = {
                item
                for item in scenario.get("required_check_ids", "").split(",")
                if item
            }
            required_checks |= ANCHORED_CHECKS.get(scenario.get("scenario_id", ""), set())
            decision_span = max(MINIMUM_DECISION_SPAN_SECONDS, int(
                scenario.get("minimum_decision_span_seconds", "600")
            )) * 1000
            check_database(
                database,
                run / "workspace",
                expected,
                required_checks,
                decision_span,
                trace_ids,
                result.get("run_id", ""),
                errors,
            )
    if errors:
        for error in errors:
            print(f"FAIL\t{run}\t{error}")
        return 1
    print(f"PASS\t{run}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
