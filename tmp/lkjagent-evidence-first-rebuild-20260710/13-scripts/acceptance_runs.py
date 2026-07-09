from __future__ import annotations

import hashlib
import subprocess
import sys
from pathlib import Path


REQUIRED_SCENARIOS = {
    "daily-life-recall",
    "multi-project-development",
    "long-artifact-recovery",
}


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, text=True, capture_output=True, check=False, timeout=7200)


def values(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def append_failure(result: subprocess.CompletedProcess[str], errors: list[str]) -> None:
    if result.returncode:
        errors.append(result.stdout.strip() or result.stderr.strip())


def workspace_semantic(
    scripts: Path, directory: Path, scenario: str, errors: list[str]
) -> None:
    append_failure(
        run(
            [
                sys.executable,
                str(scripts / "workspace_gate.py"),
                str(directory / "run.sqlite3"),
                str(directory / "workspace"),
                str(directory / "workspace-manifest.tsv"),
            ]
        ),
        errors,
    )
    append_failure(
        run(
            [
                sys.executable,
                str(scripts / "scenario_semantic_gate.py"),
                str(directory / "run.sqlite3"),
                str(directory / "workspace"),
                scenario,
            ]
        ),
        errors,
    )


def live(
    repo: Path,
    scripts: Path,
    directory: Path,
    scenario: str,
    source_commit: str,
    errors: list[str],
) -> None:
    scenario_path = repo / "evaluation" / "scenarios" / scenario / "scenario.tsv"
    append_failure(
        run(
            [
                sys.executable,
                str(scripts / "live_evidence_gate.py"),
                str(directory),
                source_commit,
                str(scenario_path),
            ]
        ),
        errors,
    )
    workspace_semantic(scripts, directory, scenario, errors)


def final_campaigns(
    repo: Path,
    evidence: Path,
    scripts: Path,
    source_commit: str,
    binary: str,
    errors: list[str],
) -> set[str]:
    campaigns = sorted(path.parent for path in evidence.glob("campaign-*/run.sqlite3"))
    scenarios: set[str] = set()
    configurations: set[str] = set()
    database_hashes: set[str] = set()
    for campaign in campaigns:
        result_path = campaign / "result.tsv"
        if not result_path.is_file():
            errors.append(f"campaign result missing: {campaign.name}")
            continue
        result = values(result_path)
        scenario = result.get("scenario_id", "")
        scenarios.add(scenario)
        configurations.add(result.get("configuration_fingerprint", ""))
        if result.get("binary_fingerprint") != binary:
            errors.append(f"campaign binary differs from frozen build: {campaign.name}")
        fingerprint = hashlib.sha256((campaign / "run.sqlite3").read_bytes()).hexdigest()
        if fingerprint in database_hashes:
            errors.append("duplicate live campaign databases")
        database_hashes.add(fingerprint)
        live(repo, scripts, campaign, scenario, source_commit, errors)
    missing = REQUIRED_SCENARIOS - scenarios
    if missing:
        errors.append(f"required live scenarios missing: {sorted(missing)}")
    return configurations


def pty_campaigns(
    evidence: Path,
    scripts: Path,
    source_commit: str,
    binary: str,
    errors: list[str],
) -> None:
    directories = sorted(path.parent for path in evidence.glob("pty-*/tui-trace.tsv"))
    if not directories:
        errors.append("PTY campaign missing")
    for directory in directories:
        result_path = directory / "result.tsv"
        if not result_path.is_file():
            errors.append(f"PTY result missing: {directory.name}")
            continue
        if values(result_path).get("binary_fingerprint") != binary:
            errors.append(f"PTY binary differs from frozen build: {directory.name}")
        append_failure(
            run(
                [
                    sys.executable,
                    str(scripts / "tui_trace_gate.py"),
                    str(directory / "tui-trace.tsv"),
                    str(directory / "run.sqlite3"),
                    str(result_path),
                    source_commit,
                ]
            ),
            errors,
        )


def adopted_campaigns(
    repo: Path,
    scripts: Path,
    directories: list[Path],
    source_commit: str,
    binary: str,
    errors: list[str],
) -> None:
    for directory in directories:
        result = values(directory / "result.tsv")
        if result.get("binary_fingerprint") != binary:
            errors.append(f"adopted experiment binary differs: {directory.name}")
        live(repo, scripts, directory, result.get("scenario_id", ""), source_commit, errors)
