# Verification Summary

## Purpose

Record commands run for the continued prompt-kernel, field-spec, experiment,
workbench, live-smoke, and proof slices.

## Commands Run

- `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs`
- `cargo run -p lkjagent-xtask -- check-lines` -> `ok check-lines`
- `cargo run -p lkjagent-xtask -- check-files` -> `ok check-files`
- `cargo run -p lkjagent-xtask -- check-style` -> `ok check-style`
- `cargo test -p lkjagent-core --test admission --test prompt_kernel --test render --test parse_contract` -> passed
- `cargo test -p lkjagent-store --test plan_store --test state_ledger` -> passed
- `cargo test -p lkjagent-app --test prompt_frame --test cli_rows --test diagnostics` -> passed
- `cargo test -p lkjagent-app --test args --test app --test cli_rows` -> passed earlier in this run
- `cargo test -p lkjagent-app workbench` -> passed earlier in this run
- `cargo test -p lkjagent-xtask --test protocol_experiment` -> passed
- `cargo run -p lkjagent-xtask -- experiment protocol --profile baseline --out tmp/protocol-experiments/20260706T084725Z-profiles/baseline.md` -> ok
- `cargo run -p lkjagent-xtask -- experiment protocol --profile strict-tool-card --out tmp/protocol-experiments/20260706T084725Z-profiles/strict-tool-card.md` -> ok
- `cargo run -p lkjagent-xtask -- experiment protocol --profile field-spec-kernel --out tmp/protocol-experiments/20260706T084725Z-profiles/field-spec-kernel.md` -> ok
- `cargo run -p lkjagent-xtask -- smoke replay` -> `ok smoke replay data=./tmp/smoke-replay-data`
- `cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current-20260706T090752Z` -> ok
- `cargo run -p lkjagent-xtask -- quiet test` -> `ok test`
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify`
- `docker compose run --rm verify` -> `ok verify`
- `docker compose run --rm test` -> `ok test`
- `docker compose run --rm lint` -> `ok check-docs`, `ok check-lines`, `ok check-style`
- `docker compose run --rm bench` -> `ok bench check-corpus`
- `docker compose run --rm replay` -> `ok smoke replay data=./tmp/smoke-replay-data`

## Live Evidence

- `tmp/live-runs/20260706T084837Z-live-smoke/summary.md` -> closed task, proof collected.
- `tmp/live-runs/20260706T085235Z-profile-smokes/strict-tool-card/summary.md` -> closed task, proof collected.
- `tmp/live-runs/20260706T085235Z-profile-smokes/field-spec-kernel/summary.md` -> closed task, proof collected.
- `tmp/live-runs/20260706T090156Z-prompt-card-rows/summary.md` -> closed task and 24 prompt-card proof rows.

## Not Run

No ten-hour or fifteen-minute-per-profile story proof ran; the completed live
smokes reached terminal state early and the longer story proof remains budgeted
future work.
