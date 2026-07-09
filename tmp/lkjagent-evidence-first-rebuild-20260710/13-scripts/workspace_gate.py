#!/usr/bin/env python3
from __future__ import annotations

import csv
import hashlib
import sqlite3
import stat
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from workspace_file_contract import KINDS, check_record, conservative_tokens


def digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def manifest_rows(path: Path, errors: list[str]) -> dict[str, dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle, delimiter="\t"))
    required = {"path", "document_id", "revision_id", "fingerprint"}
    if not rows or not required.issubset(rows[0]):
        errors.append("workspace manifest columns missing")
        return {}
    paths = [row["path"] for row in rows]
    if len(paths) != len(set(paths)):
        errors.append("workspace manifest has duplicate paths")
    return {row["path"]: row for row in rows}


def plain_files(root: Path, errors: list[str]) -> set[str]:
    result: set[str] = set()
    for path in root.rglob("*"):
        mode = path.lstat().st_mode
        relative = str(path.relative_to(root))
        if path.is_symlink() or not (stat.S_ISREG(mode) or stat.S_ISDIR(mode)):
            errors.append(f"workspace contains symlink or special node: {relative}")
        elif stat.S_ISREG(mode):
            result.add(relative)
    return result


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: workspace_gate.py SQLITE WORKSPACE MANIFEST.tsv", file=sys.stderr)
        return 2
    database, supplied_root, manifest_path = map(Path, sys.argv[1:])
    errors: list[str] = []
    if (
        not database.is_file()
        or not supplied_root.is_dir()
        or supplied_root.is_symlink()
        or not manifest_path.is_file()
    ):
        print("FAIL\tworkspace gate input missing or symlinked")
        return 1
    root = supplied_root.resolve()
    db = sqlite3.connect(f"file:{database}?mode=ro", uri=True)
    if db.execute("PRAGMA integrity_check").fetchone()[0] != "ok":
        errors.append("SQLite integrity check failed")
    if list(db.execute("PRAGMA foreign_key_check")):
        errors.append("SQLite foreign-key check failed")
    rows = list(
        db.execute(
            "SELECT d.document_id,d.path,d.kind,d.managed,d.current_revision_id,"
            "r.fingerprint,r.tokenizer_id,r.token_count,r.conservative_tokens "
            "FROM workspace_documents d JOIN workspace_revisions r "
            "ON r.id=d.current_revision_id WHERE d.state!='tombstoned'"
        )
    )
    identifiers = [row[0] for row in rows]
    paths = [row[1] for row in rows]
    revisions = [row[4] for row in rows]
    if any(len(values) != len(set(values)) for values in (identifiers, paths, revisions)):
        errors.append("workspace identity, path, or current revision is duplicated")
    manifest = manifest_rows(manifest_path, errors)
    disk = plain_files(root, errors)
    if disk != set(paths) or disk != set(manifest):
        errors.append("workspace disk, ledger, and manifest file sets disagree")
    for document_id, relative, kind, managed, revision, fingerprint, tokenizer, tokens, stored_safe in rows:
        path = Path(relative)
        if path.is_absolute() or ".." in path.parts or str(path) != relative:
            errors.append(f"non-normalized workspace path: {relative}")
            continue
        candidate = root / path
        if any((root.joinpath(*path.parts[:index])).is_symlink() for index in range(1, len(path.parts) + 1)):
            errors.append(f"symlinked workspace component: {relative}")
            continue
        if not candidate.is_file():
            errors.append(f"ledger document file missing: {relative}")
            continue
        body = candidate.read_bytes()
        actual = digest(body)
        if kind not in KINDS:
            errors.append(f"unknown workspace kind: {kind}")
        if actual != fingerprint:
            errors.append(f"revision fingerprint mismatch: {relative}")
        expected = manifest.get(relative, {})
        values = {"document_id": document_id, "revision_id": revision, "fingerprint": actual}
        if any(expected.get(key) != str(value) for key, value in values.items()):
            errors.append(f"manifest mismatch: {relative}")
        if managed:
            try:
                text = body.decode("utf-8")
                safe_count = conservative_tokens(text)
                provider_count = int(tokens)
                recorded_safe = int(stored_safe)
            except (UnicodeDecodeError, ValueError):
                errors.append(f"managed token data invalid: {relative}")
                continue
            if not tokenizer or safe_count != recorded_safe or max(safe_count, provider_count) > 512:
                errors.append(f"managed token contract failed: {relative}")
            check_record(relative, kind, document_id, body, errors)
    debt = db.execute(
        "SELECT COUNT(*) FROM index_debt WHERE current!=0 AND lower(state)!='settled'"
    ).fetchone()[0]
    if debt:
        errors.append(f"unsettled navigation debt: {debt}")
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print(f"PASS\tdocuments={len(rows)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
