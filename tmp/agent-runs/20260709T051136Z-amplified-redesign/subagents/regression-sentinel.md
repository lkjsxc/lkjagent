# Regression Sentinel Report

## Current Facts

- Scope was report-only. I did not edit product docs or source.
- Required files read: `docs/current-state.md`,
  `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/README.md`,
  `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/regression-sentinel.md`,
  `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/13-acceptance/final-gates.md`,
  `README.md`, and `AGENTS.md`.
- `docs/current-state.md` says July 9 evidence exists under
  `tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/`; that
  directory exists and contains baseline, focused, final, proof, and
  protocol-experiments outputs.
- `git ls-files data/logs data/lkjagent.sqlite3 data/lkjagent.sqlite3-shm
  data/lkjagent.sqlite3-wal data/workspace` produced no tracked files, so the
  current-state claim that runtime `data/logs` rows are no longer committed as
  product evidence is currently consistent.
- Structural gates run in this lane:
  - `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs`
  - `cargo run -p lkjagent-xtask -- check-lines` -> `ok check-lines`
  - `cargo run -p lkjagent-xtask -- check-files` -> `ok check-files`
  - `cargo run -p lkjagent-xtask -- check-style` -> `ok check-style`
  - `cargo run -p lkjagent-xtask -- bench check-corpus` ->
    `ok bench check-corpus`
  - `cargo run -p lkjagent-xtask -- structure audit` -> `ok structure audit`
  - `cargo run -p lkjagent-xtask -- smoke replay` ->
    `ok smoke replay data=./tmp/smoke-replay-data`
  - `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify`
- I did not run `docker compose run --rm verify` in this report-only lane.

## Contradictions

- Stale xtask benchmark README:
  `crates/lkjagent-xtask/src/benchmark/README.md:5` says this is a
  "Placeholder command module while the plan-engine benchmark corpus is
  rebuilt", and line 9 says `mod.rs` is a "command stub that fails honestly
  until rebuilt". Current source contradicts this: `benchmark/mod.rs:7-15`
  dispatches `bench check-corpus` and `bench run --suite tiny --data DIR`;
  `benchmark/mod.rs:18-30` validates the corpus and prints success; and
  `benchmark/mod.rs:32-52` writes a benchmark report.
- Stale xtask structure README:
  `crates/lkjagent-xtask/src/structure/README.md:5-6` says this is a
  "Placeholder command module while the structure audit is rebuilt for the plan
  engine", and line 10 says `mod.rs` is a "command stub that fails honestly
  until rebuilt". Current source contradicts this: `structure/mod.rs:21-27`
  exposes the audit command, and `structure/mod.rs:39-50` checks expected
  current crates and removed crates.
- I did not find a current contradiction in `docs/current-state.md:102-127`
  about July 9 evidence paths or uncommitted runtime logs.
- I did not find product placeholder behavior admitted by source in this pass.
  Placeholder-like values are explicitly rejected in
  `crates/lkjagent-core/src/runtime_admission.rs` and artifact placeholder
  paths are checked in `crates/lkjagent-core/src/artifact_manifest.rs`.
- `fake` and `scripted` endpoint names appear in tests and test helpers. This is
  acceptable as test harness language, not product behavior, as long as docs do
  not present those outputs as live endpoint proof.

## Line-Limit Risk

- No `docs/**/*.md` or `crates/**/*.{md,rs}` file was over 200 lines.
- Files exactly at the 200-line ceiling:
  `crates/lkjagent-app/tests/tui_state.rs`,
  `crates/lkjagent-core/src/engine.rs`,
  `crates/lkjagent-store/src/plan_schema.rs`,
  `crates/lkjagent-xtask/src/proof_state.rs`.
- Files at 199 lines:
  `crates/lkjagent-app/src/model_call.rs`,
  `crates/lkjagent-core/src/engine_steps.rs`,
  `crates/lkjagent-llm/src/wire/response.rs`,
  `crates/lkjagent-xtask/src/experiment_protocol.rs`.
- Any future edit to those files needs either deletion, split, or nearby
  compaction before adding lines.

## Exact Docs Edits

- Update `crates/lkjagent-xtask/src/benchmark/README.md`.
  - Replace line 5 with: `Define benchmark corpus validation and tiny-suite
    report generation for xtask.`
  - Replace line 9 with: `- [mod.rs](mod.rs): benchmark command dispatch,
    corpus gate, argument parsing, and report writing.`
- Update `crates/lkjagent-xtask/src/structure/README.md`.
  - Replace lines 5-6 with: `Define the repository structure audit for current
    and removed crates.`
  - Replace line 10 with: `- [mod.rs](mod.rs): structure audit command and
    crate presence checks.`
- No edit to `docs/current-state.md` is required from this sentinel finding
  unless the benchmark/structure README cleanup is committed as a behavior-doc
  cleanup item. If it is, add a small current-state note only if the project
  treats source README truth fixes as ledger-moving behavior.

## Exact Source Edits

- No source edit is required for the two stale README contradictions. The
  current benchmark and structure source behavior is already implemented and
  passed the focused gates above.
- Do not rename or remove the benchmark/structure commands as a way to satisfy
  the stale README text; that would regress working gates.

## Tests To Add Or Update

- No new product behavior tests are required for the README wording fixes.
- Optional guard: add or extend a doc/readme assertion in
  `crates/lkjagent-xtask/tests/doc_crate_readmes.rs` so source README text
  containing "Placeholder command module" or "command stub" is rejected outside
  explicit test fixtures.
- Optional guard: add a targeted assertion that xtask source READMEs do not use
  stale rebuild language such as "until rebuilt" when the related command is
  wired into `main.rs`.

## Commands To Run

- After README-only fixes:
  - `cargo run -p lkjagent-xtask -- check-docs`
  - `cargo run -p lkjagent-xtask -- check-lines`
  - `cargo run -p lkjagent-xtask -- check-files`
  - `cargo run -p lkjagent-xtask -- check-style`
  - `cargo run -p lkjagent-xtask -- bench check-corpus`
  - `cargo run -p lkjagent-xtask -- structure audit`
- Before any stage/behavior completion claim, also run packet final gates:
  - `cargo run -p lkjagent-xtask -- smoke replay`
  - `cargo run -p lkjagent-xtask -- quiet verify`
  - `docker compose run --rm verify`

## Risks

- The stale README wording hides live command behavior as placeholder work and
  could cause future agents to delete or bypass already-working gates.
- Four files are already at the 200-line ceiling; casual additions there will
  fail `check-lines`.
- Several files at 198-199 lines are one or two lines away from breaking the
  line-limit contract.
- This lane did not execute Docker Compose, so it should not be used as final
  acceptance evidence for a stage or behavior-completion claim.

## Acceptance Items Affected

- Final gate `check-docs`: affected by stale source README wording.
- Final gate `check-lines`: affected by near-limit and ceiling files.
- Final gate `check-files`: currently passes, but any source README cleanup must
  preserve README coverage and file naming.
- Required evidence "final progress ledger with no unchecked items": affected
  only if the stale README cleanup is adopted as an unchecked follow-up.
- Docker final verification remains unaffected by this report, but still must
  run before claiming full packet completion.
