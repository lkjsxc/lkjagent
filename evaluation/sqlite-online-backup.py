#!/usr/bin/env python3
from __future__ import annotations

import sqlite3
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: sqlite-online-backup.py SOURCE DESTINATION", file=sys.stderr)
        return 2
    source_path = Path(sys.argv[1]).resolve()
    destination_path = Path(sys.argv[2]).resolve()
    destination_path.unlink(missing_ok=True)
    progress_rows: list[tuple[int, int, int]] = []

    def progress(status: int, remaining: int, total: int) -> None:
        progress_rows.append((status, remaining, total))

    source = sqlite3.connect(f"file:{source_path}?mode=ro", uri=True)
    destination = sqlite3.connect(destination_path)
    try:
        source.backup(destination, pages=64, progress=progress, sleep=0.0)
        integrity = destination.execute("PRAGMA integrity_check").fetchone()
        if integrity != ("ok",):
            print(f"backup integrity failed: {integrity}", file=sys.stderr)
            return 1
        destination.commit()
    finally:
        destination.close()
        source.close()
    if not progress_rows or progress_rows[-1][1] != 0:
        print("online backup did not report completion", file=sys.stderr)
        return 1
    print("snapshot_method\tsqlite-online-backup")
    print(f"backup_steps\t{len(progress_rows)}")
    print(f"page_total\t{progress_rows[-1][2]}")
    print("integrity\tok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
