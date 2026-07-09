#!/usr/bin/env python3
from __future__ import annotations

import csv
import hashlib
import re
import sqlite3
import statistics
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from tui_cast_checks import REQUIRED_TRACE_EVENTS, check_cast, check_replay
from tui_db_checks import check_database_binding, database_messages
from live_source_checks import process_evidence


def fields(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def comma(value: str) -> list[str]:
    return [item for item in value.split(",") if item]


def result_check(
    directory: Path,
    result: dict[str, str],
    trace: Path,
    source_commit: str,
    errors: list[str],
) -> None:
    recording = directory / "terminal.cast"
    if result.get("source_commit") != source_commit:
        errors.append("PTY source commit mismatch")
    if result.get("scenario_id") != "terminal-operator":
        errors.append("PTY scenario ID mismatch")
    if result.get("pty_backend") != "unix-pty":
        errors.append("real PTY backend not recorded")
    if result.get("trace_fingerprint") != digest(trace):
        errors.append("PTY trace fingerprint mismatch")
    if not recording.is_file():
        errors.append("terminal recording missing")
    elif result.get("recording_fingerprint") != digest(recording):
        errors.append("terminal recording fingerprint mismatch")


def trace_check(
    trace: Path,
    messages: dict[str, tuple[int, str]],
    cast_duration: float,
    errors: list[str],
) -> None:
    rows = list(csv.DictReader(trace.open(encoding="utf-8"), delimiter="\t"))
    if len(rows) < 100:
        errors.append("fewer than 100 PTY trace rows")
        return
    moments = [float(row["monotonic_seconds"]) for row in rows]
    if moments != sorted(moments) or max(moments) - min(moments) < 840:
        errors.append("PTY trace span below 840 seconds")
    cast_offsets = [float(row["cast_offset_seconds"]) for row in rows]
    if cast_offsets != sorted(cast_offsets) or cast_offsets[-1] > cast_duration + 0.5:
        errors.append("PTY trace is not ordered within terminal cast")
    events = [row["event"] for row in rows]
    missing = REQUIRED_TRACE_EVENTS - set(events)
    if missing:
        errors.append(f"required PTY events missing: {sorted(missing)}")
    if events.count("owner_input") < 3:
        errors.append("fewer than three ordinary owner inputs")
    latencies: list[int] = []
    visible_frames = 0
    last_follow_sequence = -1
    prior: dict[str, str] | None = None
    for number, row in enumerate(rows, 2):
        top = int(row["top"])
        maximum = int(row["max_top"])
        expected_maximum = max(0, int(row["wrapped_rows"]) - int(row["viewport_height"]))
        if maximum != expected_maximum:
            errors.append(f"line {number}: max_top disagrees with captured geometry")
        if top < 0 or top > maximum:
            errors.append(f"line {number}: top outside bounds")
        ids = comma(row["visible_ids"])
        sequences = [int(item) for item in comma(row["visible_sequences"])]
        roles = comma(row["visible_roles"])
        if ids:
            visible_frames += 1
        if len(ids) != len(set(ids)):
            errors.append(f"line {number}: duplicate visible logical ID")
        if len(ids) != len(sequences) or len(ids) != len(roles):
            errors.append(f"line {number}: visible columns disagree")
            continue
        if sequences != sorted(sequences):
            errors.append(f"line {number}: causal inversion inside viewport")
        if len(sequences) != len(set(sequences)):
            errors.append(f"line {number}: duplicate visible sequence")
        for identifier, sequence, role in zip(ids, sequences, roles, strict=True):
            if messages.get(identifier) != (sequence, role):
                errors.append(f"line {number}: message does not match SQLite")
        if row["follow"] == "true":
            if top != maximum:
                errors.append(f"line {number}: follow not bottom anchored")
            if sequences and sequences[-1] < last_follow_sequence:
                errors.append(f"line {number}: followed sequence moved backward")
            if sequences:
                last_follow_sequence = sequences[-1]
        expected = row.get("composer_expected_hash", "")
        if expected and row.get("composer_hash") != expected:
            errors.append(f"line {number}: composer content lost")
        if row.get("forbidden_diagnostic_count") != "0":
            errors.append(f"line {number}: ordinary view contains diagnostics")
        screen_hash = row.get("screen_hash", "")
        if not re.fullmatch(r"sha256:[0-9a-f]{64}", screen_hash):
            errors.append(f"line {number}: screen hash missing")
        latency = row.get("input_latency_ms", "")
        if latency:
            measured = int(latency)
            if measured < 0:
                errors.append(f"line {number}: negative input latency")
            latencies.append(measured)
        if prior and row["event"] == "scroll_up":
            wanted = max(0, int(prior["max_top"]) - 1)
            if prior["follow"] != "true" or row["follow"] != "false" or top != wanted:
                errors.append(f"line {number}: scroll-up transition invalid")
        if prior and row["event"] == "scroll_down" and top == maximum:
            if row["follow"] != "true":
                errors.append(f"line {number}: bottom scroll did not restore follow")
        if prior and row["event"] == "agent_update" and prior["follow"] == "true":
            if row["follow"] != "true" or top != maximum:
                errors.append(f"line {number}: followed update lost bottom")
        if prior and row["event"] == "agent_update" and prior["follow"] == "false":
            if row.get("anchor_id") != prior.get("anchor_id"):
                errors.append(f"line {number}: manual anchor changed on update")
        if row["event"] in {"slow_call_input", "resize", "daemon_restart", "workbench_restart"}:
            if not row.get("composer_expected_hash"):
                errors.append(f"line {number}: composer checkpoint missing")
        prior = row
    if visible_frames < 10:
        errors.append("too few nonempty transcript frames")
    if len(latencies) < 20:
        errors.append("too few input latency samples")
    else:
        ordered = sorted(latencies)
        p95 = ordered[max(0, int(len(ordered) * 0.95) - 1)]
        if p95 > 250:
            errors.append(f"p95 input latency too high: {p95}ms")
        print(f"latency_median_ms\t{statistics.median(latencies)}")
        print(f"latency_p95_ms\t{p95}")


def main() -> int:
    if len(sys.argv) != 5:
        print(
            "usage: tui_trace_gate.py TRACE SQLITE RESULT SOURCE_COMMIT",
            file=sys.stderr,
        )
        return 2
    trace = Path(sys.argv[1])
    database = Path(sys.argv[2])
    result_path = Path(sys.argv[3])
    source_commit = sys.argv[4]
    errors: list[str] = []
    try:
        if not trace.is_file() or not database.is_file() or not result_path.is_file():
            raise ValueError("PTY trace, database, or result missing")
        result = fields(result_path)
        result_check(trace.parent, result, trace, source_commit, errors)
        process_evidence(trace.parent, result, errors)
        check_replay(trace.parent, result, source_commit, errors)
        check_database_binding(database, result, errors)
        cast_duration, _ = check_cast(trace.parent / "terminal.cast", errors)
        trace_check(trace, database_messages(database), cast_duration, errors)
    except (ValueError, sqlite3.Error, KeyError) as error:
        errors.append(str(error))
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print("PASS\tPTY trace and conversation database")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
