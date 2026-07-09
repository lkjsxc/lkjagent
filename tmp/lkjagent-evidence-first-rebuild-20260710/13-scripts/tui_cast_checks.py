from __future__ import annotations

import json
import hashlib
from pathlib import Path


REQUIRED_TRACE_EVENTS = {
    "owner_input",
    "japanese_input",
    "slow_call_input",
    "resize",
    "scroll_up",
    "scroll_down",
    "slash_command",
    "daemon_restart",
    "workbench_restart",
    "agent_update",
}


def digest(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def check_replay(
    directory: Path,
    result: dict[str, str],
    source_commit: str,
    errors: list[str],
) -> None:
    path = directory / "terminal-replay.tsv"
    cast = directory / "terminal.cast"
    trace = directory / "tui-trace.tsv"
    if not path.is_file() or not cast.is_file() or not trace.is_file():
        errors.append("terminal replay or raw input missing")
        return
    if result.get("replay_fingerprint") != digest(path):
        errors.append("terminal replay receipt missing or fingerprint mismatch")
        return
    values = dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )
    expected = {
        "source_commit": source_commit,
        "cast_fingerprint": digest(cast),
        "trace_fingerprint": digest(trace),
        "screen_mismatch_count": "0",
        "geometry_mismatch_count": "0",
        "transition_mismatch_count": "0",
    }
    if any(values.get(key) != value for key, value in expected.items()):
        errors.append("terminal replay receipt disagrees with raw inputs")
    try:
        if int(values.get("frame_count", "0")) < 100:
            errors.append("terminal replay contains fewer than 100 frames")
    except ValueError:
        errors.append("terminal replay frame count invalid")


def check_cast(path: Path, errors: list[str]) -> tuple[float, list[float]]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
        header = json.loads(lines[0])
        events = [json.loads(line) for line in lines[1:]]
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, IndexError) as error:
        errors.append(f"terminal cast is unreadable: {error}")
        return 0, []
    if not isinstance(header, dict):
        errors.append("terminal cast header is not an object")
        return 0, []
    try:
        dimensions_valid = int(header.get("width", 0)) >= 20 and int(header.get("height", 0)) >= 5
    except (TypeError, ValueError):
        dimensions_valid = False
    if header.get("version") != 2 or not dimensions_valid:
        errors.append("terminal cast header or dimensions invalid")
    moments: list[float] = []
    input_text = ""
    output_bytes = 0
    for event in events:
        if not isinstance(event, list) or len(event) != 3 or event[1] not in {"i", "o", "m"}:
            errors.append("terminal cast event malformed")
            continue
        try:
            moment = float(event[0])
        except (TypeError, ValueError):
            errors.append("terminal cast event timestamp invalid")
            continue
        moments.append(moment)
        if event[1] == "i":
            input_text += str(event[2])
        if event[1] == "o":
            output_bytes += len(str(event[2]).encode())
    if not moments or moments != sorted(moments) or moments[-1] < 840:
        errors.append("terminal cast does not span 840 ordered seconds")
    if output_bytes < 10000:
        errors.append("terminal cast contains too little rendered output")
    if input_text.count("\n") < 3 or not any(ord(character) > 127 for character in input_text):
        errors.append("terminal cast lacks recorded owner and Japanese input")
    return moments[-1] if moments else 0, moments
