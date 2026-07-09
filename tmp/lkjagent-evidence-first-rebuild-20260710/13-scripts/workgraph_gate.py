#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


def fields(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def graph(path: Path) -> dict[str, tuple[list[str], str]]:
    nodes: dict[str, tuple[list[str], str]] = {}
    root = ET.parse(path).getroot()
    if root.tag != "workgraph" or root.attrib:
        raise ValueError("invalid workgraph root")
    for node in root.findall("node"):
        identifier = (node.findtext("id") or "").strip()
        depends = (node.findtext("depends") or "").split()
        gate = (node.findtext("gate") or "").strip()
        if not identifier or identifier in nodes or gate != identifier:
            raise ValueError(f"invalid or duplicate node: {identifier}")
        nodes[identifier] = ([] if depends == ["none"] else depends, gate)
    for identifier, (depends, _) in nodes.items():
        unknown = set(depends) - nodes.keys()
        if unknown or identifier in depends:
            raise ValueError(f"invalid dependency for {identifier}: {sorted(unknown)}")
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(identifier: str) -> None:
        if identifier in visiting:
            raise ValueError(f"workgraph cycle at {identifier}")
        if identifier in visited:
            return
        visiting.add(identifier)
        for dependency in nodes[identifier][0]:
            visit(dependency)
        visiting.remove(identifier)
        visited.add(identifier)

    for identifier in nodes:
        visit(identifier)
    return nodes


def expected_command(identifier: str) -> str:
    return (
        "docker compose --profile shell run --rm shell cargo run --locked "
        f"-p lkjagent-xtask -- gate {identifier}"
    )


def receipt(
    evidence: Path,
    identifier: str,
    depends: list[str],
    sequences: dict[str, int],
    used: set[Path],
) -> tuple[bool, str]:
    node_root = evidence / "nodes" / identifier
    result = node_root / "result.tsv"
    if not result.is_file():
        return False, "missing"
    data = fields(result)
    required = {
        "node_id",
        "status",
        "source_commit",
        "completed_sequence",
        "gate_command",
        "gate_exit_code",
        "dependency_receipt_hashes",
        "evidence_refs",
        "evidence_hashes",
        "verifier_ref",
        "verifier_hash",
    }
    if required - data.keys():
        return False, "receipt fields missing"
    if data["node_id"] != identifier or data["status"] != "passed":
        return False, "identity or status mismatch"
    if data["gate_command"] != expected_command(identifier) or data["gate_exit_code"] != "0":
        return False, "anchored gate command did not pass"
    if not re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", data["source_commit"]):
        return False, "source commit malformed"
    try:
        sequence = int(data["completed_sequence"])
    except ValueError:
        return False, "completion sequence malformed"
    if sequence in sequences.values():
        return False, "completion sequence is duplicated"
    if any(sequences.get(item, sequence) >= sequence for item in depends):
        return False, "dependency completion order invalid"
    dependency_hashes = [item for item in data["dependency_receipt_hashes"].split(",") if item]
    expected_hashes = [digest(evidence / "nodes" / item / "result.tsv") for item in depends]
    if dependency_hashes != expected_hashes:
        return False, "dependency receipt hashes disagree"
    refs = [Path(item) for item in data["evidence_refs"].split(",") if item]
    hashes = [item for item in data["evidence_hashes"].split(",") if item]
    if len(refs) < 2 or len(refs) != len(hashes):
        return False, "at least two hashed evidence files are required"
    root = evidence.resolve()
    raw = (node_root / "raw").resolve()
    for ref, expected in zip(refs, hashes, strict=True):
        candidate = (root / ref).resolve()
        if raw not in candidate.parents or candidate in used or candidate.is_symlink():
            return False, "evidence path is escaped, reused, or symlinked"
        if not candidate.is_file() or digest(candidate) != expected:
            return False, "evidence file or hash mismatch"
        used.add(candidate)
    verifier = (root / data["verifier_ref"]).resolve()
    if node_root.resolve() not in verifier.parents or not verifier.is_file():
        return False, "verifier receipt missing or escaped"
    if digest(verifier) != data["verifier_hash"] or verifier.stat().st_size < 80:
        return False, "verifier receipt hash or content invalid"
    sequences[identifier] = sequence
    return True, "passed"


def main() -> int:
    if len(sys.argv) not in {3, 4} or (len(sys.argv) == 4 and sys.argv[3] != "--validate"):
        print("usage: workgraph_gate.py PACKET EVIDENCE [--validate]", file=sys.stderr)
        return 2
    packet = Path(sys.argv[1]).resolve()
    evidence = Path(sys.argv[2]).resolve()
    try:
        nodes = graph(packet / "00-bootstrap" / "workgraph.xml")
    except (OSError, ET.ParseError, ValueError) as error:
        print(f"FAIL\t{error}")
        return 1
    if len(sys.argv) == 4:
        print(f"PASS\tworkgraph_nodes={len(nodes)}")
        return 0
    passed: set[str] = set()
    invalid: list[str] = []
    sequences: dict[str, int] = {}
    used: set[Path] = set()
    for identifier, (depends, _) in nodes.items():
        valid, reason = receipt(evidence, identifier, depends, sequences, used)
        if valid:
            passed.add(identifier)
        elif reason != "missing":
            invalid.append(f"{identifier}: {reason}")
    released = [
        identifier
        for identifier, (depends, _) in nodes.items()
        if identifier not in passed and all(item in passed for item in depends)
    ]
    for identifier in released:
        print(f"next\t{identifier}\t{expected_command(identifier)}")
    if not released and len(passed) == len(nodes):
        print("SOURCE_FREEZE_EVIDENCE_GRAPH_COMPLETE")
    for item in invalid:
        print(f"invalid\t{item}")
    return 1 if invalid else 0


if __name__ == "__main__":
    raise SystemExit(main())
