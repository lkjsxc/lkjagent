#!/usr/bin/env python3
from __future__ import annotations

import re
import stat
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def check_directory(root: Path, directory: Path, errors: list[str]) -> None:
    children = [item for item in directory.iterdir() if not item.name.startswith(".")]
    if directory != root and not (directory / "README.md").is_file():
        fail(errors, f"missing README.md: {directory.relative_to(root)}")
    useful = [item for item in children if item.name != "README.md"]
    if directory != root and len(useful) < 2:
        fail(errors, f"directory needs two useful children: {directory.relative_to(root)}")
    readme = directory / "README.md"
    if readme.is_file():
        text = readme.read_text(encoding="utf-8")
        for child in useful:
            if child.name not in text:
                fail(errors, f"README omits child {child.name}: {directory.relative_to(root)}")


def check_text(root: Path, path: Path, errors: list[str]) -> None:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        fail(errors, f"non-UTF-8 text file: {path.relative_to(root)}")
        return
    lines = text.splitlines()
    if len(lines) > 200:
        fail(errors, f"over 200 lines: {path.relative_to(root)} has {len(lines)}")
    if path.suffix == ".md":
        if not lines or not lines[0].startswith("# "):
            fail(errors, f"missing single H1 start: {path.relative_to(root)}")
        if sum(1 for line in lines if line.startswith("# ")) != 1:
            fail(errors, f"Markdown must contain exactly one H1: {path.relative_to(root)}")
        if re.search(r"(?i)\bversion\b|\bv[0-9]+\b", text):
            fail(errors, f"release-style naming: {path.relative_to(root)}")
        if re.search(r"(?i)\b(TBD|FIXME)\b", text):
            fail(errors, f"unfinished marker: {path.relative_to(root)}")
        check_links(root, path, text, errors)


def check_links(root: Path, path: Path, text: str, errors: list[str]) -> None:
    for match in re.finditer(r"\[[^\]]+\]\(([^)]+)\)", text):
        target = match.group(1).split("#", 1)[0]
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        resolved = (path.parent / target).resolve()
        if root.resolve() not in (resolved, *resolved.parents) or not resolved.exists():
            fail(errors, f"broken link in {path.relative_to(root)}: {target}")


def main() -> int:
    default = Path(__file__).resolve().parents[1]
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else default
    errors: list[str] = []
    if not (root / "README.md").is_file():
        errors.append("packet root README.md missing")
    for item in (root, *root.rglob("*")):
        mode = item.lstat().st_mode
        if item.is_symlink() or not (stat.S_ISREG(mode) or stat.S_ISDIR(mode)):
            errors.append(f"symlink or special file forbidden: {item.relative_to(root)}")
    for directory in sorted(item for item in root.rglob("*") if item.is_dir()):
        check_directory(root, directory, errors)
    suffixes = {".md", ".py", ".sh", ".xml", ".tsv"}
    files = sorted(item for item in root.rglob("*") if item.is_file())
    for path in files:
        if path.suffix in suffixes:
            check_text(root, path, errors)
    try:
        tree = ET.parse(root / "00-bootstrap" / "workgraph.xml")
        nodes = tree.getroot().findall("node")
        identifiers = [(node.findtext("id") or "").strip() for node in nodes]
        if not identifiers or len(identifiers) != len(set(identifiers)):
            errors.append("workgraph IDs missing or duplicated")
        known = set(identifiers)
        for node, identifier in zip(nodes, identifiers, strict=True):
            depends = set((node.findtext("depends") or "").split()) - {"none"}
            if depends - known or identifier in depends:
                errors.append(f"workgraph dependency invalid: {identifier}")
            if (node.findtext("gate") or "").strip() != identifier:
                errors.append(f"workgraph gate missing or mismatched: {identifier}")
    except (OSError, ET.ParseError) as error:
        errors.append(f"invalid workgraph.xml: {error}")
    if errors:
        for error in errors:
            print(f"FAIL\t{error}")
        return 1
    print(f"PASS\tpacket_files={len(files)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
