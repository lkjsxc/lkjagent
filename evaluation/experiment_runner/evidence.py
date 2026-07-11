from __future__ import annotations

import json
import sqlite3
from contextlib import closing
from pathlib import Path

from .io import file_sha, sha, write_table


def workspace_manifest(db: Path, workspace: Path, destination: Path) -> None:
    with closing(sqlite3.connect(db)) as conn:
        managed = {row[2]: (row[0], row[1]) for row in conn.execute("SELECT id, fingerprint, path FROM workspace_records")}
    rows = []; pending = [workspace]
    while pending:
        directory = pending.pop()
        for path in sorted(directory.iterdir()):
            if path.is_symlink():
                raise RuntimeError(f"symlink in workspace evidence: {path}")
            if path.is_dir():
                pending.append(path); continue
            relative = path.relative_to(workspace).as_posix(); fingerprint = file_sha(path)
            document, revision = managed.get(relative, (f"external:{fingerprint}", fingerprint))
            rows.append({"path": relative, "document_id": document, "revision_id": revision, "sha256": fingerprint})
    rows.sort(key=lambda row: row["path"])
    write_table(destination, ["path", "document_id", "revision_id", "sha256"], rows)


def export_rows(db: Path, run: Path) -> None:
    specs = {
        "events.tsv": ("event_id\tkind\tsource\tdecision_id\tcreated_at\n", "SELECT id,kind,source,COALESCE(decision_id,''),created_at FROM runtime_events ORDER BY created_at,id"),
        "decisions.tsv": ("decision_id\toperation\tstatus\tselected_at\tsettled_at\n", "SELECT id,operation_key,status,selected_at,COALESCE(settled_at,'') FROM runtime_decisions ORDER BY selected_at,id"),
        "exchanges.tsv": ("exchange_id\tdecision_id\texchange_ref\toutcome\tstarted_at\tfinished_at\n", "SELECT id,decision_id,exchange_ref,outcome_json,started_at,COALESCE(finished_at,'') FROM provider_exchanges ORDER BY started_at,id"),
        "admissions.tsv": ("admission_id\tdecision_id\ttool\tstatus\tcreated_at\n", "SELECT id,decision_id,action_tool,status,created_at FROM tool_admissions ORDER BY created_at,id"),
        "observations.tsv": ("observation_id\tdecision_id\teffect\tstatus\tcreated_at\n", "SELECT id,decision_id,effect_name,status,created_at FROM observations ORDER BY created_at,id"),
    }
    with closing(sqlite3.connect(db)) as conn:
        for name, (header, query) in specs.items():
            rows = conn.execute(query).fetchall()
            (run / name).write_text(header + "".join("\t".join(str(value).replace("\t", " ").replace("\n", " ") for value in row) + "\n" for row in rows), encoding="utf-8")
        target = run / "tables"; target.mkdir()
        names = [row[0] for row in conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")]
        for name in names:
            columns = [row[1] for row in conn.execute(f'PRAGMA table_info("{name}")')]
            lines = [json.dumps({"_columns": columns}, sort_keys=True, separators=(",", ":"))]
            for values in conn.execute(f'SELECT * FROM "{name}"'):
                row = {key: (f"hex:{value.hex()}" if isinstance(value, bytes) else value) for key, value in zip(columns, values, strict=True)}
                lines.append(json.dumps(row, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
            (target / f"{name}.jsonl").write_text("\n".join(lines) + "\n", encoding="utf-8")


def redact_value(value: object, sensitive: bool = False) -> object:
    keys = {"content", "system", "user", "error", "diagnosis", "detail", "message", "body", "preview", "anomaly", "raw"}
    if isinstance(value, dict):
        return {key: redact_value(item, sensitive or key.lower() in keys) for key, item in value.items()}
    if isinstance(value, list):
        return [redact_value(item, sensitive) for item in value]
    if sensitive and isinstance(value, str):
        return f"[redacted {sha(value.encode())}]"
    return value


def redact_runner(path: Path) -> None:
    output = []
    for line in path.read_text().splitlines():
        output.append(line if line.startswith("$ ") or line.startswith("exit=") or not line else f"[redacted {sha(line.encode())}]")
    path.with_name("runner-redacted.log").write_text("\n".join(output) + "\n")


def redact_logs(data: Path, run: Path) -> None:
    target = run / "logs-redacted"; target.mkdir(exist_ok=True)
    source = data / "logs"
    if not source.exists():
        return
    for path in sorted(item for item in source.rglob("*.json") if item.is_file()):
        value = redact_value(json.loads(path.read_text()))
        out = target / path.relative_to(source); out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(value, sort_keys=True, separators=(",", ":")), encoding="utf-8")


def manifest(root: Path, name: str) -> None:
    rows = []
    for path in sorted(item for item in root.rglob("*") if item.is_file() and item != root / name):
        if path.is_symlink():
            raise RuntimeError(f"symlink in evidence: {path}")
        rows.append({"path": str(path.relative_to(root)), "sha256": file_sha(path)})
    write_table(root / name, ["path", "sha256"], rows)


def raw_manifest(run: Path) -> None:
    manifest(run, "raw-manifest.tsv")
