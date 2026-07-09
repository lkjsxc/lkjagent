from __future__ import annotations

import sqlite3
import hashlib
from pathlib import Path


def check_database_binding(
    path: Path, result: dict[str, str], errors: list[str]
) -> None:
    actual = "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()
    if result.get("status") != "completed":
        errors.append("PTY run status is not completed")
    if result.get("database_fingerprint") != actual:
        errors.append("PTY database fingerprint mismatch")
    db = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    runs = [row[0] for row in db.execute("SELECT id FROM runs")]
    if runs != [result.get("run_id")]:
        errors.append("PTY database is not isolated to declared run")


def database_messages(path: Path) -> dict[str, tuple[int, str]]:
    db = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    if db.execute("PRAGMA integrity_check").fetchone()[0] != "ok":
        raise ValueError("SQLite integrity check failed")
    if list(db.execute("PRAGMA foreign_key_check")):
        raise ValueError("SQLite foreign-key check failed")
    rows = list(
        db.execute("SELECT logical_id, sequence, role, body FROM conversation_messages")
    )
    if len(rows) < 40:
        raise ValueError("PTY conversation has fewer than forty messages")
    ids = [row[0] for row in rows]
    sequences = [row[1] for row in rows]
    if len(ids) != len(set(ids)) or len(sequences) != len(set(sequences)):
        raise ValueError("conversation IDs or sequences are duplicated")
    bodies = "\n".join(str(row[3]) for row in rows).lower()
    if "state: queue:" in bodies or any(line.startswith("/") for line in bodies.splitlines()):
        raise ValueError("diagnostic or slash command leaked into conversation")
    return {row[0]: (int(row[1]), row[2]) for row in rows}
