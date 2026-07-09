from __future__ import annotations

import subprocess
from pathlib import Path


VERIFIER_FILES = {
    "independent-verifier.tsv",
    "verifier-artifacts.tsv",
    "verifier-commands.log",
    "verifier-report.md",
}


def command(repo: Path, *args: str) -> str:
    result = subprocess.run(args, cwd=repo, text=True, capture_output=True, check=False)
    if result.returncode:
        raise RuntimeError(f"{' '.join(args)}\n{result.stdout}\n{result.stderr}")
    return result.stdout.strip()


def inside(path: Path, root: Path) -> bool:
    return path == root or root in path.parents


def changed_paths(repo: Path, commit: str) -> list[Path]:
    output = command(
        repo, "git", "diff-tree", "--root", "--no-commit-id", "--name-only", "-r", commit
    )
    return [Path(item) for item in output.splitlines() if item]


def is_verifier(path: Path, evidence: Path) -> bool:
    return inside(path, evidence) and path.relative_to(evidence).as_posix() in VERIFIER_FILES


def derive(
    repo: Path, packet: Path, evidence: Path
) -> tuple[str, str, str, str]:
    additions = command(
        repo, "git", "log", "--diff-filter=A", "--format=%H", "--", str(packet / "README.md")
    ).splitlines()
    if len(additions) != 1:
        raise RuntimeError("packet must have exactly one introduction commit")
    anchor = additions[0]
    head = command(repo, "git", "rev-parse", "HEAD")
    evidence_rel = evidence.relative_to(repo)
    progress_rel = Path("tmp/lkjagent-progress")
    if any(not inside(path, packet) for path in changed_paths(repo, anchor)):
        raise RuntimeError("packet introduction commit contains unrelated changes")
    commits = command(repo, "git", "rev-list", "--reverse", f"{anchor}..{head}").splitlines()
    source_candidates = [
        commit
        for commit in commits
        if any(
            not inside(path, packet)
            and not inside(path, evidence_rel)
            and not inside(path, progress_rel)
            for path in changed_paths(repo, commit)
        )
    ]
    if not source_candidates:
        raise RuntimeError("no implementation commit exists after packet anchor")
    source = source_candidates[-1]
    post_source = commits[commits.index(source) + 1 :]
    for commit in post_source:
        illegal = [
            path
            for path in changed_paths(repo, commit)
            if not inside(path, evidence_rel) and not inside(path, progress_rel)
        ]
        if illegal:
            raise RuntimeError(f"post-freeze commit changes non-proof paths: {illegal[:5]}")
    material_candidates = [
        commit
        for commit in post_source
        if any(not is_verifier(path, evidence_rel) for path in changed_paths(repo, commit))
    ]
    if not material_candidates:
        raise RuntimeError("no raw evidence commit exists after source freeze")
    material = material_candidates[-1]
    material_index = post_source.index(material)
    if any(
        is_verifier(path, evidence_rel)
        for commit in post_source[: material_index + 1]
        for path in changed_paths(repo, commit)
    ):
        raise RuntimeError("verifier receipt was committed before raw evidence froze")
    for commit in post_source[material_index + 1 :]:
        if any(not is_verifier(path, evidence_rel) for path in changed_paths(repo, commit)):
            raise RuntimeError("verification commit contains non-verifier evidence")
    if len({anchor, source, material, head}) != 4:
        raise RuntimeError("anchor, source, material, and verification commits must be distinct")
    existing = command(
        repo, "git", "ls-tree", "-r", "--name-only", source, "--", str(evidence_rel)
    )
    if existing:
        raise RuntimeError("final evidence tree already exists in source commit")
    return anchor, source, material, head
