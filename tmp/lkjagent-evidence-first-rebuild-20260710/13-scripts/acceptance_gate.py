#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from acceptance_runs import adopted_campaigns, final_campaigns, pty_campaigns
from experiment_gate import check_experiments
from verifier_receipt_gate import check_verifier


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, text=True, capture_output=True, check=False, timeout=7200)


def fields(text: str) -> dict[str, str]:
    return dict(line.split("\t", 1) for line in text.splitlines() if "\t" in line)


def file_fields(path: Path) -> dict[str, str]:
    return fields(path.read_text(encoding="utf-8"))


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: acceptance_gate.py REPO EVIDENCE PACKET_PATH", file=sys.stderr)
        return 2
    repo = Path(sys.argv[1]).resolve()
    evidence = Path(sys.argv[2]).resolve()
    scripts = Path(__file__).resolve().parent
    packet = scripts.parent
    supplied_packet = (repo / sys.argv[3]).resolve()
    expected_packet = repo / "tmp" / "lkjagent-evidence-first-rebuild-20260710"
    if supplied_packet != packet or packet != expected_packet:
        print("FAIL\tacceptance must run from the anchored fixed packet path")
        return 1
    errors: list[str] = []
    repository = run(
        [
            sys.executable,
            str(scripts / "repository_gate.py"),
            str(repo),
            str(packet.relative_to(repo)),
            str(evidence),
        ]
    )
    repository_values = fields(repository.stdout)
    source_commit = repository_values.get("source_commit", "unknown")
    material_commit = repository_values.get("evidence_material_commit", "unknown")
    if repository.returncode:
        errors.append(repository.stdout.strip() or repository.stderr.strip())
    progress = repo / "tmp" / "lkjagent-progress"
    workgraph = run(
        [sys.executable, str(scripts / "workgraph_gate.py"), str(packet), str(progress)]
    )
    if workgraph.returncode or "SOURCE_FREEZE_EVIDENCE_GRAPH_COMPLETE" not in workgraph.stdout:
        errors.append(workgraph.stdout.strip() or workgraph.stderr.strip())
    for receipt in (progress / "nodes").glob("*/result.tsv"):
        commit = file_fields(receipt).get("source_commit", "")
        ancestor = run(
            ["git", "-C", str(repo), "merge-base", "--is-ancestor", commit, source_commit]
        )
        if ancestor.returncode:
            errors.append(f"node receipt commit is not in frozen source: {receipt.parent.name}")
    focused = run(
        [sys.executable, str(scripts / "focused_gate.py"), str(repo), str(evidence)]
    )
    if focused.returncode:
        errors.append(focused.stdout.strip() or focused.stderr.strip())
    binary = fields(focused.stdout).get("binary_fingerprint", "")
    final_configurations = final_campaigns(
        repo, evidence, scripts, source_commit, binary, errors
    )
    pty_campaigns(evidence, scripts, source_commit, binary, errors)
    adopted = check_experiments(
        repo, evidence, final_configurations, source_commit, errors
    )
    adopted_campaigns(repo, scripts, adopted, source_commit, binary, errors)
    if repository.returncode == 0:
        check_verifier(evidence, packet, source_commit, material_commit, errors)
    dirty = run(["git", "-C", str(repo), "status", "--porcelain"])
    if dirty.returncode or dirty.stdout.strip():
        errors.append("final gates changed the clean repository")
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print(repository.stdout.strip())
    print(focused.stdout.strip())
    print("ALL_REQUIRED_ACCEPTANCE_GATES_PASSED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
