from __future__ import annotations

import datetime as dt
import csv
import hashlib
import json
import re
import sqlite3
from pathlib import Path


HASH = re.compile(r"^sha256:[0-9a-f]{64}$")


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def bundle_digest(directory: Path) -> str:
    value = hashlib.sha256()
    for name in (
        "scenario.tsv",
        "matters.tsv",
        "owner-schedule.tsv",
        "seed-manifest.tsv",
        "checks.tsv",
    ):
        path = directory / name
        value.update(name.encode())
        value.update(b"\0")
        value.update(path.read_bytes())
        value.update(b"\0")
    return "sha256:" + value.hexdigest()


def instant(value: str) -> dt.datetime:
    parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("wall instant has no offset")
    return parsed


def validate_run_binding(
    run: Path,
    result: dict[str, str],
    scenario_path: Path,
    errors: list[str],
) -> None:
    scenario_root = scenario_path.parent
    for name in (
        "scenario.tsv",
        "matters.tsv",
        "owner-schedule.tsv",
        "seed-manifest.tsv",
        "checks.tsv",
    ):
        if not (scenario_root / name).is_file():
            errors.append(f"source scenario file missing: {name}")
            return
    if result.get("scenario_bundle_fingerprint") != bundle_digest(scenario_root):
        errors.append("scenario bundle fingerprint mismatch")
    if result.get("seed_fingerprint") != digest(scenario_root / "seed-manifest.tsv"):
        errors.append("seed manifest fingerprint mismatch")
    configuration = run / "configuration.json"
    if not configuration.is_file():
        errors.append("effective configuration evidence missing")
    else:
        try:
            data = json.loads(configuration.read_text(encoding="utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            errors.append(f"effective configuration invalid: {error}")
            data = None
        if not isinstance(data, dict) or any(
            type(value) not in {str, int, bool} for value in (data or {}).values()
        ):
            errors.append("effective configuration is not one flat scalar object")
        if result.get("configuration_fingerprint") != digest(configuration):
            errors.append("effective configuration fingerprint mismatch")
    if not HASH.fullmatch(result.get("binary_fingerprint", "")):
        errors.append("binary fingerprint missing or malformed")
    if not result.get("model_identifier") or not result.get("run_id"):
        errors.append("model identifier or run ID missing")
    if result.get("status") != "completed":
        errors.append("required final run status is not completed")
    try:
        elapsed = (instant(result["end_utc"]) - instant(result["start_utc"])).total_seconds()
        if elapsed < 840:
            errors.append("wall-clock run duration below 840 seconds")
    except (KeyError, ValueError) as error:
        errors.append(f"wall-clock binding invalid: {error}")
    for suffix in ("-wal", "-shm"):
        if (run / f"run.sqlite3{suffix}").exists():
            errors.append("ambiguous SQLite sidecar included with online backup")
    process_evidence(run, result, errors)


def process_evidence(
    run: Path, result: dict[str, str], errors: list[str]
) -> None:
    lifecycle_path = run / "process-lifecycle.tsv"
    provider_path = run / "provider-manifest.tsv"
    runner_log = run / "runner.log"
    if not lifecycle_path.is_file() or not provider_path.is_file() or not runner_log.is_file():
        errors.append("process lifecycle, provider manifest, or runner log missing")
        return
    lifecycle = list(csv.DictReader(lifecycle_path.open(encoding="utf-8"), delimiter="\t"))
    if [row.get("event") for row in lifecycle] != ["start", "end"]:
        errors.append("process lifecycle does not contain exact start/end events")
    else:
        try:
            span = int(lifecycle[1]["monotonic_ns"]) - int(lifecycle[0]["monotonic_ns"])
            pids = {int(row["pid"]) for row in lifecycle}
        except (KeyError, ValueError):
            span, pids = 0, set()
        if span < 840_000_000_000 or len(pids) != 1 or next(iter(pids), 0) <= 1:
            errors.append("process lifecycle PID or monotonic span invalid")
        for row in lifecycle:
            if row.get("run_id") != result.get("run_id") or row.get(
                "binary_fingerprint"
            ) != result.get("binary_fingerprint"):
                errors.append("process lifecycle run or binary binding mismatch")
    if runner_log.stat().st_size < 200 or result.get("runner_log_fingerprint") != digest(runner_log):
        errors.append("runner log missing content or fingerprint binding")
    manifest = list(csv.DictReader(provider_path.open(encoding="utf-8"), delimiter="\t"))
    if len(manifest) < 3:
        errors.append("provider manifest has fewer than three exchanges")
        return
    expected = {
        row[0]: (row[1], row[2], int(row[3]), int(row[4]))
        for row in sqlite3.connect(f"file:{run / 'run.sqlite3'}?mode=ro", uri=True).execute(
            "SELECT id,request_fingerprint,response_fingerprint,started_monotonic_ns,"
            "ended_monotonic_ns FROM provider_exchanges"
        )
    }
    actual: dict[str, tuple[str, str, int, int]] = {}
    try:
        for row in manifest:
            actual[row["exchange_id"]] = (
                row["request_fingerprint"], row["response_fingerprint"],
                int(row["started_monotonic_ns"]), int(row["ended_monotonic_ns"]),
            )
    except (KeyError, ValueError):
        errors.append("provider manifest values invalid")
        return
    if len(actual) != len(manifest) or actual != expected:
        errors.append("provider manifest and SQLite exchange tuples disagree")
    if any(not HASH.fullmatch(item) for row in actual.values() for item in row[:2]):
        errors.append("provider request or response fingerprint malformed")
    if any(row[3] <= row[2] for row in actual.values()):
        errors.append("provider exchange timing is nonpositive")
