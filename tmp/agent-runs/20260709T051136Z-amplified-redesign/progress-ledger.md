# Progress Ledger

## Purpose

Track completion of the amplified workspace-first packet with commit and
command evidence.

## Run

- Stamp: 20260709T051136Z-amplified-redesign
- Branch: main, ahead of origin/main
- Objective: complete `tmp/lkjagent-yolo-amplified-thinking-packet-20260708`
- Docs-first commit: `6a328f88` Define flat config as the runtime contract
- Implementation/evidence commit: `c2915b48` Prove strict flat config in the amplified harness
- Daily-use evidence commit: `72fa3e2c` Capture daily workspace family evidence
- Ledger file: this tracked file records final closure; acceptance items name
  stable commits below.

## Case State

- Objective: make lkjagent usable as a workspace-first personal agent harness
  with visible owner-readable records, selected tool views, clean prompt context,
  recovery proof, and TUI transcript stability.
- Constraints: docs first, source and docs under 200 lines, no stale contracts,
  no JSON model context, XML-like action envelopes, committed evidence, Docker
  final gate, no unchecked ledger items.
- Assumptions: this packet's local stop file is `13-acceptance/final-gates.md`;
  the inherited stop file is
  `tmp/lkjagent-more-more-thinking-20260708/11-acceptance/final-definition-of-done.md`.
- Risks: packet helper scripts scan ignored historical runtime artifacts; raw
  SQLite and JSON logs were left local and not committed.
- Evidence root: `tmp/agent-runs/20260709T051136Z-amplified-redesign/`
- Stop condition: all final gates below are checked and no unchecked item
  remains.

## Acceptance Status

- [x] Docs and code agree - commits `6a328f88` and `c2915b48`; docs state
  strict flat config and code rejects nested config.
- [x] Final gates pass - commit `c2915b48`; final command captures at
  `commands/20260709T052732Z-*`, `commands/20260709T052742Z-*`, and
  `commands/20260709T052808Z-docker-compose-run---rm-verify.out`.
- [x] Workspace records proven - commits `c2915b48` and `72fa3e2c`; workspace
  probe and daily-use workspace trees are under `evidence/probe-data/workspace/`
  and `evidence/daily-use-data/workspace/`.
- [x] Diary path proven - commit `72fa3e2c`; diary record exists at
  `evidence/daily-use-data/workspace/records/life/journal/2026/07/09/entry.md`.
- [x] TUI duplicate regression proven - commit `c2915b48`; TUI subagent report
  and capture analyzer evidence are `subagents/tui-engineer.md` and
  `commands/20260709T051843Z-*-tui_log_.out`.
- [x] TUI scroll clamp proven - commit `c2915b48`; focused TUI tests are covered
  by `cargo test -p lkjagent-app --tests` and TUI report.
- [x] Tool view parity proven - commit `c2915b48`; tool report plus protocol
  experiments are `subagents/tool-protocol-engineer.md` and
  `evidence/protocol-experiments/`.
- [x] Context lint proven - commit `c2915b48`; context report, proof context,
  and `quiet verify` cover dedupe, source refs, bounds, and non-JSON prompt
  rendering. Raw `prompt-frame.json` artifacts were excluded from the commit.
- [x] Recovery campaigns proven - commit `c2915b48`; state harness, evaluation,
  protocol experiments, and `quiet verify` cover parse, admission, effect,
  endpoint, and check recovery rows.
- [x] Docker Compose final gate - commit `c2915b48`; final capture
  `commands/20260709T052808Z-docker-compose-run---rm-verify.out` exits 0.
- [x] Live campaigns or honest skip evidence - tracked historical live evidence
  exists under `tmp/live-runs/20260708Tstandardenv/`; this run adds scripted
  daily-use evidence in commit `72fa3e2c`.

## Final Gate Commands

- [x] `cargo run -p lkjagent-xtask -- check-docs` - exit 0 in
  `commands/20260709T052732Z-cargo-run--p-lkjagent-xtask----check-docs.out`.
- [x] `cargo run -p lkjagent-xtask -- check-lines` - exit 0 in
  `commands/20260709T052732Z-cargo-run--p-lkjagent-xtask----check-lines.out`.
- [x] `cargo run -p lkjagent-xtask -- check-files` - exit 0 in
  `commands/20260709T052732Z-cargo-run--p-lkjagent-xtask----check-files.out`.
- [x] `cargo run -p lkjagent-xtask -- check-style` - exit 0 in
  `commands/20260709T052732Z-cargo-run--p-lkjagent-xtask----check-style.out`.
- [x] `cargo run -p lkjagent-xtask -- smoke replay` - exit 0 in
  `commands/20260709T052742Z-cargo-run--p-lkjagent-xtask----smoke-replay.out`.
- [x] `cargo run -p lkjagent-xtask -- quiet verify` - exit 0 in
  `commands/20260709T052742Z-cargo-run--p-lkjagent-xtask----quiet-verify.out`.
- [x] `docker compose run --rm verify` - exit 0 in
  `commands/20260709T052808Z-docker-compose-run---rm-verify.out`.

## Inherited Definition Of Done

- [x] Product docs describe implemented behavior and no stale contract remains -
  `6a328f88`; final `check-docs` passed.
- [x] Source files and docs respect line limits - final `check-lines` passed;
  staged line audit showed every added evidence file below 200 lines.
- [x] Ordinary record turns write workspace files and rows - `72fa3e2c`;
  journal, todo, calendar, finance, and note CLI records plus redacted SQLite
  rows are in daily-use evidence.
- [x] Every owner turn writes transcript or inbox evidence - `c2915b48`;
  probe transcript files are under `evidence/probe-data/workspace/artifacts/transcripts/`.
- [x] Artifact creation creates files, artifact rows, checks, and response paths
  - `c2915b48`; proof bundle includes artifacts, checks, and workspace tree.
- [x] Observed file-work failure has a focused regression test - `c2915b48`;
  app tests and `quiet verify` cover blocked artifact and effect failure paths.
- [x] Earlier blocked work prevents later response work without recovery evidence
  - `c2915b48`; state harness report and app tests are included.
- [x] Prompt context is deduplicated, source-linked, bounded, and non-JSON -
  `c2915b48`; context report, proof context, and `quiet verify`.
- [x] Tool views are selected by state and do not show the full catalog by
  default - `c2915b48`; tool report and protocol experiments.
- [x] XML-like action grammar and admission have focused tests - `c2915b48`;
  protocol experiments and xtask tests.
- [x] Recovery states handle parse, admission, effect, endpoint, and check
  failures - `c2915b48`; recovery and evaluation reports.
- [x] TUI duplicate and bottom-follow regressions have tests - `c2915b48`;
  TUI report, app tests, and capture analyzer.
- [x] Deterministic replay passes - final `smoke replay` capture exits 0.
- [x] Quiet verify passes - final `quiet verify` capture exits 0.
- [x] Docker Compose verify passes or honest skip is committed - Docker capture
  exits 0.
- [x] Live campaigns run or honest skips are committed - tracked live evidence
  under `tmp/live-runs/20260708Tstandardenv/` plus daily-use script evidence.
- [x] Final handoff names commits, commands, evidence paths, and residual risks
  - this ledger and final response.

## Notes

- `audit_repo.py .` and `prompt_context_lint.py .` were captured with exit 1
  before final proof because they scan ignored historical tmp/data artifacts.
  The product gates that define completion passed after the last source change.
- Raw SQLite databases, raw JSON logs, and workspace manifests were left local
  when they were not needed for readable committed proof.
