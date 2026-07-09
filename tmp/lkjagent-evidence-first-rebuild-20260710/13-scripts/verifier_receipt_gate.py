from __future__ import annotations

import csv
import hashlib
from pathlib import Path


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    value = hashlib.sha256()
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        value.update(str(path.relative_to(root)).encode())
        value.update(b"\0")
        value.update(path.read_bytes())
        value.update(b"\0")
    return "sha256:" + value.hexdigest()


def fields(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def check_verifier(
    evidence: Path,
    packet: Path,
    source_commit: str,
    evidence_material_commit: str,
    errors: list[str],
) -> None:
    receipt_path = evidence / "independent-verifier.tsv"
    if not receipt_path.is_file():
        errors.append("independent verifier receipt missing")
        return
    receipt = fields(receipt_path)
    expected = {
        "source_commit": source_commit,
        "evidence_material_commit": evidence_material_commit,
        "packet_fingerprint": tree_digest(packet),
        "status": "passed",
    }
    if any(receipt.get(key) != value for key, value in expected.items()):
        errors.append("independent verifier identity or status mismatch")
    root = evidence.resolve()
    files: dict[str, Path] = {}
    for key in ("report_ref", "command_log_ref", "artifact_manifest_ref"):
        candidate = (root / receipt.get(key, "")).resolve()
        if root not in candidate.parents or candidate.is_symlink() or not candidate.is_file():
            errors.append(f"verifier {key} missing or escaped")
        else:
            files[key] = candidate
            if receipt.get(key.replace("_ref", "_fingerprint")) != digest(candidate):
                errors.append(f"verifier {key} fingerprint mismatch")
    report = files.get("report_ref")
    command_log = files.get("command_log_ref")
    manifest = files.get("artifact_manifest_ref")
    if report and (report.stat().st_size < 500 or "# Verdict" not in report.read_text(encoding="utf-8")):
        errors.append("verifier report is not substantive")
    if command_log and command_log.stat().st_size < 200:
        errors.append("verifier command log is empty")
    if not manifest:
        return
    rows = list(csv.DictReader(manifest.open(encoding="utf-8"), delimiter="\t"))
    if len(rows) < 10 or not {"path", "fingerprint"}.issubset(rows[0] if rows else {}):
        errors.append("verifier artifact manifest is incomplete")
        return
    seen: set[Path] = set()
    for row in rows:
        candidate = (root / row["path"]).resolve()
        if root not in candidate.parents or candidate in seen or not candidate.is_file():
            errors.append("verifier manifest path invalid or duplicated")
            continue
        seen.add(candidate)
        if row["fingerprint"] != digest(candidate):
            errors.append(f"verifier artifact fingerprint mismatch: {row['path']}")
