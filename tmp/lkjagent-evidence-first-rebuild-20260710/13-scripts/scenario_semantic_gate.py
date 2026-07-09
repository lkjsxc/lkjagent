#!/usr/bin/env python3
from __future__ import annotations

import re
import hashlib
import sqlite3
import sys
from pathlib import Path


def normalized(text: str) -> str:
    return "".join(text.lower().split())


def documents(db: sqlite3.Connection) -> list[tuple[str, str, str]]:
    return list(
        db.execute(
            "SELECT document_id,path,kind FROM workspace_documents "
            "WHERE state NOT IN ('tombstoned','invalid')"
        )
    )


def diary(
    db: sqlite3.Connection,
    root: Path,
    docs: list[tuple[str, str, str]],
    errors: list[str],
) -> None:
    kinds = {row[2] for row in docs}
    required = {"journal", "todo", "calendar", "finance", "note"}
    if not required.issubset(kinds):
        errors.append(f"daily scenario record kinds missing: {sorted(required - kinds)}")
    journals = [root / path for _, path, kind in docs if kind == "journal"]
    if len(journals) != 1 or not journals[0].is_file():
        errors.append("daily scenario does not have exactly one current journal")
        return
    text = journals[0].read_text(encoding="utf-8")
    owner = [
        str(row[0])
        for row in db.execute(
            "SELECT body FROM conversation_messages WHERE role='owner' ORDER BY sequence"
        )
    ]
    if owner and len(normalized(owner[0])) >= 6 and normalized(owner[0]) in normalized(text):
        errors.append("journal contains the triggering owner command verbatim")
    if len(re.sub(r"[#<>/_`*\s-]", "", text)) < 80:
        errors.append("journal lacks substantive composed content")


def projects(
    db: sqlite3.Connection,
    root: Path,
    docs: list[tuple[str, str, str]],
    errors: list[str],
) -> None:
    projects = {
        Path(path).parts[1]
        for _, path, _ in docs
        if len(Path(path).parts) > 2 and Path(path).parts[0] == "projects"
    }
    if len(projects) < 2:
        errors.append("multi-project scenario exposes fewer than two projects")
    source_files = [path for path in root.rglob("*.rs") if path.is_file()]
    if not source_files:
        errors.append("multi-project workspace has no Rust source bytes")
    passed = {
        row[0] for row in db.execute("SELECT id FROM checks WHERE current!=0 AND passed!=0")
    }
    required = {"context-project-isolation", "source-edit-verified", "project-separation"}
    if not required.issubset(passed):
        errors.append("multi-project independent check set incomplete")
    if not list(db.execute("SELECT 1 FROM runtime_events WHERE kind='daemon.restart' LIMIT 1")):
        errors.append("multi-project daemon restart event missing")
    verification = root.parent / "logs-redacted" / "verification.log"
    if not verification.is_file() or verification.stat().st_size < 200:
        errors.append("multi-project verification log missing or empty")
    else:
        fingerprint = "sha256:" + hashlib.sha256(verification.read_bytes()).hexdigest()
        row = db.execute(
            "SELECT evidence_fingerprint FROM checks "
            "WHERE id='source-edit-verified' AND current!=0 AND passed!=0"
        ).fetchone()
        if not row or row[0] != fingerprint:
            errors.append("source-edit check does not bind verification log bytes")


def long_artifact(
    db: sqlite3.Connection,
    root: Path,
    docs: list[tuple[str, str, str]],
    errors: list[str],
) -> None:
    files = [
        root / path
        for _, path, kind in docs
        if kind == "artifact-section" and Path(path).parts[:1] == ("artifacts",)
    ]
    if len(files) < 3 or any(not path.is_file() for path in files):
        errors.append("long artifact has fewer than three durable semantic units")
        return
    words = sum(len(re.findall(r"\b\w+\b", path.read_text(encoding="utf-8"))) for path in files)
    if words < 1500:
        errors.append(f"long artifact is below 1500 words: {words}")
    kinds = {row[0] for row in db.execute("SELECT kind FROM runtime_events")}
    required = {"provider.output_limit", "recovery.strategy_changed", "artifact.unit.commit"}
    if not required.issubset(kinds):
        errors.append("long-artifact failure and recovery lineage incomplete")
    messages = "\n".join(
        str(row[0])
        for row in db.execute(
            "SELECT body FROM conversation_messages WHERE role='agent' AND lifecycle='final'"
        )
    )
    named = set(re.findall(r"artifacts/[A-Za-z0-9_./-]+", messages))
    if not named or any(not (root / path.rstrip(".,:;)")).is_file() for path in named):
        errors.append("final response does not name only existing artifact paths")


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: scenario_semantic_gate.py SQLITE WORKSPACE SCENARIO_ID", file=sys.stderr)
        return 2
    database = Path(sys.argv[1])
    root = Path(sys.argv[2]).resolve()
    scenario = sys.argv[3]
    errors: list[str] = []
    db = sqlite3.connect(f"file:{database}?mode=ro", uri=True)
    docs = documents(db)
    if scenario == "daily-life-recall":
        diary(db, root, docs, errors)
    elif scenario == "multi-project-development":
        projects(db, root, docs, errors)
    elif scenario == "long-artifact-recovery":
        long_artifact(db, root, docs, errors)
    else:
        errors.append(f"unknown anchored semantic scenario: {scenario}")
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print(f"PASS\tanchored_semantics={scenario}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
