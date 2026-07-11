#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import pty
import select
import subprocess
import sys
import time
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: pty-recorder.py CAST", file=sys.stderr)
        return 2
    cast_path = Path(sys.argv[1])
    master, slave = pty.openpty()
    command = [
        "/bin/sh",
        "-c",
        "IFS= read -r line; printf 'accepted:%s\\n' \"$line\"; "
        "sleep 0.02; printf 'frame:raw-pty-output\\n'",
    ]
    process = subprocess.Popen(
        command,
        stdin=slave,
        stdout=slave,
        stderr=slave,
        close_fds=True,
        start_new_session=True,
    )
    os.close(slave)
    started = time.monotonic()
    owner_input = "こんにちは evaluation owner\n"
    frames: list[list[object]] = []
    frames.append([time.monotonic() - started, "i", owner_input])
    os.write(master, owner_input.encode("utf-8"))
    while process.poll() is None:
        ready, _, _ = select.select([master], [], [], 0.2)
        if master in ready:
            try:
                chunk = os.read(master, 4096)
            except OSError:
                chunk = b""
            if chunk:
                frames.append(
                    [
                        time.monotonic() - started,
                        "o",
                        chunk.decode("utf-8", errors="replace"),
                    ]
                )
    try:
        tail = os.read(master, 4096)
    except OSError:
        tail = b""
    if tail:
        frames.append(
            [time.monotonic() - started, "o", tail.decode("utf-8", errors="replace")]
        )
    os.close(master)
    if process.returncode != 0:
        print(f"PTY child failed with {process.returncode}", file=sys.stderr)
        return 1
    header = {
        "version": 2,
        "width": 80,
        "height": 24,
        "timestamp": 0,
        "env": {"TERM": "xterm-256color"},
    }
    lines = [json.dumps(header, sort_keys=True)]
    lines.extend(json.dumps(frame, ensure_ascii=False) for frame in frames)
    cast_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"input_frames\t{sum(frame[1] == 'i' for frame in frames)}")
    print(f"output_frames\t{sum(frame[1] == 'o' for frame in frames)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
