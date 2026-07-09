# Evaluation Engineer Report

## Scope

- Lane packet:
  `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/evaluation-engineer.md`.
- Mode: report only. I did not edit product docs or source.
- Required files read: `docs/current-state.md`, packet `README.md`,
  `crates/lkjagent-xtask/src/benchmark/*`, `crates/lkjagent-xtask/src/proof.rs`,
  `docs/evaluation/live-proof.md`, and `docs/operations/verification.md`.

## Current Facts

- `docs/current-state.md` says the July 9 continuation reran static inspection,
  `check-docs`, `check-lines`, all package tests, smoke replay, workspace probe,
  proof collection, protocol experiments, `quiet verify`, and Docker Compose
  verification from clean `66027735`.
- Current run command captures under
  `tmp/agent-runs/20260709T051136Z-amplified-redesign/commands/` show:
  `check-docs`, `check-lines`, `check-files`, `check-style`, `quiet verify`,
  and cargo tests for app/core/store/xtask exited 0.
- Current run `quiet verify` output is exactly `ok verify`.
- Current run static packet scripts failed:
  `audit_repo.py .` exit 1 on two over-200-line historical tmp files, and
  `prompt_context_lint.py .` exit 1 on JSON-like prompt frames in historical
  tmp/data logs plus `data/logs`.
- I found no current-run command artifact for `smoke replay`, `bench
  check-corpus`, `proof collect`, `experiment protocol --all`,
  `experiment live-profiles`, or `docker compose run --rm verify` in this
  stamped run directory.
- `quiet verify` implementation in `crates/lkjagent-xtask/src/lib.rs` runs
  static gates, `benchmark::validate_corpus`, `smoke::run_replay`, then quiet
  tests before printing `ok verify`.
- `docs/operations/verification.md` says `quiet verify` checks only
  check-docs, check-lines, check-files, check-style, and quiet test.
- `docs/evaluation/benchmarks.md` says `bench check-corpus` is part of
  `quiet verify`; that matches source.
- `docs/evaluation/replay.md` says `smoke replay` is part of `quiet verify`;
  that matches source.
- `crates/lkjagent-xtask/src/benchmark` validates corpus shape and parse
  fixtures, and `bench run` drives each corpus entry through the app daemon into
  isolated entry directories under `DIR/entries/`.
- `evaluation/corpus/tiny/` currently contains ten entry directories:
  artifact-report, context-lanes, docs-tree-small, fault-truncated,
  fault-wrong-envelope, file-work, journal, meeting-pack, question, and
  workspace-daily.
- `docs/evaluation/live-proof.md` requires proof bundles to show requested
  profile, prompt/context/tool fingerprints, exchanges, admissions,
  observations, artifacts, checks, state rows, workspace records or aliases,
  and adoption notes.
- `experiment live-profiles` tests prove honest skip behavior for four profiles:
  personal-workspace, software-project, structured-artifact, and protocol-stress.
- Historical live evidence under `tmp/live-runs/20260708Tstandardenv/` records
  `elapsed_seconds=900` for personal-workspace, software-project,
  structured-artifact, and protocol-stress.
- `proof collect` currently writes summary, status, state-vector, decisions,
  prompt-frames, prompt-cards, admissions count, observations, exchanges,
  artifacts, context, context-edges, records, selector-candidates, checks,
  attempts, token-usage, workspace-tree, word-counts, and warnings.
- Candidate file line counts are within the 200-line rule:
  `proof.rs` 198, `proof_state.rs` 200, benchmark files 88/126/174 lines.

## Contradictions

- `docs/operations/verification.md` is stale for `quiet verify`; source also
  runs corpus validation and smoke replay.
- `docs/operations/proof-bundles.md` promises files that are not emitted with
  those names by `proof collect`: `cases.md` and `tool-views.md`.
- `docs/operations/proof-bundles.md` says `summary.md` records commands,
  results, anomalies, and next action, but source summary only reports table
  counts for tasks, steps, and checks.
- `docs/evaluation/live-proof.md` lists many preferred daily-use profiles, but
  current live-profile runner/test surface covers only four coarse profiles.
- Packet final gates require explicit `smoke replay` and Docker verification
  after final source change; this stamped run has `quiet verify` evidence but no
  separate current-run smoke replay or Docker command artifact.
- Packet static scripts are noisy when run at repo root because they scan
  historical tmp/data evidence; this conflicts with using them as final gates
  unless scoped or taught exclusions.

