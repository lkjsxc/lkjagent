# Verification Tools

## Purpose

Define direct verification tools that replace routine shell command gates.

## verify.cargo

Runs cargo directly with no shell. `gate` is required and may be `fmt`,
`check`, `test`, or `clippy`. `package` is optional. `timeout` is optional
and bounded by policy. `fmt` maps to `cargo fmt --check`.

## verify.xtask

Runs `cargo run -p lkjagent-xtask -- <gate>` directly with no shell. Legal
gates are `check-docs`, `check-lines`, `check-style`,
`benchmark-check-corpus`, `quiet-test`, and `quiet-verify`.

## Status

implemented.
