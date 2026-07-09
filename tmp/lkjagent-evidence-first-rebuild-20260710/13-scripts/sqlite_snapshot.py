#!/usr/bin/env python3
from __future__ import annotations

import sqlite3
import sys
from pathlib import Path


IMPORTANT = (
    "matters",
    "obligations",
    "operations",
    "runtime_events",
    "runtime_decisions",
    "effect_journal",
    "tool_admissions",
    "observations",
    "checks",
    "workspace_documents",
    "conversation_messages",
)


def columns(connection: sqlite3.Connection, table: str) -> set[str]:
    return {row[1] for row in connection.execute(f"PRAGMA table_info({table})")}


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: sqlite_snapshot.py PATH", file=sys.stderr)
        return 2
    path = Path(sys.argv[1])
    connection = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    integrity = connection.execute("PRAGMA integrity_check").fetchone()[0]
    print(f"integrity\t{integrity}")
    tables = {
        row[0]
        for row in connection.execute(
            "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
        )
    }
    for table in IMPORTANT:
        if table not in tables:
            print(f"table\t{table}\tmissing")
            continue
        count = connection.execute(f"SELECT COUNT(*) FROM {table}").fetchone()[0]
        print(f"table\t{table}\t{count}")
        fields = columns(connection, table)
        state = "lifecycle" if "lifecycle" in fields else "state" if "state" in fields else ""
        if state:
            for value, number in connection.execute(
                f"SELECT {state}, COUNT(*) FROM {table} GROUP BY {state} ORDER BY {state}"
            ):
                print(f"state\t{table}\t{value}\t{number}")
    return 0 if integrity == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
