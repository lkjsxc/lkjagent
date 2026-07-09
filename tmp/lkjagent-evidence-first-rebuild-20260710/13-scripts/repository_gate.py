#!/usr/bin/env python3
from __future__ import annotations

import re
import stat
import subprocess
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from repository_ci import public_ci
from repository_history import derive


EXPECTED_PACKET = Path("tmp/lkjagent-evidence-first-rebuild-20260710")


def command(repo: Path, *args: str) -> str:
    result = subprocess.run(
        args, cwd=repo, text=True, capture_output=True, check=False, timeout=7200
    )
    if result.returncode:
        raise RuntimeError(f"{' '.join(args)}\n{result.stdout}\n{result.stderr}")
    return result.stdout.strip()


def require_plain_tree(repo: Path, root: Path) -> None:
    for path in (root, *root.rglob("*")):
        mode = path.lstat().st_mode
        if path.is_symlink() or not (stat.S_ISREG(mode) or stat.S_ISDIR(mode)):
            raise RuntimeError(f"non-plain protected path: {path.relative_to(repo)}")
    staged = command(repo, "git", "ls-files", "--stage", "--", str(root.relative_to(repo)))
    if any(line.startswith("120000 ") for line in staged.splitlines()):
        raise RuntimeError("Git symlink forbidden in protected tree")


def require_tracked(repo: Path, root: Path) -> None:
    relative = root.relative_to(repo)
    actual = {
        str(path.relative_to(repo)) for path in root.rglob("*") if path.is_file()
    }
    tracked = set(command(repo, "git", "ls-files", "--", str(relative)).splitlines())
    missing = sorted(actual - tracked)
    if missing:
        raise RuntimeError(f"untracked evidence: {missing[:10]}")


def source_hygiene(repo: Path) -> None:
    errors: list[str] = []
    if not (repo / "docs" / "README.md").is_file() or not (repo / "crates").is_dir():
        errors.append("docs/README.md or crates source root is missing")
    for directory in (repo / "docs", *(repo / "docs").rglob("*")):
        if directory.is_dir() and not (directory / "README.md").is_file():
            errors.append(f"docs directory lacks README: {directory.relative_to(repo)}")
        if directory.is_dir():
            useful = [item for item in directory.iterdir() if item.name != "README.md"]
            if len(useful) < 2:
                errors.append(f"docs directory has fewer than two children: {directory.relative_to(repo)}")
    for root in (repo / "docs", repo / "crates"):
        for path in root.rglob("*"):
            if path.suffix not in {".md", ".rs"}:
                continue
            text = path.read_text(encoding="utf-8")
            if len(text.splitlines()) > 200:
                errors.append(f"over 200 lines: {path.relative_to(repo)}")
            if path.suffix == ".md" and re.search(r"(?i)\bversion\b|\bv[0-9]+\b", text):
                errors.append(f"release-style naming: {path.relative_to(repo)}")
    banned = ("TaskSnapshot", "LegacyArtifact", "finish_summary", "plan_bridge")
    for path in (repo / "crates").glob("*/src/**/*.rs"):
        text = path.read_text(encoding="utf-8")
        for token in banned:
            if token in text:
                errors.append(f"old authority {token}: {path.relative_to(repo)}")
    tracked_rust = command(repo, "git", "ls-files", "*.rs").splitlines()
    for relative in tracked_rust:
        if len((repo / relative).read_text(encoding="utf-8").splitlines()) > 200:
            errors.append(f"over 200 lines: {relative}")
    for name in ("README.md", "AGENTS.md"):
        path = repo / name
        if path.is_file():
            text = path.read_text(encoding="utf-8")
            if len(text.splitlines()) > 200:
                errors.append(f"over 200 lines: {name}")
            if re.search(r"(?i)\bversion\b|\bv[0-9]+\b", text):
                errors.append(f"release-style naming: {name}")
    if errors:
        raise RuntimeError("\n".join(errors))


def workflow_contract(repo: Path, source: str) -> None:
    workflow = command(repo, "git", "show", f"{source}:.github/workflows/verify.yml")
    active = "\n".join(line for line in workflow.splitlines() if not line.lstrip().startswith("#"))
    clean = "tmp/lkjagent-evidence-first-rebuild-20260710/13-scripts/clean_checkout_gate.sh"
    if "actions/checkout@" not in active or not re.search(rf"run:\s+(?:sh\s+)?{re.escape(clean)}\s+\.", active):
        raise RuntimeError("public workflow does not directly execute anchored clean-checkout gate")


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: repository_gate.py REPO PACKET_PATH EVIDENCE", file=sys.stderr)
        return 2
    repo = Path(sys.argv[1]).resolve()
    packet = Path(sys.argv[2])
    evidence = Path(sys.argv[3]).resolve()
    try:
        if packet != EXPECTED_PACKET:
            raise RuntimeError("packet path is not the fixed anchored path")
        if command(repo, "git", "status", "--porcelain"):
            raise RuntimeError("worktree is not clean")
        if repo == evidence or repo not in evidence.parents:
            raise RuntimeError("evidence must be a strict repository descendant")
        anchor, source, material, head = derive(repo, packet, evidence)
        expected = repo / "tmp" / "lkjagent-acceptance" / source
        if evidence != expected:
            raise RuntimeError("evidence root must be keyed by frozen source commit")
        command(repo, "git", "diff", "--exit-code", anchor, head, "--", str(packet))
        command(repo, "git", "ls-files", "--error-unmatch", "Cargo.lock")
        require_plain_tree(repo, repo / packet)
        require_plain_tree(repo, evidence)
        progress = repo / "tmp" / "lkjagent-progress"
        require_plain_tree(repo, progress)
        require_tracked(repo, evidence)
        require_tracked(repo, progress)
        source_hygiene(repo)
        workflow_contract(repo, source)
        scripts = Path(__file__).resolve().parent
        command(repo, sys.executable, str(scripts / "packet_lint.py"), str(repo / packet))
        command(repo, str(scripts / "clean_checkout_gate.sh"), str(repo))
        ci_url = public_ci(repo, head)
    except (RuntimeError, ValueError, OSError) as error:
        print(f"FAIL\t{error}")
        return 1
    print(f"anchor_commit\t{anchor}")
    print(f"source_commit\t{source}")
    print(f"evidence_material_commit\t{material}")
    print(f"verification_commit\t{head}")
    print(f"public_ci_url\t{ci_url}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
