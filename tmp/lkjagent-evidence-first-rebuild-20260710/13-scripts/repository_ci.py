from __future__ import annotations

import json
import subprocess
from pathlib import Path


def command(repo: Path, *args: str) -> str:
    result = subprocess.run(
        args, cwd=repo, text=True, capture_output=True, check=False, timeout=300
    )
    if result.returncode:
        raise RuntimeError(f"{' '.join(args)}\n{result.stdout}\n{result.stderr}")
    return result.stdout.strip()


def public_ci(repo: Path, head: str) -> str:
    name = command(
        repo, "gh", "repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"
    )
    if name != "lkjsxc/lkjagent":
        raise RuntimeError(f"unexpected GitHub repository: {name}")
    raw = command(
        repo, "gh", "run", "list", "--repo", name, "--commit", head,
        "--workflow", "verify.yml", "--json", "conclusion,headSha,url", "--limit", "20",
    )
    runs = json.loads(raw)
    success = [
        run for run in runs if run.get("headSha") == head and run.get("conclusion") == "success"
    ]
    if not success:
        raise RuntimeError("public verify workflow has no success for evidence commit")
    return str(success[0]["url"])
