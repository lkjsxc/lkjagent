#!/usr/bin/env python3
"""Capture the real native TUI under a Unix PTY without persisting raw frames."""

import fcntl
import json
import os
import pty
import select
import signal
import sqlite3
import struct
import subprocess
import sys
import termios
import time
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 5:
        print("usage: pty-recorder.py CAST BINARY DATA SCHEDULE", file=sys.stderr)
        return 2
    cast, binary, data, schedule = map(Path, sys.argv[1:])
    turns = owner_turns(schedule)
    started = time.monotonic()
    frames: list[list[object]] = []
    process, master = spawn(binary, data, frames, started)
    provider_started = False
    provider_finished = False
    search_sent = False
    activity_sent = False
    resized = False
    restarted = False
    sent = 0
    try:
        while time.monotonic() - started < 901.0:
            elapsed = time.monotonic() - started
            while sent < len(turns) and elapsed >= turns[sent][0]:
                send(master, turns[sent][1] + "\r", frames, started)
                sent += 1
            read_available(master, frames, started)
            provider = provider_status(data / "lkjagent.sqlite3")
            status = provider[0] if provider else None
            if status and not provider_started:
                duration = provider[1] / 1000.0 if provider[1] else 0.0
                frames.append([max(0.0, elapsed - duration), "m", "slow-start"])
                provider_started = True
            if provider_started and not provider_finished and status in {"succeeded", "failed"}:
                frames.append([elapsed, "m", "slow-end"])
                provider_finished = True
            if provider_started and not provider_finished and elapsed >= 0.25 and not search_sent:
                send(master, "\x06日本語検索\x1b", frames, started)
                search_sent = True
            if provider_started and not provider_finished and elapsed >= 0.40 and not resized:
                resize(master, 100, 32, frames, started)
                os.kill(process.pid, signal.SIGWINCH)
                resized = True
            if provider_started and not provider_finished and elapsed >= 0.55 and not activity_sent:
                send(master, "\x1bOQ", frames, started)
                activity_sent = True
            if elapsed >= 650.0 and not restarted:
                write_restart_marker(data / "lkjagent.sqlite3", cast.parent / "restart.marker")
                stop(process, master, frames, started)
                process, master = spawn(binary, data, frames, started)
                restarted = True
            if process.poll() is not None:
                return fail(f"TUI exited early with {process.returncode}")
            time.sleep(0.02)
        stop(process, master, frames, started)
    finally:
        if process.poll() is None:
            process.kill()
            process.wait(timeout=5)
        try:
            os.close(master)
        except OSError:
            pass
    if sent != len(turns) or not restarted:
        return fail("PTY schedule did not complete")
    write_cast(cast, frames)
    print(f"frame_count\t{len(frames)}")
    return 0


def owner_turns(path: Path) -> list[tuple[float, str]]:
    rows = path.read_text(encoding="utf-8").splitlines()[1:]
    turns = []
    for row in rows:
        fields = row.split("\t")
        if len(fields) != 4:
            raise ValueError("owner schedule row is malformed")
        turns.append((float(fields[0]), fields[3]))
    return turns


def spawn(binary: Path, data: Path, frames: list, started: float):
    master, slave = pty.openpty()
    resize(master, 80, 24, frames, started)
    process = subprocess.Popen(
        [str(binary), "--data", str(data), "tui"],
        stdin=slave,
        stdout=slave,
        stderr=slave,
        close_fds=True,
        start_new_session=True,
        env=os.environ.copy(),
    )
    os.close(slave)
    os.set_blocking(master, False)
    return process, master


def send(master: int, text: str, frames: list, started: float) -> None:
    payload = text.encode("utf-8")
    os.write(master, payload)
    frames.append([time.monotonic() - started, "i", text])


def resize(master: int, width: int, height: int, frames: list, started: float) -> None:
    fcntl.ioctl(master, termios.TIOCSWINSZ, struct.pack("HHHH", height, width, 0, 0))
    frames.append([time.monotonic() - started, "r", f"{width}x{height}"])


def read_available(master: int, frames: list, started: float) -> None:
    while select.select([master], [], [], 0)[0]:
        try:
            chunk = os.read(master, 16384)
        except (BlockingIOError, OSError):
            return
        if not chunk:
            return
        frames.append([time.monotonic() - started, "o", chunk.decode("utf-8", "replace")])


def provider_status(database: Path) -> tuple[str, int] | None:
    if not database.is_file():
        return None
    try:
        connection = sqlite3.connect(f"file:{database}?mode=ro", uri=True, timeout=0.05)
        row = connection.execute(
            "SELECT status,started_monotonic_ms,finished_monotonic_ms FROM provider_exchanges ORDER BY started_monotonic_ms LIMIT 1"
        ).fetchone()
        connection.close()
        if not row:
            return None
        duration = max(0, (row[2] or row[1]) - row[1])
        return row[0], duration
    except sqlite3.Error:
        return None


def write_restart_marker(database: Path, marker: Path) -> None:
    connection = sqlite3.connect(f"file:{database}?mode=ro", uri=True, timeout=1.0)
    rows = connection.execute(
        "SELECT id FROM conversation_messages ORDER BY sequence"
    ).fetchall()
    connection.close()
    if not rows:
        raise RuntimeError("restart marker has no durable messages")
    marker.write_text("".join(f"message\t{row[0]}\n" for row in rows), encoding="utf-8")


def stop(process, master: int, frames: list, started: float) -> None:
    send(master, "\x03", frames, started)
    try:
        process.wait(timeout=8)
    except subprocess.TimeoutExpired:
        process.terminate()
        process.wait(timeout=8)
    read_available(master, frames, started)
    os.close(master)


def write_cast(path: Path, frames: list) -> None:
    header = {"version": 2, "width": 80, "height": 24, "timestamp": 0}
    lines = [json.dumps(header, sort_keys=True)]
    lines.extend(json.dumps(frame, ensure_ascii=False) for frame in frames)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
