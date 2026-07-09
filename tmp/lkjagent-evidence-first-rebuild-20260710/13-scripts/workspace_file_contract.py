from __future__ import annotations

import math
import re
import datetime as dt
import xml.etree.ElementTree as ET


ID = re.compile(r"^[a-z][a-z0-9-]*_[0-9]{8}t[0-9]{6}z_[a-z2-7]{6,16}$")
DATE = re.compile(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")
KINDS = {
    "activity",
    "artifact-section",
    "calendar",
    "decision",
    "external-source",
    "finance",
    "generated-index",
    "journal",
    "navigation",
    "note",
    "project-note",
    "project-source",
    "session",
    "todo",
}
STATES = {"active", "archived", "done", "invalid", "open", "superseded", "waiting"}
FIELDS = {
    "document_id",
    "kind",
    "effective_date",
    "state",
    "source_ref",
    "related_document",
}


def conservative_tokens(text: str) -> int:
    total = 0
    ascii_run = 0
    for character in text:
        if character.isascii() and character.isalnum():
            ascii_run += len(character.encode())
            continue
        if ascii_run:
            total += math.ceil(ascii_run / 3)
            ascii_run = 0
        if not character.isspace():
            total += 1
    return total + math.ceil(ascii_run / 3)


def metadata(text: str, relative: str, errors: list[str]) -> tuple[dict[str, list[str]], str]:
    lines = text.splitlines()
    if not lines or not lines[0].startswith("# ") or len(lines[0]) < 4:
        errors.append(f"managed Markdown lacks meaningful leading H1: {relative}")
    if text.count("<lkjagent_record>") != 1 or text.count("</lkjagent_record>") != 1:
        errors.append(f"managed Markdown needs exactly one metadata block: {relative}")
        return {}, ""
    start = text.find("<lkjagent_record>")
    end = text.find("</lkjagent_record>")
    if start != text.find("\n\n") + 2 or end < start:
        errors.append(f"metadata block is not immediately after H1: {relative}")
    fragment = text[start : end + len("</lkjagent_record>")]
    try:
        root = ET.fromstring(fragment)
    except ET.ParseError as error:
        errors.append(f"invalid metadata XML in {relative}: {error}")
        return {}, ""
    if root.tag != "lkjagent_record" or root.attrib:
        errors.append(f"metadata root or attributes invalid: {relative}")
    values: dict[str, list[str]] = {}
    for item in root:
        if item.tag not in FIELDS or item.attrib or list(item):
            errors.append(f"invalid metadata field {item.tag}: {relative}")
            continue
        values.setdefault(item.tag, []).append(item.text or "")
    for scalar in ("document_id", "kind", "effective_date", "state"):
        if len(values.get(scalar, [])) != 1:
            errors.append(f"metadata scalar {scalar} is not singular: {relative}")
    body = text[end + len("</lkjagent_record>") :].strip()
    if len(re.sub(r"[#*_`\s-]", "", body)) < 20:
        errors.append(f"managed record has no meaningful body: {relative}")
    return values, body


def check_record(
    relative: str, kind: str, document_id: str, body: bytes, errors: list[str]
) -> None:
    try:
        text = body.decode("utf-8")
    except UnicodeDecodeError:
        errors.append(f"managed record is not UTF-8: {relative}")
        return
    values, content = metadata(text, relative, errors)
    if values.get("document_id", [None])[0] != document_id or not ID.fullmatch(document_id):
        errors.append(f"document ID/header invalid: {relative}")
    if values.get("kind", [None])[0] != kind:
        errors.append(f"kind/header mismatch: {relative}")
    date = values.get("effective_date", [""])[0]
    state = values.get("state", [""])[0]
    try:
        dt.date.fromisoformat(date)
        valid_date = bool(DATE.fullmatch(date))
    except ValueError:
        valid_date = False
    if not valid_date or state not in STATES:
        errors.append(f"effective date or state invalid: {relative}")
    references = values.get("source_ref", []) + values.get("related_document", [])
    if any(not ID.fullmatch(item) for item in references):
        errors.append(f"invalid metadata reference: {relative}")
    if kind == "journal" and relative != f"life/journal/{date.replace('-', '/')}/entry.md":
        errors.append(f"journal path/date mismatch: {relative}")
    lowered = content.lower()
    for banned in ("no specific diary details were provided", "state: queue:"):
        if banned in lowered:
            errors.append(f"banned generated text in {relative}: {banned}")