## Exact Docs Edits

- None made by this lane.
- Proposed exact edit: update `docs/operations/verification.md` `quiet verify`
  row to say: `check-docs, check-lines, check-files, check-style, bench
  check-corpus, smoke replay, quiet test`.
- Proposed exact edit: update `docs/operations/proof-bundles.md` contents to
  match emitted files, or update `proof collect` to emit the promised
  `cases.md`, `tool-views.md`, and richer command/anomaly `summary.md`.
- Proposed exact edit: update `docs/evaluation/live-proof.md` to distinguish
  implemented live profiles from preferred future/manual campaign profiles, or
  add runner profiles until the implemented set matches the preferred set.
- Proposed exact edit: add a note in evaluation docs that packet helper scripts
  must run against scoped current evidence dirs, not repo root, unless historical
  tmp/data logs are intentionally part of the lint target.

## Exact Source Edits

- None made by this lane.
- Proposed exact edit: in `crates/lkjagent-xtask/src/proof.rs`, replace count
  only `summary()` with a summary that can include command manifest, results,
  anomalies, skipped commands, and next action, or narrow the docs contract.
- Proposed exact edit: add `cases.md` output from cases/tasks rows and
  `tool-views.md` output from runtime decision/tool view rows, if keeping the
  current proof-bundle docs.
- Proposed exact edit: add live-profile definitions for diary, TODO, calendar,
  finance, note, project, artifact creation, workspace rebalance, TUI operator,
  and recovery profiles, if keeping the preferred profile list as acceptance
  rather than aspiration.
- Proposed exact edit: add xtask or script exclude rules for historical
  `tmp/coding-agent-runs`, `tmp/live-profile-data`, `tmp/probe-data-*`, and
  `data/logs` when running prompt/context lint as a current-behavior gate.

## Tests To Add Or Update

- Add an xtask test that asserts `quiet verify` includes corpus validation and
  smoke replay, or update docs-only if source behavior is intentionally broader.
- Extend `crates/lkjagent-xtask/tests/proof_collect.rs` to assert the complete
  promised file set and summary fields.
- Add a proof-collect fixture with runtime decisions that verifies tool-view
  fingerprints appear in the proof bundle.
- Add live-profile skip tests for every profile named in `docs/evaluation/live-proof.md`
  if those names remain acceptance criteria.
- Add a scoped prompt-context lint test proving generated current prompt frames
  avoid JSON-like context while historical evidence can be excluded.

## Commands To Run

- `cargo run -p lkjagent-xtask -- bench check-corpus`
- `cargo run -p lkjagent-xtask -- smoke replay`
- `cargo run -p lkjagent-xtask -- experiment protocol --all --out-dir tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/protocol-experiments-rerun`
- `cargo run -p lkjagent-xtask -- proof collect --data tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/probe-data --out tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/proof-current`
- `cargo run -p lkjagent-xtask -- experiment live-profiles --out-dir tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/live-skip --data tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/live-skip-data --duration-seconds 1 --skip-endpoint`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`
- Run packet scripts only on scoped current evidence until exclusions are fixed:
  `python3 tmp/lkjagent-yolo-amplified-thinking-packet-20260708/11-scripts/prompt_context_lint.py tmp/agent-runs/20260709T051136Z-amplified-redesign/evidence/probe-data`

## Risks

- Final acceptance could overclaim because `quiet verify` passes but separate
  final-gate artifacts for smoke replay and Docker are missing in the current
  run directory.
- Proof bundles may mislead reviewers because docs promise command/anomaly and
  tool-view detail that the collector does not fully emit.
- Live proof remains narrower than the daily-use campaign matrix; four 900s
  profiles do not prove finance, calendar, TUI operator, rebalance, or recovery
  flows.
- Repo-root packet lints can fail on historical evidence unrelated to current
  behavior, creating ambiguous gate status.

## Acceptance Items Affected

- Docs/code agreement: blocked until verification and proof-bundle docs match
  source or source grows to match docs.
- Final gates: incomplete in this stamped run until explicit smoke replay and
  Docker Compose verify artifacts exist after final source changes.
- Context lint proven: currently not proven at repo-root scope because packet
  lint fails on historical prompt-frame JSON.
- Recovery campaigns proven: partially covered by protocol experiments and tests,
  not by the full campaign matrix.
- Live campaigns or honest skip evidence: partially covered by four historical
  900s profiles and skip tests, not the full preferred profile set.
