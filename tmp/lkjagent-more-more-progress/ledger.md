# Progress Ledger

## Current Objective
Docs-first slice for workspace-first personal agent harness. Case state: objective is to align product docs with the packet target before source edits; constraints are docs as contract, files under 200 lines, no stale bridge or demo claims, XML-like non-JSON model protocol, operation-scoped tool views, evidence-only completion, and Docker-backed final verification; assumptions are that existing data/log deletions are pre-existing and must not be touched; risks are broad scope, dirty worktree, and existing bridge code; evidence requirements are docs diff, focused tests, quiet verify, Docker verify or honest skip, daily-use campaign evidence, and final acceptance matrix; candidate files are docs/current-state.md, docs/product/*, docs/workspace/*, docs/runtime/*, docs/context/*, docs/tools/*, docs/protocol/*, docs/evaluation/*, and later runtime/router/context/tool/TUI source files; next action is baseline inspection and docs-only target update.

## Docs Changed
- Commit dd97d9c2 (`docs: state workspace-first product target`) rewrote
  docs/current-state.md with workspace-first target, proven checkout, baseline
  evidence, and packet-level gaps.
- Commit dd97d9c2 updated product, workspace, runtime, context, tools,
  workbench, and evaluation docs to state transcript/inbox evidence,
  write-through records, non-JSON prompt context, operation-scoped tool views,
  recovery blockers, and TUI identity/follow requirements.

## Source Changed
- Commit 31ed049f (`feat: write owner turns to workspace evidence`) added
  workspace root scaffolding on CLI and daemon entry.
- Commit 31ed049f added owner-turn transcript and inbox trace writers, including
  send-time trace files and daemon delivery refreshes.
- Commit 31ed049f added ambiguous save-like inbox routing for `remember this` and
  `save this` forms.
- Commit 10e3d211 (`feat: scope default explore tool view`) added default
  explore views without `shell.run` and an explicit shell-capable view for
  persisted decisions.
- Commit 3803a3b0 (`feat: deduplicate prompt context items`) suppresses
  duplicate clean context items by semantic key, body, source type, and source
  fingerprint.
- Commit faec18ca (`feat: suppress json-like prompt context`) suppresses
  JSON-like context bodies with source-linked prompt markers.
- Commit ea640511 (`feat: gate artifact success on proof path`) blocks artifact-request closure until the response names the output path after file, artifact row, and check row evidence exists.
- Commit 80e080db (`feat: track tui transcript identity`) adds stable transcript
  entry ids, agent draft accumulation, id-based durable/session merge, and saved
  transcript ids/source paths.
- Commit a1ecdcdc (`feat: model workbench viewport follow state`) adds a
  `Viewport::Follow`/`Viewport::Manual` state, scroll-down follow restoration,
  manual-top preservation, and pane rendering that uses the viewport height
  helper without adding a product source file.
- Commit 660429b1 (`feat: harden action value admission`) bounds XML-like action
  argument values and rejects empty executable values before effects.
- Pending recovery ladder slice records resolved `recovery.failure` cells for
  parse, admission, effect, endpoint, and check failures.

## Tests Added
- `crates/lkjagent-core/tests/owner_turn.rs::ambiguous_save_like_turns_route_to_inbox`.
- `crates/lkjagent-app/tests/workspace_evidence.rs` covering send transcript
  traces, inbox trace without endpoint, and empty-workspace record/question
  regression.
- Extended `cli_run_once_processes_record_like_turn` to assert transcript trace.
- Added `crates/lkjagent-app/tests/tool_views.rs::default_explore_prompt_hides_shell_tool`.
- Updated contamination coverage so shell observations use an explicit shell view.
- Added `context_plan_suppresses_duplicate_clean_items` in core context tests.
- Added `crates/lkjagent-app/tests/context_no_json.rs` for JSON-like prompt
  context suppression.
- Added `crates/lkjagent-app/tests/tui_transcript_identity.rs` for streaming
  delta commit, identical text with different ids, durable override by id, and
  saved ids/source paths.
- Added workbench viewport reducer tests for scroll-down follow restoration and
  manual-top preservation after refresh.
- Added `crates/lkjagent-core/tests/tool_call_edges.rs` for Japanese values,
  large bounded values, and oversized action values.
- Added admission coverage for empty required values.
- Extended queue routing artifact tests for file, artifact/check rows, response path, and missing-path blocking.
- Added `recovery_ladder.rs` and updated admission rejection coverage for durable
  recovery failure cells.

## Commands Run
- `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs`.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify`.
- `docker compose run --rm verify` -> `ok verify`.
- `python3 tmp/lkjagent-more-more-thinking-20260708/12-scripts/repo_static_report.py .`
  -> 1087 files, 49957 lines, largest docs/current-state.md was 201 before rewrite.
- `python3 tmp/lkjagent-more-more-thinking-20260708/12-scripts/sqlite_evidence.py data/lkjagent.sqlite3`
  -> workspace_records 0, artifacts 0, check_results 0, admissions 0, observations 0.
- `cargo test -p lkjagent-core --test owner_turn` -> 7 passed.
- `cargo test -p lkjagent-app --test owner_turn_records` -> 2 passed.
- `cargo test -p lkjagent-app --test workspace_evidence` -> 3 passed.
- `cargo fmt --all -- --check` -> no output.
- `cargo test -p lkjagent-app --test diagnostics` -> 2 passed after updating
  the doctor expectation for scaffolded workspaces.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify` after the
  diagnostics test update.
- `docker compose run --rm verify` -> `ok verify` after the workspace-evidence
  source slice.
- `cargo test -p lkjagent-app --test tool_views` -> 1 passed.
- `cargo test -p lkjagent-app --test contamination` -> 2 passed.
- `cargo test -p lkjagent-app --test explore` -> 3 passed.
- `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs` after the
  tool-view slice.
- `cargo run -p lkjagent-xtask -- check-lines` -> `ok check-lines` after the
  tool-view slice.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify` after the
  tool-view slice.
- `docker compose run --rm verify` -> `ok verify` after the tool-view slice.
- `cargo test -p lkjagent-core --test context_completion` -> 4 passed.
- `cargo run -p lkjagent-xtask -- check-docs` -> `ok check-docs` after the
  context-dedup slice.
- `cargo run -p lkjagent-xtask -- check-lines` -> `ok check-lines` after the
  context-dedup slice.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify` after the
  context-dedup slice.
- `docker compose run --rm verify` -> `ok verify` after the context-dedup slice.
- `cargo test -p lkjagent-app --test tui_state` -> 11 passed after the TUI
  transcript identity slice.
- `cargo test -p lkjagent-app --test tui_snapshot` -> 1 passed after the TUI
  transcript identity slice.
- `cargo test -p lkjagent-app --test tui_transcript_identity` -> 4 passed.
- Initial `cargo run -p lkjagent-xtask -- quiet verify` for the TUI slice failed
  in the inline `tui_view` durable transcript test because the test still used
  the legacy string-only snapshot field; updated it to use transcript entries.
- `cargo test -p lkjagent-app --lib tui_view::tests::transcript_uses_durable_snapshot_agent_messages`
  -> 1 passed.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify` after the TUI
  transcript identity fix.
- `docker compose run --rm verify` -> `ok verify` after the TUI transcript
  identity fix.
- Scripted campaign command with `python` shim and `BIN='cargo run -p lkjagent-app'`
  wrote `tmp/agent-runs/20260708T110203Z/transcript.txt`; gates printed
  `ok check-docs` and `ok verify`; workspace probe printed `missing: none` and
  `workspace validate: ok`, with transcript files and one todo record.
- `cargo run -p lkjagent-xtask -- smoke live` -> `ok smoke live status=skipped
  reason=operator-command-required`; skip recorded in
  `tmp/agent-runs/20260708T110203Z/live-skip.txt`.
- Commit 88075ff6 (`test: cover workbench bottom follow`) added pane
  bottom-follow and manual-scroll growth coverage; `cargo test -p lkjagent-app
  --lib workbench_render::tests` -> 3 passed.
- `cargo test -p lkjagent-app --test workbench_viewport` -> 3 passed after
  adding viewport state and scroll-down follow restoration.
- `cargo test -p lkjagent-app --lib workbench_render::tests` -> 3 passed after
  switching pane height to the viewport helper.
- `cargo test -p lkjagent-app --lib workbench_line::tests::mode_command_switches_to_pane_renderer`
  -> 1 passed after updating the scroll-at-bottom expectation.
- `cargo run -p lkjagent-xtask -- quiet verify` -> `ok verify` after keeping the
  viewport helper inside existing source-file budget.
- `docker compose run --rm verify` -> `ok verify` after the viewport slice.
- Protocol gates: tool_call_edges 2 passed; admission 3 passed; fmt, quiet
  verify, and Docker verify passed.
- Bottom-follow gates: check-docs, check-lines, quiet verify, and Docker verify passed.
- No-JSON context: context_no_json 1 passed; docs, lines, quiet verify, and Docker verify passed.
- Artifact proof: queue_routing 7 passed, app template regression 1 passed,
  quiet verify passed, and fmt/docs/lines/Docker gates passed.
- Recovery ladder: recovery_ladder, recovery, admission_rejection, and app fake
  endpoint tests passed; quiet verify, fmt/docs/lines, and Docker gates passed.

## Failed Or Skipped Commands
- Packet scripts failed under `python`; reran with `python3`.
- Early workspace, diagnostics, viewport, protocol, and artifact-proof gates
  failed on stale expectations or guardrails; adjusted tests/code and reran.
- Combined workbench lib filter failed because cargo accepts one filter; reran
  filters separately.
- Recovery gates failed on clippy `ptr_arg`, changed admission-error behavior,
  and active recovery cells blocking work; fixed each and reran successfully.

## Evidence Files
- Packet indexed from tmp/lkjagent-more-more-thinking-20260708.
- Baseline command output is indexed in the context-mode batch from this session.

## Acceptance Matrix
- Product docs describe implemented behavior and no stale contract remains: open.
- Source files and docs respect line limits: open.
- Ordinary record turns write workspace files and rows: focused tests passed;
  broad campaign evidence still open.
- Every owner turn writes transcript or inbox evidence: focused tests passed;
  broad campaign evidence still open.
- Artifact creation creates files, artifact rows, checks, and response paths:
  focused queue routing proof passed; broad campaign proof remains open.
- Observed file-work failure has focused regression: open.
- Earlier blocked work prevents later response work without recovery: open.
- Prompt context is deduplicated, source-linked, bounded, and non-JSON:
  duplicate-context and JSON-like suppression focused tests passed; failed-output
  containment still open.
- Tool views are selected by state, not global catalog: default explore shell
  hiding has focused tests; broader state matrix remains open.
- XML-like action grammar and admission have focused tests: commit 660429b1
  records focused Japanese, multiline, entity, stale decision, context mismatch,
  bad params, oversized, empty, and placeholder test evidence.
- Recovery states handle parse, admission, effect, endpoint, and check failures:
  focused recovery ladder passed; broad campaign proof remains open.
- TUI duplicate and bottom-follow regressions have tests: transcript identity,
  duplicate-by-id, pane bottom-follow, and reducer-level re-enable-follow
  focused tests passed.
- Deterministic replay passes: open.
- Quiet verify passes: open.
- Docker Compose verify passes or honest skip committed: open.
- Live campaigns run or honest skips committed: commit 88272e70 recorded the
  scripted campaign transcript and live skip file under
  tmp/agent-runs/20260708T110203Z.
- Final handoff names commits, commands, evidence paths, and risks: open.

## Open Risks
- Existing worktree contains many deleted data/log files and untracked logs before this work.
- The request spans docs, runtime, workspace, context, tools, TUI, and evaluation.
- Baseline gates may be expensive or blocked by environment.

## Next Action
Commit recovery ladder, then run final packet/campaign acceptance checks.
