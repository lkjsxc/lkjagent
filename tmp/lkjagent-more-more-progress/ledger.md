# Lkjagent More More Progress Ledger

## Purpose

Track the workspace-first harness work, commands actually run, commits, evidence,
and unresolved acceptance items for the 2026-07-08 packet.

## Case State

Objective: make lkjagent behave as a workspace-first personal agent harness whose
ordinary owner turns leave readable workspace evidence and whose completion is
computed by state, checks, artifacts, observations, and blockers.

Constraints:

- Docs are the implementation contract and move before source changes.
- Source and docs stay at or below 200 lines per authored file.
- Runtime decisions and durable rows are the single control plane.
- Model context uses attribute-less XML-like envelopes and no JSON payloads.
- Tool views are selected from state and must not expose the global catalog.
- Completion claims require command evidence and Docker Compose verification or
  an honest committed skip.

Assumptions:

- Existing dirty `data/` changes predate this work and must not be included in
  commits unless a gate explicitly produces new evidence for this packet.
- The packet under `tmp/lkjagent-more-more-thinking-20260708/` is the acceptance
  source for this request.

Risks:

- The worktree already has many deleted `data/logs` paths and untracked runtime
  data, so commits must use explicit path staging.
- Live endpoint campaigns may be unavailable; if so, skip evidence must be
  written and committed.
- Broad gates may expose unrelated preexisting data-state failures.

Evidence requirements:

- Focused tests for workspace writes, blocked-step projection, XML action grammar,
  context hygiene, recovery states, and TUI regressions.
- Deterministic replay, quiet verify, Docker Compose verify or honest skip, and
  scripted or live daily-use campaign evidence.
- Final acceptance table filled item by item with commit ids and evidence paths.

Candidate files:

- Docs: `docs/current-state.md`, `docs/product/`, `docs/workspace/`,
  `docs/runtime/`, `docs/context/`, `docs/tools/`, `docs/protocol/`,
  `docs/evaluation/`.
- Source: runtime projection, owner routing, workspace scaffold and indexes,
  context rendering, tool catalog/admission, recovery, TUI reducer and transcript,
  xtask proof and replay.

Next action: run baseline static and deterministic gates, then make the required
docs-first product architecture commit before source edits.

## 2026-07-09 Continuation State

Start time: 2026-07-09T04:18:54Z.

Clean HEAD: `66027735`.

Current branch objective: verify the already-implemented workspace-first packet
closure against the July 9 checkout, refresh stale docs and ledger text, rerun
static inspection, focused acceptance tests, quiet verify, Docker Compose verify,
and daily-use evidence, then commit only the fresh docs or evidence delta.

Current assumptions:

- The docs-first architecture commit required by the packet exists as `a9a9fe08`.
- Source implementation commits through `66027735` must be verified before any
  further implementation is attempted.
- No source edit is justified until fresh evidence finds a failing acceptance
  item.

Current result: fresh July 9 verification passed without source edits. The only
failed commands were expected surface issues: `python` is absent, the probe
script default `BIN` adds an extra `--`, and `live-profiles --help` is not a
supported help command.

Fresh evidence root:
`tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/`.

Verification commit: `98e16b48`.

Current next action: none for this continuation branch unless the owner requests
fresh live endpoint evidence.

## 2026-07-09 Corrective Reopen

The owner reported that TUI messages still duplicated and that most packet
issues looked unimproved. Treat the earlier closure as overclaimed.

Corrected evidence:

- `97a8803d`: copied store data reproduced workbench pane transcript pollution
  with `stepdone` and `taskclosed`; final pane evidence shows only
  `owner: hello` and `agent: hello`.
- `8a6ea73a`: finance turns now create `indexes/budget-month.md` and an
  `index-budget-month` artifact row; command evidence is under
  `tmp/agent-runs/20260709T070000Z-finance-index/`.

Open after this correction:

- Workspace large-file split or durable justification remains unimplemented.
- Rebalance fingerprint continuity, link repair, and rollback or compensation
  remain unimplemented or unaudited.
- Tool admission repeat-call suppression, recovery-policy hiding, budget
  suppressors, and distinct mismatch events remain report-only findings.
- Context XML-like normal cards, ranking, and richer conflict source refs remain
  report-only findings unless docs are narrowed or source is expanded.

## Packet Read Log

