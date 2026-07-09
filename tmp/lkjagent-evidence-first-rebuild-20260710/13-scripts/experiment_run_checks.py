from __future__ import annotations

import csv
import hashlib
import json
import sqlite3
import subprocess
from pathlib import Path

from experiment_metrics import metrics as load_metrics


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def values(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def validate_run(
    repo: Path,
    run: Path,
    row: dict[str, str],
    source_commit: str,
    errors: list[str],
) -> tuple[str, str, dict[str, float]] | None:
    result_path = run / "result.tsv"
    database = run / "run.sqlite3"
    metrics_path = run / "metrics.tsv"
    if not result_path.is_file() or not database.is_file() or not metrics_path.is_file():
        errors.append(f"experiment raw evidence incomplete: {row['evidence_ref']}")
        return None
    result = values(result_path)
    expected = {
        "experiment_id": row["experiment_id"],
        "scenario_id": row["scenario_id"],
        "configuration_fingerprint": row["configuration_fingerprint"],
        "repeat": row["repeat"],
        "source_commit": row["tested_commit"],
    }
    if any(result.get(key) != value for key, value in expected.items()):
        errors.append(f"experiment result binding mismatch: {row['evidence_ref']}")
    ancestor = subprocess.run(
        ["git", "-C", str(repo), "merge-base", "--is-ancestor", row["tested_commit"], source_commit],
        capture_output=True,
        check=False,
    )
    if ancestor.returncode:
        errors.append(f"experiment commit is not in frozen history: {row['evidence_ref']}")
    database_hash = digest(database)
    if row["database_fingerprint"] != database_hash:
        errors.append(f"experiment database fingerprint mismatch: {row['evidence_ref']}")
    configuration = run / "configuration.json"
    canonical = ""
    try:
        parsed = json.loads(configuration.read_text(encoding="utf-8"))
        if not isinstance(parsed, dict) or any(
            type(value) not in {str, int, bool} for value in parsed.values()
        ):
            raise ValueError("effective configuration is not flat scalar JSON")
        canonical = json.dumps(
            parsed, sort_keys=True, separators=(",", ":"), ensure_ascii=False
        ) + "\n"
        if configuration.read_text(encoding="utf-8") != canonical:
            raise ValueError("effective configuration bytes are not canonical")
        if digest(configuration) != row["configuration_fingerprint"]:
            raise ValueError("effective configuration fingerprint mismatch")
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        errors.append(f"experiment configuration invalid: {row['evidence_ref']}: {error}")
    event_path = run / "events.tsv"
    try:
        events = list(csv.DictReader(event_path.open(encoding="utf-8"), delimiter="\t"))
        moments = [float(item["monotonic_seconds"]) for item in events]
        if not moments or moments != sorted(moments) or moments[-1] - moments[0] < 840:
            raise ValueError("event trace is not an ordered 840-second run")
        if sum(item["kind"] == "owner.turn" for item in events) < 3:
            raise ValueError("fewer than three owner turns")
    except (OSError, KeyError, ValueError) as error:
        errors.append(f"experiment event trace invalid: {row['evidence_ref']}: {error}")
    db = sqlite3.connect(f"file:{database}?mode=ro", uri=True)
    if db.execute("PRAGMA integrity_check").fetchone()[0] != "ok" or list(
        db.execute("PRAGMA foreign_key_check")
    ):
        errors.append(f"experiment SQLite invalid: {row['evidence_ref']}")
    first, last, decisions = db.execute(
        "SELECT MIN(selected_monotonic_ms),MAX(selected_monotonic_ms),COUNT(*) "
        "FROM runtime_decisions"
    ).fetchone()
    if not decisions or decisions < 5 or last - first < 600000:
        errors.append(f"experiment lacks real decision activity: {row['evidence_ref']}")
    try:
        measured = load_metrics(metrics_path)
    except ValueError as error:
        errors.append(str(error))
        measured = {}
    return canonical, database_hash, measured
