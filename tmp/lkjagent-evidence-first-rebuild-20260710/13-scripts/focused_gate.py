#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
import tempfile
from pathlib import Path


TESTS = (
    ("lkjagent-core", "configuration_contract"),
    ("lkjagent-core", "state_runtime_contract"),
    ("lkjagent-core", "context_protocol_contract"),
    ("lkjagent-store", "workspace_contract"),
    ("lkjagent-effects", "effect_recovery_contract"),
    ("lkjagent-app", "continuity_contract"),
    ("lkjagent-app", "tui_contract"),
    ("lkjagent-xtask", "evidence_contract"),
)


def registry_keys(packet: Path) -> set[str]:
    text = (packet / "02-product" / "configuration-registry.md").read_text(encoding="utf-8")
    return {
        match.group(1)
        for match in re.finditer(r"^\| ([a-z][a-z0-9_]*) \|", text, re.MULTILINE)
    }


def configuration(repo: Path, packet: Path, errors: list[str]) -> None:
    path = repo / "data" / "lkjagent.json"
    if not path.is_file():
        errors.append("tracked data/lkjagent.json missing")
        return
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        errors.append(f"configuration JSON invalid: {error}")
        return
    if not isinstance(data, dict):
        errors.append("configuration root is not an object")
        return
    if set(data) != registry_keys(packet):
        errors.append("configuration keys do not exactly match anchored registry")
    if any(type(value) not in {str, int, bool} for value in data.values()):
        errors.append("configuration contains non-scalar or ambiguous values")
    tracked = subprocess.run(
        ["git", "ls-files", "--error-unmatch", "data/lkjagent.json"],
        cwd=repo,
        capture_output=True,
        check=False,
    )
    if tracked.returncode:
        errors.append("data/lkjagent.json is not tracked")


def run_test(repo: Path, package: str, test: str, errors: list[str]) -> None:
    command = [
        "docker",
        "compose",
        "--profile",
        "shell",
        "run",
        "--build",
        "--rm",
        "shell",
        "cargo",
        "test",
        "--locked",
        "-p",
        package,
        "--test",
        test,
        "--",
        "--nocapture",
    ]
    result = subprocess.run(
        command, cwd=repo, text=True, capture_output=True, check=False, timeout=1800
    )
    output = result.stdout + result.stderr
    if result.returncode or "running 0 tests" in output or "test result: ok" not in output:
        errors.append(f"focused test failed or empty: {package}/{test}\n{output[-2000:]}")


def run_node(repo: Path, evidence: Path, identifier: str, errors: list[str]) -> None:
    container_evidence = "/src/" + str(evidence.relative_to(repo))
    command = [
        "docker", "compose", "--profile", "shell", "run", "--build", "--rm",
        "-e", f"LKJAGENT_ACCEPTANCE_ROOT={container_evidence}", "shell",
        "cargo", "run", "--locked", "-p", "lkjagent-xtask", "--", "gate", identifier,
    ]
    result = subprocess.run(
        command, cwd=repo, text=True, capture_output=True, check=False, timeout=1800
    )
    output = result.stdout + result.stderr
    marker = f"GATE_PASSED\t{identifier}"
    if result.returncode or marker not in output or "running 0 tests" in output:
        errors.append(f"node gate failed or empty: {identifier}\n{output[-2000:]}")


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: focused_gate.py REPO EVIDENCE", file=sys.stderr)
        return 2
    repo = Path(sys.argv[1]).resolve()
    evidence = Path(sys.argv[2]).resolve()
    packet = Path(__file__).resolve().parents[1]
    errors: list[str] = []
    configuration(repo, packet, errors)
    for package, test in TESTS:
        run_test(repo, package, test, errors)
    graph = ET.parse(packet / "00-bootstrap" / "workgraph.xml").getroot()
    nodes = [(node.findtext("id") or "").strip() for node in graph.findall("node")]
    for identifier in nodes:
        run_node(repo, evidence, identifier, errors)
    with tempfile.TemporaryDirectory(prefix="lkjagent-binary-data-") as data:
        environment = os.environ.copy()
        environment["LKJAGENT_DATA_DIR"] = data
        binary = subprocess.run(
            ["docker", "compose", "run", "--build", "--rm", "--no-deps", "agent",
             "sha256sum", "/usr/local/bin/lkjagent"],
            cwd=repo, env=environment, text=True, capture_output=True,
            check=False, timeout=1800,
        )
    match = re.search(r"\b([0-9a-f]{64})\b", binary.stdout)
    if binary.returncode or not match:
        errors.append("unable to fingerprint final runtime binary")
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print(f"PASS\tfocused_suites={len(TESTS)}\tnode_gates={len(nodes)}")
    print(f"binary_fingerprint\tsha256:{match.group(1)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
