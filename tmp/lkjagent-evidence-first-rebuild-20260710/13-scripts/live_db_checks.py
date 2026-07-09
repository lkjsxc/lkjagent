#!/usr/bin/env python3
from __future__ import annotations

import csv
import hashlib
import sqlite3
from pathlib import Path

from scenario_policy import (
    MINIMUM_DECISIONS,
    MINIMUM_PROGRESS_DECISIONS,
    MINIMUM_USEFUL_DECISIONS,
)
from live_relational_checks import linkage_check


def count(db: sqlite3.Connection, sql: str, params: tuple[object, ...] = ()) -> int:
    return int(db.execute(sql, params).fetchone()[0])


def tsv_rows(path: Path) -> list[dict[str, str]]:
    return list(csv.DictReader(path.open(encoding="utf-8"), delimiter="\t"))


def schema_check(db: sqlite3.Connection, errors: list[str]) -> bool:
    required = {
        "runs",
        "matters",
        "obligations",
        "operations",
        "runtime_events",
        "runtime_decisions",
        "context_frames",
        "provider_exchanges",
        "tool_admissions",
        "effect_journal",
        "failure_lineages",
        "observations",
        "checks",
        "workspace_documents",
        "workspace_revisions",
        "conversation_messages",
    }
    tables = {
        row[0]
        for row in db.execute("SELECT name FROM sqlite_master WHERE type='table'")
    }
    missing = sorted(required - tables)
    if missing:
        errors.append(f"native evidence tables missing: {missing}")
    retired = sorted({"tasks", "steps"} & tables)
    if retired:
        errors.append(f"retired authority tables remain: {retired}")
    if db.execute("PRAGMA integrity_check").fetchone()[0] != "ok":
        errors.append("SQLite integrity check failed")
    if list(db.execute("PRAGMA foreign_key_check")):
        errors.append("SQLite foreign-key check failed")
    return not missing


def lifecycle_check(
    db: sqlite3.Connection, expected_path: Path, errors: list[str]
) -> None:
    expected = {
        row["scenario_key"]: row["expected_terminal"].lower()
        for row in tsv_rows(expected_path)
    }
    actual = {
        row[0]: row[1]
        for row in db.execute("SELECT scenario_key, lower(lifecycle) FROM matters")
    }
    if not actual:
        errors.append("no matters")
        return
    if expected != actual:
        errors.append(f"matter terminal mismatch expected={expected} actual={actual}")
    for scenario_key, lifecycle in actual.items():
        if lifecycle == "completed" and count(
            db,
            "SELECT COUNT(*) FROM obligations o JOIN matters m ON m.id=o.matter_id "
            "WHERE m.scenario_key=? AND o.required!=0 AND lower(o.state)!='satisfied'",
            (scenario_key,),
        ):
            errors.append(f"completed matter {scenario_key} has unsatisfied obligation")
    if count(db, "SELECT COUNT(*) FROM obligations") == 0:
        errors.append("no obligations")
    if count(
        db,
        "SELECT COUNT(*) FROM operations "
        "WHERE current!=0 AND lower(state) IN ('pending','active','failed','blocked')",
    ):
        errors.append("current unfinished or blocked operation")


def timing_and_events(
    db: sqlite3.Connection,
    minimum_span_ms: int,
    trace_events: dict[str, tuple[str, str, str, int]],
    errors: list[str],
) -> None:
    first, last, total = db.execute(
        "SELECT MIN(selected_monotonic_ms), MAX(selected_monotonic_ms), COUNT(*) "
        "FROM runtime_decisions"
    ).fetchone()
    if not total or last - first < minimum_span_ms:
        errors.append("real decisions do not span the required session period")
    if total < MINIMUM_DECISIONS:
        errors.append("runtime decision count below anchored floor")
    if count(db, "SELECT COUNT(*) FROM runtime_decisions WHERE useful!=0") < MINIMUM_USEFUL_DECISIONS:
        errors.append("useful decision count below anchored floor")
    if count(db, "SELECT COUNT(*) FROM runtime_decisions WHERE progressed!=0") < MINIMUM_PROGRESS_DECISIONS:
        errors.append("progress decision count below anchored floor")
    actual = {
        row[0]: (row[1], row[2] or "", row[3] or "", int(row[4]))
        for row in db.execute(
            "SELECT id,kind,matter_id,source_ref,monotonic_ms FROM runtime_events"
        )
    }
    if trace_events != actual:
        errors.append("raw event trace and SQLite tuples disagree")


def workspace_check(db: sqlite3.Connection, root: Path, errors: list[str]) -> None:
    root = root.resolve()
    documents = list(
        db.execute(
            "SELECT d.path, r.fingerprint FROM workspace_documents d "
            "JOIN workspace_revisions r ON r.id=d.current_revision_id "
            "WHERE d.state!='tombstoned'"
        )
    )
    if not documents:
        errors.append("no workspace documents")
    for relative, fingerprint in documents:
        path = Path(relative)
        if path.is_absolute() or ".." in path.parts:
            errors.append(f"unsafe workspace path: {relative}")
            continue
        candidate = (root / path).resolve()
        if root not in candidate.parents or not candidate.is_file():
            errors.append(f"workspace row has no contained file: {relative}")
            continue
        actual = "sha256:" + hashlib.sha256(candidate.read_bytes()).hexdigest()
        if fingerprint != actual:
            errors.append(f"workspace fingerprint mismatch: {relative}")


def check_database(
    database: Path,
    workspace: Path,
    expected_matters: Path,
    required_checks: set[str],
    minimum_span_ms: int,
    trace_events: dict[str, tuple[str, str, str, int]],
    run_id: str,
    errors: list[str],
) -> None:
    db = sqlite3.connect(f"file:{database}?mode=ro", uri=True)
    if not schema_check(db, errors):
        return
    runs = [row[0] for row in db.execute("SELECT id FROM runs")]
    if not run_id or runs != [run_id]:
        errors.append("database is not an isolated snapshot of the declared run")
    lifecycle_check(db, expected_matters, errors)
    linkage_check(db, required_checks, errors)
    timing_and_events(db, minimum_span_ms, trace_events, errors)
    workspace_check(db, workspace, errors)
