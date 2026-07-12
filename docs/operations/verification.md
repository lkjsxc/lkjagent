# Verification

## Purpose

Define few substantive suites, Docker commands, and honest evidence rules.

## Focused Suites

The target program reuses these substantive suites rather than adding one shallow
gate per workgraph row:

```sh
cargo test --locked -p lkjagent-core contract_tables
cargo test --locked -p lkjagent-effects workspace_safety
cargo test --locked -p lkjagent-store durable_boundaries
cargo test --locked -p lkjagent-app product_flows
cargo test --locked -p lkjagent-app tui_contract
cargo test --locked -p lkjagent-xtask acceptance_negative
```

Most named filters are not implemented yet. `../current-state.md` owns that gap.
Current deterministic commands remain:

```sh
cargo run --locked -p lkjagent-xtask -- check-docs
cargo run --locked -p lkjagent-xtask -- check-lines
cargo run --locked -p lkjagent-xtask -- check-files
cargo run --locked -p lkjagent-xtask -- check-style
cargo run --locked -p lkjagent-xtask -- quiet test
cargo run --locked -p lkjagent-xtask -- quiet verify
```

## Docker

```sh
docker compose build --no-cache verify test lint
docker compose run --rm verify
docker compose run --rm test
docker compose run --rm lint
```

Final proof runs from a clean locked export with no source bind mount and records
source, image, and binary hashes.

## Semantic Gates

Parser tests and scripted responses may exercise mechanics but cannot pass model
semantics. Public file, recovery, daily, project, report, and TUI behavior require
the configured real endpoint. PTY requirements use raw terminal frames.

## Claims

A command that did not run did not pass. Nonzero output is failure evidence, not
a tested trailer. A historical receipt does not bind changed source. Final
success requires the acceptance command in `../evaluation/live-proof.md`.
