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
- Ledger file: this tracked file is reopened. Historical checks below are
  evidence, not final closure, until the corrective items have no open rows.

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
- Stop condition: reopened after owner report. Final completion is not valid
  until current corrective rows have no open item.

## Corrective Reopen

- `97a8803d` fixed the live TUI pane duplicate: copied store data showed
  `stepdone` and `taskclosed` rows in the transcript pane before the fix; final
  evidence shows only `owner: hello` and `agent: hello`.
- `8a6ea73a` fixed the finance workspace gate: a finance CLI turn now writes a
  month-grouped finance record, `indexes/budget-month.md`, and
  `index-budget-month` artifact row evidence.
- `57c39d2e` fixed oversized record bodies: main records now carry size
  justification and links to `.parts/part-NNN.md` files with the full body.
- Open: non-record generated artifact splitting or durable justification is not
  yet proven.
- `03570fba` fixed rebalance fingerprint audit and exact old-path link repair;
  transaction-backed compensation after alias or audit write failure remains open.
- `d8cfedfb` fixed repeat-call rejection and recovery-policy hiding; budget
  suppressors and distinct mismatch events remain open.
- Open: context XML-like normal cards, ranking, and richer conflict source refs
  remain report-only findings unless docs are narrowed or source is expanded.

## Acceptance Status

- [x] Docs and code agree - commits `6a328f88` and `c2915b48`; docs state
  strict flat config and code rejects nested config.
- [x] Final gates pass - commit `c2915b48`; final command captures at
  `commands/20260709T052732Z-*`, `commands/20260709T052742Z-*`, and
  `commands/20260709T052808Z-docker-compose-run---rm-verify.out`.
- [x] Workspace records proven - commits `c2915b48`, `72fa3e2c`, and
  `8a6ea73a`; finance index evidence is under
  `tmp/agent-runs/20260709T070000Z-finance-index/`.
- [x] Diary path proven - commit `72fa3e2c`; diary record exists at
  `evidence/daily-use-data/workspace/records/life/journal/2026/07/09/entry.md`.
- [x] TUI duplicate regression proven - corrected by `97a8803d`; evidence is
  under `tmp/agent-runs/20260709T062335Z-reopen-tui/`.
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
- [x] Ordinary record turns write workspace files and rows - `72fa3e2c` and
  `8a6ea73a`; finance records now include `budget-month.md` index evidence.
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
- [x] TUI duplicate and bottom-follow regressions have tests - `c2915b48` and
  `97a8803d`; the latter includes rendered pane evidence from store rows.
- [x] Deterministic replay passes - final `smoke replay` capture exits 0.
- [x] Quiet verify passes - final `quiet verify` capture exits 0.
- [x] Docker Compose verify passes or honest skip is committed - Docker capture
  exits 0.
- [x] Live campaigns run or honest skips are committed - tracked live evidence
  under `tmp/live-runs/20260708Tstandardenv/` plus daily-use script evidence.
- [ ] Final handoff names commits, commands, evidence paths, and residual risks
  - blocked until all corrective rows close.

## Notes

- `audit_repo.py .` and `prompt_context_lint.py .` were captured with exit 1
  before final proof because they scan ignored historical tmp/data artifacts.
  The product gates that define completion passed after the last source change.
- Raw SQLite databases, raw JSON logs, and workspace manifests were left local
  when they were not needed for readable committed proof.