- Read `docs/README.md` and `docs/current-state.md`.
- Indexed and inspected `tmp/lkjagent-more-more-thinking-20260708/`.
- Read acceptance gate files, first-90-minutes guidance, current evidence notes,
  north-star state-ledger and workspace docs, and implementation worktracks.
- On 2026-07-09, read the remaining packet sections: tool protocol, workspace
  OS, TUI workbench, subagent prompts, evaluation, scripts, templates, idea bank,
  quality manifest, and raw evidence excerpts.

## Command Evidence

| Time | Command | Result | Evidence |
| --- | --- | --- | --- |
| 2026-07-08 | `git status --porcelain=v1` via `ctx_execute` | 402 preexisting entries: 383 deleted under `data/logs`, 19 untracked under `data/` | status summary in session output |
| 2026-07-08 | `python3 tmp/lkjagent-more-more-thinking-20260708/12-scripts/repo_static_report.py .` | EXIT=0; largest file reported at 438 lines under prior tmp run evidence; product source max observed 201 in `crates/lkjagent-xtask/src/proof_state.rs` | `ctx_batch_execute` repo_static_report |
| 2026-07-08 | `python3 tmp/lkjagent-more-more-thinking-20260708/12-scripts/sqlite_evidence.py data/lkjagent.sqlite3` | EXIT=0; workspace_records=1, workspace_record_history=2, artifacts=6, check_results=0, tool_admissions=5, observations=5 | `ctx_batch_execute` sqlite_evidence |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- check-docs` | ok check-docs; EXIT=0 | `ctx_batch_execute` check_docs |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- quiet verify` | ok verify; EXIT=0 | `ctx_batch_execute` quiet_verify |
| 2026-07-08 | `docker compose run --rm verify` | ok verify; EXIT=0 | `ctx_batch_execute` docker_compose_verify |
| 2026-07-08 | post-docs `cargo run -p lkjagent-xtask -- check-docs` | failed; banned token `legacy` in `docs/product/README.md`; fixed wording | `ctx_batch_execute` post_docs_check_docs |
| 2026-07-08 | post-docs retry `cargo run -p lkjagent-xtask -- check-docs` | failed; banned token `compatibility` in `docs/product/README.md`; fixed wording | `ctx_batch_execute` post_docs_check_docs_retry |
| 2026-07-08 | post-docs final `cargo run -p lkjagent-xtask -- check-docs` | ok check-docs; EXIT=0 | `ctx_execute` post-docs check-docs final |
| 2026-07-08 | `cargo test -p lkjagent-core --test completion_safety` | first run failed `blocked_file_work_with_later_pending_response_does_not_continue`; after adding preflight in `next_work_rendered`, retry passed 11 tests; EXIT=0 | `ctx_batch_execute` core_completion_safety_tests and retry |
| 2026-07-08 | `cargo test -p lkjagent-app runtime_cell::tests::blocked_plan_with_later_pending_projects_completion_blocked` | passed focused projection regression; EXIT=0 | `ctx_batch_execute` app_runtime_cell_unit_tests_retry |
| 2026-07-08 | runtime slice `cargo run -p lkjagent-xtask -- quiet verify` | first failed on `.expect(` in `runtime_cell.rs`; fixed test helper | `ctx_batch_execute` runtime_slice_quiet_verify |
| 2026-07-08 | runtime slice `cargo run -p lkjagent-xtask -- quiet verify` | then failed `docs_tree_dangling_link_materializes_revise_then_closes`; adjusted skipped-verify preflight to allow repair and superseding verify targets | `ctx_batch_execute` runtime_slice_quiet_verify_post_fmt and target_supersede |
| 2026-07-08 | `cargo test -p lkjagent-app --test docs_tree docs_tree_dangling_link_materializes_revise_then_closes` | passed after repair-target allowance; EXIT=0 | `ctx_batch_execute` docs_tree_regression_repair_allow |
| 2026-07-08 | final runtime slice focused tests | `cargo test -p lkjagent-core --test completion_safety` passed 11 tests; app runtime cell focused test passed; EXIT=0 | `ctx_batch_execute` runtime_slice_focused_tests_repair_allow |
| 2026-07-08 | runtime slice `cargo run -p lkjagent-xtask -- quiet verify` | ok verify; EXIT=0 | `ctx_batch_execute` runtime_slice_quiet_verify_repair_allow |
| 2026-07-08 | runtime slice `cargo run -p lkjagent-xtask -- check-docs` | ok check-docs; EXIT=0 | `ctx_batch_execute` runtime_slice_check_docs_final |
| 2026-07-08 | runtime slice `docker compose run --rm verify` | ok verify; EXIT=0 | `ctx_batch_execute` runtime_slice_docker_verify |
| 2026-07-08 | `bash tmp/lkjagent-more-more-thinking-20260708/12-scripts/run_campaigns.sh` | failed first because `python` was absent; failed second because default `BIN` added a duplicate `--`; reran with `python3` shim and `BIN="cargo run -p lkjagent-app"` | transcripts under `tmp/agent-runs/20260708T180102Z`, `20260708T180110Z`, `20260708T180141Z` |
| 2026-07-08 | campaign evidence assertions | `ok check-docs`, `ok verify`, records=1, artifacts=6, workspace validate ok, transcript and index files present; EXIT=0 | `tmp/agent-runs/20260708T180141Z/transcript.txt` and `tmp/probe-data-20260708T180141Z/` |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- experiment live-profiles --skip-endpoint ...` | wrote honest endpoint skip profiles; EXIT=0 | `tmp/live-runs/20260708T180320Z/` |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- experiment protocol --all ...` | wrote protocol experiment matrix; EXIT=0 | `tmp/agent-runs/20260708T180141Z/protocol-experiments/` |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- proof collect --data tmp/probe-data-20260708T180141Z ...` | wrote proof bundle with state, records, checks, artifacts, prompt, decisions, tool admissions, observations, workspace tree, and word counts; EXIT=0 | `tmp/agent-runs/20260708T180141Z/proof/` |
| 2026-07-08 | focused workspace and artifact tests | owner_turn_records 2 passed, queue_routing 7 passed, journal_artifact 2 passed; all EXIT=0 | `ctx_batch_execute` focused_workspace_artifact_tests |
| 2026-07-08 | focused context/tool/recovery/TUI tests | one wrong target `recovery_states` failed with no such test; corrected targets passed: context, no-JSON, contamination, tool views, XML parser/admission, recovery, effect, endpoint, admission rejection, TUI identity, viewport | `ctx_batch_execute` focused_context_tool_recovery_tests, focused_recovery_tests_correct, focused_tui_tests |
| 2026-07-08 | `cargo run -p lkjagent-xtask -- smoke replay` | ok smoke replay; EXIT=0 | `tmp/smoke-replay-data` |
| 2026-07-08 | final line counts | `docs/current-state.md` 136 lines; ledger 115 lines; EXIT=0 | `ctx_batch_execute` final_line_counts |
| 2026-07-08 | final `cargo run -p lkjagent-xtask -- check-docs` | ok check-docs; EXIT=0 | `ctx_batch_execute` final_check_docs |
| 2026-07-08 | final `cargo run -p lkjagent-xtask -- quiet verify` | ok verify; EXIT=0 | `ctx_batch_execute` final_quiet_verify |
| 2026-07-08 | final `docker compose run --rm verify` | ok verify; EXIT=0 | `ctx_batch_execute` final_docker_verify |
| 2026-07-08 | `.env` inventory | `.env` exists with LKJAGENT_API_KEY, LKJAGENT_ENDPOINT_URL, LKJAGENT_MODEL, and LKJAGENT_CONTEXT_LENGTH present; values were not printed | `ctx_batch_execute` safe_env_inventory |
| 2026-07-08 | live endpoint run with `.env` | `cargo run -p lkjagent-xtask -- experiment live-profiles --out-dir tmp/live-runs/20260708Tliveenv --data tmp/live-profile-data/20260708Tliveenv --duration-seconds 30`; EXIT=0 | `ctx_execute` live profile run using env |
| 2026-07-08 | live endpoint summary | personal-workspace closed, structured-artifact closed, software-project open, protocol-stress open; metrics recorded without secret values | `tmp/live-runs/20260708Tliveenv/*/summary.md` |
| 2026-07-08 | data cleanup | `sudo rm -rf data && git restore --source=HEAD --worktree -- data`; git status then reported 0 entries | `ctx_execute` sudo wipe data and restore tracked state |
| 2026-07-08 | post-live line counts | `docs/current-state.md` 138 lines; ledger 123 lines; EXIT=0 | `ctx_batch_execute` post_live_line_counts |
| 2026-07-08 | post-live `cargo run -p lkjagent-xtask -- check-docs` | ok check-docs; EXIT=0 | `ctx_batch_execute` post_live_check_docs |
| 2026-07-08 | post-live `cargo run -p lkjagent-xtask -- quiet verify` | ok verify; EXIT=0 | `ctx_batch_execute` post_live_quiet_verify |
| 2026-07-08 | post-live `docker compose run --rm verify` | ok verify; EXIT=0 | `ctx_execute` post live docker verify |
| 2026-07-08 | redesign packet focused scripts | fmt, prompt-no-json, protocol grep, TUI capture, workspace evidence, compose config, focused core/app tests all EXIT=0 | `tmp/coding-agent-runs/20260708T183000Z-workspace-runtime-redesign/final/summary.log` |
| 2026-07-08 | redesign packet final gates | check-docs, quiet verify, and docker compose verify all EXIT=0 | `tmp/coding-agent-runs/20260708T183000Z-workspace-runtime-redesign/final/summary.log` |
| 2026-07-08 | standard `.env` live profiles | 4 profiles ran 900 seconds each and closed; EXIT=0; elapsed_seconds=3605 total | `tmp/live-runs/20260708Tstandardenv/` |
| 2026-07-09 | static and docs gates | `python` failed absent; `python3` static and SQLite scripts passed; check-docs and check-lines passed | `tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/baseline/` |
| 2026-07-09 | focused acceptance tests | `cargo test -p lkjagent-core --tests` and `cargo test -p lkjagent-app --tests` passed | `tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/focused/` |
| 2026-07-09 | replay, probe, proof, protocol | smoke replay passed; workspace probe passed with python shim; proof bundle and protocol experiments written | `tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/` |
| 2026-07-09 | final deterministic gates | `cargo run -p lkjagent-xtask -- quiet verify` and `docker compose run --rm verify` passed | `tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/final/` |

## Acceptance Ledger

| Item | Commit | Evidence | Status |
| --- | --- | --- | --- |
| docs synced | a9a9fe08; 9cd8069c | check-docs passed after docs-first edits and after final current-state update | done |
| workspace writes | 9cd8069c; 8a6ea73a | owner_turn_records passed; finance index evidence includes `budget-month.md` and `index-budget-month` row | partial |
| owner transcript | 9cd8069c | owner_turn_records and campaign transcript files `workspace/artifacts/transcripts/queue-000001.md` through `queue-000003.md` | done |
| artifact flow | 9cd8069c | queue_routing artifact tests passed; journal_artifact passed; proof artifacts bundle captured | done |
| blocked-step guard | 2dc0c841 | completion_safety passed; runtime_cell focused test passed; docs-tree repair regression passed; quiet and Docker verify passed | done |
| context dedupe | 9cd8069c | context_items, context_no_json, contamination, and proof context outputs passed/captured | done |
| tool view scope | 9cd8069c | tool_views, runtime_selector, render_tool_cards, and protocol experiment matrix passed/captured | done |
| XML action grammar | 9cd8069c | tool_call and protocol experiment matrix passed/captured | done |
| recovery states | 9cd8069c | recovery, recovery_ladder, effect_error, endpoint, and admission_rejection tests passed | done |
| TUI duplicate fix | 9cd8069c; 97a8803d | rendered pane evidence from copied store rows shows diagnostics no longer enter transcript pane | partial |
| TUI follow fix | 9cd8069c | workbench_viewport passed; scroll down to bottom re-enables follow and refresh preserves manual top line | done |
| deterministic replay | 9cd8069c | `cargo run -p lkjagent-xtask -- smoke replay` and quiet verify smoke replay passed | done |
| quiet verify | 9cd8069c | final `cargo run -p lkjagent-xtask -- quiet verify` passed after source changes | done |
| Docker verify | 9cd8069c | final `docker compose run --rm verify` passed after source changes | done |
| live campaigns | 9cd8069c; 6d0be7d8; 44a791ec | scripted campaign passed with workspace validation; skip files committed; standard `.env` endpoint run added under `tmp/live-runs/20260708Tstandardenv/` with all four profiles closed after 900-second time boxes | done |
| large file split | none | no source proof found in corrective audit | open |
| rebalance safety | none | fingerprint continuity, link repair, and rollback or compensation remain report-only | open |
| tool admission hardening | none | repeat-call, recovery-policy, budget, and mismatch-event claims remain report-only | open |
| context contract gaps | none | XML-like normal cards, ranking, and richer conflict refs remain report-only | open |
