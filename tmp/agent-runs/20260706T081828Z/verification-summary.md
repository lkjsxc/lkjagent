# Verification Summary

## Purpose

Record commands run for the workbench and strict tool-card implementation slice.

## Commands Run

- `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs`
- `cargo run -p lkjagent-xtask -- check-lines` -> `ok check-lines`
- `cargo run -p lkjagent-xtask -- check-files` -> `ok check-files`
- `cargo run -p lkjagent-xtask -- check-style` -> `ok check-style`
- `cargo test -p lkjagent-core --test admission --test parse_contract --test render --test state_runtime` -> passed
- `cargo test -p lkjagent-app --test args --test app --test cli_rows` -> passed
- `cargo test -p lkjagent-app workbench` -> passed
- `cargo test -p lkjagent-xtask --test protocol_experiment` -> passed
- `cargo run -p lkjagent-xtask -- experiment protocol --out tmp/protocol-experiments/20260706T083144Z-strict-tool-card.md` -> `ok experiment protocol`
- `cargo run -p lkjagent-xtask -- quiet test` -> `ok test`
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify`
- `docker compose run --rm verify` -> `ok verify`
- `printf '/quit\n' | cargo run -p lkjagent-app -- --data "$tmpdir" workbench` -> printed workbench refresh and `console: bye`

## Not Run

No endpoint-backed live trial ran; no endpoint credentials were provided.
