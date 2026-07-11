from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import tempfile
import uuid
from pathlib import Path

from .evidence import manifest
from .io import SOURCE_PATHS, build_env, file_sha, pairs, source_sha, table, write_table
from .run import one_run

SCENARIOS = ("daily-life-recall", "multi-project-development", "long-artifact-recovery")


def saved_rows(campaign: Path) -> list[dict[str, str]]:
    rows = []
    for path in sorted((campaign / "runs").glob("*/matrix-row.tsv")):
        if (path.parent / "failure.tsv").exists():
            raise RuntimeError(f"failed run retained a matrix row: {path.parent}")
        found = table(path)
        if len(found) != 1:
            raise RuntimeError(f"invalid saved matrix row: {path}")
        rows.append(found[0])
    keys = [(row["cell_id"], row["scenario_id"], row["repeat"]) for row in rows]
    if len(keys) != len(set(keys)):
        raise RuntimeError("duplicate saved experiment tuple")
    return rows


def execute_jobs(root: Path, campaign: Path, binary: Path, source: str, base: dict[str, object],
                 jobs: list[tuple[dict[str, str], str, int]], rows: list[dict[str, str]], label: str) -> None:
    for job in jobs:
        row = one_run(root, campaign, binary, source, *job, base)
        rows.append(row); print(f"ok {label}{row['run_id']}", flush=True)


def run_jobs(root: Path, campaign: Path, binary: Path, source: str,
             cells: list[dict[str, str]], base: dict[str, object]) -> list[dict[str, str]]:
    rows = saved_rows(campaign)
    present = {(row["cell_id"], row["scenario_id"], int(row["repeat"])) for row in rows}
    initial = [(cell, scenario, repeat) for cell in cells for scenario in SCENARIOS for repeat in range(1, 4)
        if (cell["cell_id"], scenario, repeat) not in present]
    execute_jobs(root, campaign, binary, source, base, initial, rows, "")
    noisy = []
    for cell in cells:
        for scenario in SCENARIOS:
            first = [row for row in rows if row["cell_id"] == cell["cell_id"]
                and row["scenario_id"] == scenario and int(row["repeat"]) <= 3]
            if len(first) != 3:
                raise RuntimeError(f"incomplete first repeats: {cell['cell_id']} {scenario}")
            if len({row["outcome_fingerprint"] for row in first}) > 1:
                noisy.extend((cell, scenario, repeat) for repeat in (4, 5)
                    if (cell["cell_id"], scenario, repeat) not in present)
    execute_jobs(root, campaign, binary, source, base, noisy, rows, "escalation ")
    return rows


def read_pairs(path: Path) -> dict[str, str]:
    output = {}
    for line in path.read_text().splitlines():
        key, value = line.split("\t", 1)
        if not key or key in output:
            raise RuntimeError(f"invalid pair table: {path}")
        output[key] = value
    return output


def prepare_resume(campaign: Path, source: str, plan_commit: str, source_tree: str) -> None:
    if (campaign / "source-commit.txt").read_text().strip() != source or \
       (campaign / "plan-commit.txt").read_text().strip() != plan_commit or \
       (campaign / "source-tree-sha256.txt").read_text().strip() != source_tree:
        raise RuntimeError("resume source or plan changed")
    history = campaign / "history" / f"resume-{uuid.uuid4().hex}"; history.mkdir(parents=True)
    for name in ("failure.tsv", "campaign-manifest.tsv", "experiment-matrix.tsv", "adoption.tsv"):
        path = campaign / name
        if path.exists():
            path.replace(history / name)
    failed = [path for path in (campaign / "runs").glob("*")
        if path.is_dir() and ((path / "failure.tsv").is_file() or not (path / "matrix-row.tsv").is_file())]
    if failed:
        target = history / "runs"; target.mkdir()
        for path in failed:
            path.replace(target / path.name)
    if not (campaign / "lkjagent").is_file() or not (campaign / "build.tsv").is_file():
        for name in ("build.log", "build.tsv", "lkjagent"):
            path = campaign / name
            if path.exists():
                path.replace(history / name)


