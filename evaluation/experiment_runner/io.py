from __future__ import annotations

import csv
import hashlib
import json
import os
import subprocess
from pathlib import Path

SOURCE_PATHS = (
    "Cargo.toml", "Cargo.lock", ".cargo", "rust-toolchain", "rust-toolchain.toml",
    "crates", "docs", "evaluation", "Dockerfile", "docker-compose.yml", "config/lkjagent.example.json",
)


def sha(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def file_sha(path: Path) -> str:
    return sha(path.read_bytes())


def tree_sha(root: Path) -> str:
    value = bytearray()
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        value.extend(str(path.relative_to(root)).encode()); value.append(0)
        value.extend(path.read_bytes()); value.append(0)
    return sha(bytes(value))


def source_sha(root: Path) -> str:
    names = subprocess.check_output(["git", "ls-files", "-z", "--", *SOURCE_PATHS], cwd=root).split(b"\0")
    value = bytearray()
    for raw in sorted(item for item in names if item):
        path = root / raw.decode()
        if path.is_symlink():
            raise RuntimeError(f"source input is a symlink: {path}")
        value.extend(raw); value.append(0); value.extend(path.read_bytes()); value.append(0)
    return sha(bytes(value))


def table(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_table(path: Path, fields: list[str], rows: list[dict[str, object]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader(); writer.writerows(rows)


def pairs(path: Path, values: list[tuple[str, object]]) -> None:
    path.write_text("".join(f"{key}\t{value}\n" for key, value in values), encoding="utf-8")


def safe_env(build: bool = False) -> dict[str, str]:
    exact = {"PATH", "HOME", "CARGO_HOME", "RUSTUP_HOME", "SSL_CERT_FILE", "SSL_CERT_DIR",
        "HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY", "NO_PROXY"}
    if not build:
        exact |= {"LKJAGENT_ENDPOINT_URL", "LKJAGENT_MODEL", "LKJAGENT_API_KEY", "LKJAGENT_ENDPOINT_TIMEOUT_SECONDS", "TZ"}
    return {key: value for key, value in os.environ.items() if key in exact}


def build_env(scratch: Path, source: Path) -> dict[str, str]:
    env = safe_env(True); cargo_home = scratch / "cargo-home"; cargo_home.mkdir()
    original = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    for name in ("registry", "git"):
        path = original / name
        if path.exists():
            (cargo_home / name).symlink_to(path, target_is_directory=True)
    env["CARGO_HOME"] = str(cargo_home)
    env["RUSTFLAGS"] = (f"--remap-path-prefix={source}=/lkjagent-source "
        f"--remap-path-prefix={original}=/cargo-home --remap-path-prefix={cargo_home}=/cargo-home")
    return env


def command(args: list[str], log: list[str], log_path: Path, env: dict[str, str]) -> int:
    shown = args[:-1] + [f"<owner-text {sha(args[-1].encode())}>"] if "send" in args else args
    try:
        result = subprocess.run(args, text=True, capture_output=True, env=env, timeout=600)
        log.extend(("$ " + " ".join(shown), result.stdout, result.stderr, f"exit={result.returncode}"))
        code = result.returncode
    except subprocess.TimeoutExpired as error:
        log.extend(("$ " + " ".join(shown), str(error.stdout or ""), str(error.stderr or ""), "exit=124 timeout")); code = 124
    log_path.write_text("\n".join(log), encoding="utf-8")
    return code
