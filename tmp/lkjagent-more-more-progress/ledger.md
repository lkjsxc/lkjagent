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

## Packet Read Log

- Read `docs/README.md` and `docs/current-state.md`.
- Indexed and inspected `tmp/lkjagent-more-more-thinking-20260708/`.
- Read acceptance gate files, first-90-minutes guidance, current evidence notes,
  north-star state-ledger and workspace docs, and implementation worktracks.

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

## Acceptance Ledger

| Item | Commit | Evidence | Status |
| --- | --- | --- | --- |
| docs synced | a9a9fe08; pending evidence commit | check-docs passed after docs-first edits and after final current-state update | done |
| workspace writes | pending evidence commit | owner_turn_records passed; campaign workspace probe records=1 and validation ok | done |
| owner transcript | pending evidence commit | owner_turn_records and campaign transcript files `workspace/artifacts/transcripts/queue-000001.md` through `queue-000003.md` | done |
| artifact flow | pending evidence commit | queue_routing artifact tests passed; journal_artifact passed; proof artifacts bundle captured | done |
| blocked-step guard | 2dc0c841 | completion_safety passed; runtime_cell focused test passed; docs-tree repair regression passed; quiet and Docker verify passed | done |
| context dedupe | pending evidence commit | context_items, context_no_json, contamination, and proof context outputs passed/captured | done |
| tool view scope | pending evidence commit | tool_views, runtime_selector, render_tool_cards, and protocol experiment matrix passed/captured | done |
| XML action grammar | pending evidence commit | tool_call and protocol experiment matrix passed/captured | done |
| recovery states | pending evidence commit | recovery, recovery_ladder, effect_error, endpoint, and admission_rejection tests passed | done |
| TUI duplicate fix | pending evidence commit | tui_transcript_identity passed; identical text with different ids remains visible and durable row overrides matching ephemeral id | done |
| TUI follow fix | pending evidence commit | workbench_viewport passed; scroll down to bottom re-enables follow and refresh preserves manual top line | done |
| deterministic replay | pending evidence commit | `cargo run -p lkjagent-xtask -- smoke replay` and quiet verify smoke replay passed | done |
| quiet verify | pending evidence commit | final `cargo run -p lkjagent-xtask -- quiet verify` passed after source changes | done |
| Docker verify | pending evidence commit | final `docker compose run --rm verify` passed after source changes | done |
| live campaigns | pending evidence commit | scripted campaign passed with workspace validation; live profiles wrote honest endpoint skip files | done |
