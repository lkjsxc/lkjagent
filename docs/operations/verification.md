# Verification

## Purpose

Define few substantive suites, Docker commands, and honest evidence rules.

## Focused Suites

The target program reuses these substantive suites rather than adding one shallow
gate per workgraph row:

```sh
cargo test --locked -p lkjagent-core
cargo test --locked -p lkjagent-effects
cargo test --locked -p lkjagent-store
cargo test --locked -p lkjagent-app
cargo test --locked -p lkjagent-app --test tui_contract
cargo test --locked -p lkjagent-app --test tui_native --test tui_responsive --test tui_terminal_guard --test tui_pty
cargo test --locked -p lkjagent-xtask --test acceptance_negative
```

These suites execute substantive tests; zero-test filters are not evidence. The
current deterministic commands are:

```sh
cargo run --locked -p lkjagent-xtask -- check-docs
cargo run --locked -p lkjagent-xtask -- check-lines
cargo run --locked -p lkjagent-xtask -- check-files
cargo run --locked -p lkjagent-xtask -- check-style
cargo run --locked -p lkjagent-xtask -- gate docs-authority
cargo run --locked -p lkjagent-xtask -- gate evaluation-harness
git diff --check SOURCE..HEAD
```

## Docker

```sh
docker compose build --no-cache verify test lint agent
docker compose run --rm verify
docker compose run --rm test
docker compose run --rm lint
```

Final proof runs from a clean locked export with no source bind mount and records
source, image, and binary hashes. The Docker build export has no VCS metadata, so
baseline semantic/hash validation runs there while tracked-file and reachable-
history checks remain mandatory host gates.

## Native TUI

Tests use real SQLite intake and frame projection, a test-only blocking endpoint
to prove input independence, and a native binary Unix PTY cleanup smoke. They are
not a configured endpoint or final PTY campaign.

## Semantic Gates

Parser tests and scripted responses may exercise mechanics but cannot pass model
semantics. Public file, recovery, daily, project, report, and TUI behavior require
the configured real endpoint. PTY requirements use raw terminal frames.

## Claims

A command that did not run did not pass. Nonzero output is failure evidence, not
a tested trailer. A historical receipt does not bind changed source. Final
success requires the acceptance command in `../evaluation/live-proof.md`.
