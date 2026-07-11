from __future__ import annotations

import json
import os
import shutil
import sqlite3
import uuid
from pathlib import Path

from .evidence import export_rows, raw_manifest, redact_logs, redact_runner, workspace_manifest
from .io import command, file_sha, pairs, safe_env, sha, table, tree_sha, write_table


def owner_text(root: Path, scenario: str) -> str:
    rows = table(root / "evaluation/scenarios" / scenario / "owner-schedule.tsv")
    return min(rows, key=lambda row: int(row["offset_seconds"]))["owner_text"]


def counts(db: Path) -> dict[str, object]:
    with sqlite3.connect(db) as conn:
        row = conn.execute("""SELECT
          (SELECT COUNT(*) FROM runtime_decisions), (SELECT COUNT(*) FROM provider_exchanges),
          (SELECT COUNT(*) FROM provider_exchanges WHERE finished_at IS NOT NULL AND outcome_json NOT LIKE '%endpoint_error%'),
          (SELECT COUNT(*) FROM provider_exchanges WHERE outcome_json LIKE '%parse_fault%'),
          (SELECT COUNT(*) FROM tool_admissions), (SELECT COUNT(*) FROM tool_admissions WHERE status = 'Admitted'),
          (SELECT COUNT(*) FROM observations), (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked'),
          (SELECT COUNT(*) FROM runtime_events WHERE kind LIKE '%recovery%'),
          (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'recovery.no-progress')""").fetchone()
        keys = ("decisions", "exchanges", "real", "parse_faults", "admissions", "admitted", "observations", "blockers", "recovery_events", "no_progress_events")
        facts = dict(zip(keys, row, strict=True))
        first = conn.execute("""SELECT decision_id, outcome_json, exchange_ref FROM provider_exchanges
          WHERE finished_at IS NOT NULL AND outcome_json NOT LIKE '%endpoint_error%'
          ORDER BY started_at, id LIMIT 1""").fetchone()
        if first:
            actions = conn.execute("SELECT action_tool, status FROM tool_admissions WHERE decision_id = ? ORDER BY created_at, id", (first[0],)).fetchall()
            facts.update(first_parse=int("parse_fault" not in first[1]), first_admissions=len(actions),
                first_admitted=sum(status == "Admitted" for _, status in actions),
                first_actions=",".join(f"{tool}:{status}" for tool, status in actions), first_outcome=first[1], first_ref=first[2])
    return facts


def outcome(facts: dict[str, object]) -> str:
    if facts["exchanges"] == 0:
        return "probe-no-exchange"
    if not facts["first_parse"]:
        return "probe-parse-fault"
    if facts["first_admissions"] and not facts["first_admitted"]:
        return "probe-admission-rejected"
    if facts["first_admitted"]:
        return "probe-admitted"
    return "probe-message"


def reported(value: object) -> object:
    return value if isinstance(value, int) and not isinstance(value, bool) else "not-reported"


def outcome_fingerprint(facts: dict[str, object]) -> str:
    value = "\0".join((outcome(facts), str(facts.get("first_outcome", "none")), str(facts.get("first_actions", "")),
        str(facts["observations"]), str(facts["blockers"]), str(facts["recovery_events"]), str(facts["no_progress_events"])))
    return sha(value.encode())