def build_binary(root: Path, campaign: Path, source: str, source_tree: str) -> int:
    binary = campaign / "lkjagent"
    if binary.is_file() and (campaign / "build.tsv").is_file():
        build_info = read_pairs(campaign / "build.tsv")
        expected = {"source_commit": source, "source_tree_sha256": source_tree,
            "cargo_lock_sha256": file_sha(root / "Cargo.lock"), "executable_sha256": file_sha(binary),
            "build_log_sha256": file_sha(campaign / "build.log"),
            "build_mode": "detached-offline-release-remapped", "build_exit": "0"}
        if build_info != expected:
            raise RuntimeError("resume build provenance changed")
        return 0
    build_root = Path(tempfile.mkdtemp(prefix=f"lkjagent-experiment-build-{source[:8]}-")); build_source = build_root / "source"
    added = subprocess.run(["git", "worktree", "add", "--detach", "--quiet", str(build_source), source], cwd=root)
    if added.returncode:
        shutil.rmtree(build_root, ignore_errors=True)
        raise RuntimeError("detached build worktree failed")
    target = build_root / "target"
    args = ["cargo", "build", "--locked", "--offline", "--release", "-p", "lkjagent-app", "--target-dir", str(target)]
    try:
        try:
            build = subprocess.run(args, cwd=build_source, env=build_env(build_root, build_source), text=True, capture_output=True, timeout=1800)
        except subprocess.TimeoutExpired as error:
            build = subprocess.CompletedProcess(args, 124, str(error.stdout or ""), str(error.stderr or "") + "\nbuild timeout")
        (campaign / "build.log").write_text("$ " + " ".join(args) + "\n" + build.stdout + build.stderr + f"\nexit={build.returncode}\n")
        if not build.returncode:
            shutil.copy2(target / "release/lkjagent", binary)
    finally:
        removed = subprocess.run(["git", "worktree", "remove", "--force", str(build_source)], cwd=root)
        shutil.rmtree(build_root, ignore_errors=True)
        if removed.returncode:
            raise RuntimeError("detached build worktree cleanup failed")
    if build.returncode:
        pairs(campaign / "failure.tsv", [("status", "failed"), ("phase", "build"), ("exit", build.returncode)])
        manifest(campaign, "campaign-manifest.tsv"); return build.returncode
    pairs(campaign / "build.tsv", [("source_commit", source), ("source_tree_sha256", source_tree),
        ("cargo_lock_sha256", file_sha(root / "Cargo.lock")), ("executable_sha256", file_sha(binary)),
        ("build_log_sha256", file_sha(campaign / "build.log")),
        ("build_mode", "detached-offline-release-remapped"), ("build_exit", 0)])
    return 0


def main() -> int:
    root = Path(sys.argv[1]).resolve()
    status = subprocess.check_output(["git", "status", "--porcelain", "--", *SOURCE_PATHS], cwd=root, text=True)
    if status:
        raise RuntimeError("candidate source inputs are dirty")
    campaign = (root / "tmp/lkjagent-progress/nodes/domain-experiments/campaign").resolve()
    nonempty = campaign.exists() and any(campaign.iterdir()); resume = os.environ.get("LKJAGENT_EXPERIMENT_RESUME") == "1"
    if nonempty and not resume:
        raise RuntimeError(f"campaign directory is not fresh: {campaign}; set LKJAGENT_EXPERIMENT_RESUME=1")
    campaign.mkdir(parents=True, exist_ok=True)
    source = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
    plan_commit = subprocess.check_output(["git", "log", "--format=%H", "--diff-filter=A", "--",
        "evaluation/experiment-plan.tsv"], cwd=root, text=True).splitlines()[0]
    if plan_commit == source or subprocess.run(["git", "merge-base", "--is-ancestor", plan_commit, source], cwd=root).returncode \
       or subprocess.run(["git", "diff", "--quiet", plan_commit, source, "--", "evaluation/experiment-plan.tsv"], cwd=root).returncode:
        raise RuntimeError("experiment plan is not an unchanged strict ancestor")
    source_tree = source_sha(root)
    if nonempty: prepare_resume(campaign, source, plan_commit, source_tree)
    else:
        (campaign / "source-commit.txt").write_text(source + "\n"); (campaign / "plan-commit.txt").write_text(plan_commit + "\n")
        (campaign / "source-tree-sha256.txt").write_text(source_tree + "\n")
    try: build_status = build_binary(root, campaign, source, source_tree)
    except BaseException as error:
        pairs(campaign / "failure.tsv", [("status", "failed"), ("phase", "build"),
            ("error_type", type(error).__name__), ("error", str(error))])
        manifest(campaign, "campaign-manifest.tsv"); raise
    if build_status: return 1
    cells = table(root / "evaluation/experiment-plan.tsv"); base = json.loads((root / "data/lkjagent.json").read_text())
    try: rows = run_jobs(root, campaign, campaign / "lkjagent", source, cells, base)
    except BaseException as error:
        pairs(campaign / "failure.tsv", [("status", "failed"), ("phase", "runs"),
            ("error_type", type(error).__name__), ("error", str(error))])
        manifest(campaign, "campaign-manifest.tsv"); raise
    rows.sort(key=lambda row: (row["cell_id"], row["scenario_id"], int(row["repeat"])))
    fields = ["cell_id", "scenario_id", "repeat", "run_id", "source_commit", "config_sha256",
        "scenario_sha256", "executable_sha256", "run_ref", "outcome", "outcome_fingerprint"]
    write_table(campaign / "experiment-matrix.tsv", fields, rows)
    adoption = []
    for cell in cells:
        outcomes = {row["outcome"] for row in rows if row["cell_id"] == cell["cell_id"]}
        config_rejected = "probe-config-rejected" in outcomes; no_exchange = "probe-no-exchange" in outcomes
        rejected = config_rejected or no_exchange or bool(outcomes & {"probe-parse-fault", "probe-admission-rejected"})
        rationale = "configuration-rejected" if config_rejected else "no-provider-exchange" if no_exchange else "probe-protocol-failure" if rejected \
            else "requires-fault-and-frozen-live-campaign"
        adoption.append({"cell_id": cell["cell_id"], "decision": "rejected" if rejected else "conditional", "rationale": rationale})
    write_table(campaign / "adoption.tsv", ["cell_id", "decision", "rationale"], adoption)
    manifest(campaign, "campaign-manifest.tsv")
    print(f"PASS experiment runs={len(rows)} source={source}"); return 0
