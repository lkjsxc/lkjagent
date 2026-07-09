#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, text=True, capture_output=True, check=False, timeout=7200)


def main() -> int:
    if len(sys.argv) != 4 or sys.argv[3] not in {"next", "final"}:
        print("usage: controller.py REPO EVIDENCE next|final", file=sys.stderr)
        return 2
    repo = Path(sys.argv[1]).resolve()
    evidence = Path(sys.argv[2]).resolve()
    scripts = Path(__file__).resolve().parent
    packet = scripts.parent
    expected = repo / "tmp" / "lkjagent-evidence-first-rebuild-20260710"
    if packet != expected:
        print("CONTROL_INVALID\tpacket is not at fixed anchored path")
        return 20
    lint = run([sys.executable, str(scripts / "packet_lint.py"), str(packet)])
    if lint.returncode:
        print(lint.stdout or lint.stderr)
        return 20
    if sys.argv[3] == "final":
        final = run(
            [
                sys.executable,
                str(scripts / "acceptance_gate.py"),
                str(repo),
                str(evidence),
                "tmp/lkjagent-evidence-first-rebuild-20260710",
            ]
        )
        print(final.stdout or final.stderr)
        return final.returncode
    progress = run(
        [
            sys.executable,
            str(scripts / "workgraph_gate.py"),
            str(packet),
            str(evidence),
        ]
    )
    print(progress.stdout or progress.stderr)
    if progress.returncode:
        print("CONTROL_INVALID\trepair receipt or dependency evidence")
        return 20
    print("WORK_REMAINS\texecute every released node, then invoke controller again")
    return 10


if __name__ == "__main__":
    raise SystemExit(main())