def _one_run(root: Path, campaign: Path, binary: Path, source: str,
             cell: dict[str, str], scenario: str, repeat: int, base: dict[str, object], run_id: str) -> dict[str, str]:
    run = campaign / "runs" / run_id; data, workspace = run / "data", run / "workspace"
    data.mkdir(parents=True); shutil.copytree(root / "evaluation/scenarios" / scenario / "seed", workspace)
    config = dict(base); config.update(json.loads(cell["factor_config_json"])); config["workspace_root"] = str(workspace)
    config_bytes = json.dumps(config, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    (run / "config.json").write_text(config_bytes, encoding="utf-8"); (data / "lkjagent.json").write_text(config_bytes, encoding="utf-8")
    log: list[str] = []; log_path = run / "runner.log"; env = safe_env()
    if command([str(binary), "--data", str(data), "send", "--new", owner_text(root, scenario)], log, log_path, env):
        raise RuntimeError(f"send failed for {run_id}")
    facts: dict[str, object] = {}
    for _native_step in range(6):
        if command([str(binary), "--data", str(data), "run", "--once"], log, log_path, env):
            raise RuntimeError(f"run failed for {run_id}")
        facts = counts(data / "lkjagent.sqlite3")
        if facts["exchanges"] > 0:
            break
    if facts.get("exchanges") not in (0, 1) or facts.get("real") != facts.get("exchanges"):
        raise RuntimeError(f"ambiguous provider exchange for {run_id}")
    no_exchange = facts["exchanges"] == 0
    if no_exchange:
        (data / "logs").mkdir(exist_ok=True)
    backup = run / "run.sqlite3"
    with sqlite3.connect(data / "lkjagent.sqlite3") as source_db, sqlite3.connect(backup) as target:
        source_db.backup(target)
    with sqlite3.connect(backup) as conn:
        if conn.execute("PRAGMA integrity_check").fetchone()[0] != "ok":
            raise RuntimeError(f"backup integrity failed for {run_id}")
    export_rows(backup, run); workspace_manifest(backup, workspace, run / "workspace-manifest.tsv")
    redact_logs(data, run); redact_runner(log_path)
    endpoint = os.environ.get("LKJAGENT_ENDPOINT_URL", str(config.get("endpoint_url", "")))
    model = os.environ.get("LKJAGENT_MODEL", str(config.get("endpoint_model", "")))
    controls = {key: value for key, value in safe_env().items() if key != "LKJAGENT_API_KEY"}
    pairs(run / "provider-manifest.tsv", [("transport", "http-not-used" if no_exchange else "http"), ("endpoint_sha256", sha(endpoint.encode())),
        ("model_sha256", sha(model.encode())), ("environment_sha256", sha(json.dumps(controls, sort_keys=True).encode())),
        ("real_requests", facts["exchanges"]), ("credential_present", str(bool(os.environ.get("LKJAGENT_API_KEY"))).lower())])
    response = {} if no_exchange else json.loads((data / str(facts["first_ref"]) / "response.json").read_text())
    timing = {} if no_exchange else json.loads((data / str(facts["first_ref"]) / "timing.json").read_text())
    usage = response.get("usage", {})
    pairs(run / "metrics.tsv", [("provider_exchanges", facts["exchanges"]), ("endpoint_calls", facts["exchanges"]),
        ("first_pass_parse", "not-applicable" if no_exchange else facts["first_parse"]),
        ("first_pass_admission", "not-applicable" if no_exchange or facts["first_admissions"] == 0 else int(facts["first_admitted"] > 0)),
        ("action_identity", facts.get("first_actions") or "none"), ("prompt_tokens", reported(usage.get("prompt_tokens"))),
        ("completion_tokens", reported(usage.get("completion_tokens"))), ("cached_tokens", reported(usage.get("cached_tokens"))),
        ("duration_ms", reported(timing.get("duration_ms"))), ("observations", facts["observations"]),
        ("unexpected_blockers", facts["blockers"]), ("recovery_events", facts["recovery_events"]),
        ("no_progress_events", facts["no_progress_events"]), ("recovery_factor_exercised", int(facts["no_progress_events"] > 0)),
        ("fault_schedule_exercised", 0), ("required_source_recall", "not-measured"), ("unsupported_claims", "not-measured"),
        ("repeated_failure", "not-measured"), ("recovery_time_ms", "not-measured"), ("primary_task_success", "not-measured"),
        ("semantic_checks", "not-measured"), ("full_live_floor_measured", 0)])
    pairs(run / "result.tsv", [("status", "rejected" if no_exchange else "conditional"),
        ("reason", "no-provider-exchange" if no_exchange else "requires-fault-and-frozen-live-campaign"),
        ("snapshot_method", "sqlite-online-backup"), ("source_commit", source),
        ("run_id", run_id), ("runner_log_sha256", file_sha(log_path))])
    row = {"cell_id": cell["cell_id"], "scenario_id": scenario, "repeat": str(repeat), "run_id": run_id,
        "source_commit": source, "config_sha256": sha(config_bytes.encode()),
        "scenario_sha256": tree_sha(root / "evaluation/scenarios" / scenario), "executable_sha256": file_sha(binary),
        "run_ref": f"runs/{run_id}", "outcome": outcome(facts), "outcome_fingerprint": outcome_fingerprint(facts)}
    write_table(run / "matrix-row.tsv", list(row), [row]); raw_manifest(run)
    return row


def one_run(root: Path, campaign: Path, binary: Path, source: str,
            cell: dict[str, str], scenario: str, repeat: int, base: dict[str, object]) -> dict[str, str]:
    run_id = f"{cell['cell_id']}--{scenario}--r{repeat}--{uuid.uuid4().hex}"
    try:
        return _one_run(root, campaign, binary, source, cell, scenario, repeat, base, run_id)
    except BaseException as error:
        run = campaign / "runs" / run_id; run.mkdir(parents=True, exist_ok=True)
        if (run / "matrix-row.tsv").exists():
            (run / "matrix-row.tsv").replace(run / "failed-matrix-row.tsv")
        pairs(run / "failure.tsv", [("status", "failed"), ("error_type", type(error).__name__), ("error", str(error))])
        source_db = run / "data/lkjagent.sqlite3"
        if (run / "data/logs").is_dir():
            try: redact_logs(run / "data", run)
            except (OSError, ValueError) as redact_error: pairs(run / "redaction-failure.tsv", [("error", str(redact_error))])
        if (run / "runner.log").is_file():
            redact_runner(run / "runner.log")
        if source_db.is_file():
            try:
                with sqlite3.connect(source_db) as source_conn, sqlite3.connect(run / "failure.sqlite3") as target:
                    source_conn.backup(target)
            except sqlite3.Error as backup_error: pairs(run / "backup-failure.tsv", [("error", str(backup_error))])
        raw_manifest(run); raise
