# Docs Cartographer Report

## Scope

- Packet lane: `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/docs-cartographer.md`.
- Output scope from owner: report only; no product docs or source edits.
- Candidate docs inspected: `docs/README.md`, `docs/current-state.md`,
  `docs/product/daemon.md`, `docs/workspace/filesystem-layout.md`,
  `docs/context/prompt-assembly.md`,
  `docs/tools/toolset-view-and-admission.md`, `docs/product/workbench.md`.

## Current Facts

- `docs/current-state.md` is current enough to include July 9 proof claims,
  record-family paths, XML action parsing, state-selected tool views, recovery
  cells, artifact response-path gating, workspace transcripts/inbox traces, and
  workbench transcript/renderer evidence.
- `docs/product/daemon.md` says one cycle writes transcript or inbox evidence,
  classifies owner turns, persists one `RuntimeDecision`, calls the endpoint
  only when needed, and keeps unsupported system operations blocked with
  evidence.
- `crates/lkjagent-app/src/daemon_owner_routes.rs` writes owner-turn traces to
  `artifacts/transcripts/queue-NNNNNN.md` or `inbox/queue-NNNNNN.md`.
- `docs/product/daemon.md` says record-only turns may bypass the endpoint and
  write structured records, metadata, indexes, route evidence, and state events.
- `crates/lkjagent-core/src/workspace_record_paths.rs` implements dated journal
  paths, todo state paths, dated calendar paths, month finance paths, title
  slugged project/development paths, artifact/proof paths, and undated fallback.
- `crates/lkjagent-app/tests/owner_turn_records.rs`,
  `record_wrappers.rs`, and `workspace_evidence.rs` assert journal `entry.md`
  paths, no `unix:` path content, README/index artifacts, fingerprints, and
  transcript traces.
- `docs/context/prompt-assembly.md` specifies dedupe, context lanes,
  no JSON-like prompt bodies, bounded recovery, lane fingerprints, and
  context-frame fingerprint propagation.
- `crates/lkjagent-core/src/runtime_context_plan.rs` dedupes by semantic key,
  body, source type, and source fingerprint, excludes stale/contaminated/conflict
  items, and builds `relevant-records` and `excluded-context-notes` lanes.
- `crates/lkjagent-app/src/context_bridge.rs` suppresses JSON-like context
  bodies with a source-linked marker before prompt rendering.
- `docs/tools/toolset-view-and-admission.md` specifies view-selected tools,
  deterministic tool-view fingerprints, XML action grammar, and rejection before
  effects.
- `crates/lkjagent-core/src/runtime_tool_catalog.rs` makes default explore
  exclude `shell.run`; `shell_tool_view()` is explicit.
- `crates/lkjagent-core/src/runtime_tool_call.rs` and tests reject attributes,
  JSON-shaped action bodies, duplicate scalars, stale decisions, context
  mismatches, unknown tools/args, bad primitive classes, crossed tags, and bad
  entities.
- `crates/lkjagent-core/src/runtime_admission.rs` rejects tools absent from the
  decision view, missing/unknown params, placeholders, empty values, invalid
  counts, and workspace path escapes.
- `docs/product/workbench.md` specifies append/pane modes, durable transcript
  identity, reducer purity, Japanese/mixed-width editing, scroll/follow behavior,
  and authority limits.
- `crates/lkjagent-app/src/tui_reduce.rs`, `tui_render.rs`,
  `workbench_state.rs`, and `workbench_render.rs` implement grapheme cursor
  handling, display-width cursor placement, follow/manual scroll, append/pane
  rendering, and status/transcript grouping.

## Contradictions

- `tmp/.../01-current-snapshot/summary.md` says journal paths still use generic
  record ids and Unix-like timestamps; current source/tests now prove dated
  journal paths and no `unix:` path strings. Treat the packet snapshot as stale
  evidence, not current truth.
- `tmp/.../02-product-contract/docs-first-map.md` asks
  `docs/workspace/filesystem-layout.md` to add dated journal, calendar, finance,
  transcript, and project paths. Current source implements them, but this
  candidate doc still lists only family directories and source rules, so it is
  less precise than implementation.
- `tmp/.../02-product-contract/docs-first-map.md` asks
  `docs/product/daemon.md` to clarify owner-authorized maintenance versus true
  idle waiting. Current daemon doc defines true idle as no endpoint calls or
  self-assigned work, but does not name an owner-authorized maintenance route.
- `tmp/.../02-product-contract/docs-first-map.md` asks
  `docs/product/workbench.md` to specify debug hiding. Current workbench doc
  names bounded sections and authority limits, but does not explicitly say debug
  internals/raw logs are hidden unless selected by row-backed commands.
- `docs/context/prompt-assembly.md` says workspace record and index context is
  rendered as bounded metadata, not full file bodies. Current
  `context_bridge.rs` renders `item.body` for non-JSON clean context items; this
  may be acceptable for already-bounded context rows, but the doc should state
  the row body must already be bounded/summarized before admission.
- `docs/workspace/filesystem-layout.md` says each major owner-facing workspace
  directory README explains purpose, record shape, allowed actions, source rules,
  and index behavior. Current `workspace_scaffold.rs` writes generic READMEs
  with title, purpose, and child links only.

## Exact Docs Edits

- Edit `docs/workspace/filesystem-layout.md`: add a `## Canonical Paths`
  section naming:
  `records/life/journal/YYYY/MM/DD/entry.md`,
  `records/life/todo/<state>/<id>.md`,
  `records/life/calendar/YYYY/MM/DD/<id>.md`,
  `records/life/finance/YYYY/MM/<id>.md`,
  `records/work/projects/<title-slug>/<id>.md`,
  `records/work/development/<title-slug>/<id>.md`,
  `artifacts/transcripts/queue-NNNNNN.md`, and
  `inbox/queue-NNNNNN.md`.
- Edit `docs/workspace/filesystem-layout.md`: revise `Workspace READMEs` to
  either match the generic generated README contract or require source changes
  that generate the richer per-directory text.
- Edit `docs/product/daemon.md`: add an `Owner-Authorized Maintenance` section
  saying maintenance is active only when selected by an owner turn or persisted
  runtime decision, writes evidence rows, and is not idle self-assignment.
- Edit `docs/product/workbench.md`: add that pane/append views hide raw debug
  dumps, contaminated model output, and private prompt/provider bodies unless a
  row-backed inspection command explicitly selects bounded refs.
- Edit `docs/context/prompt-assembly.md`: clarify that record/index context rows
  admitted to the prompt must already contain bounded metadata or summaries;
  prompt rendering does not read arbitrary full workspace files.
- Edit `docs/current-state.md` after any implementation/doc sync change with
  the exact proven behavior, gates run, and remaining gaps.

## Exact Source Edits

- None made in this lane.
- If choosing to honor the existing `Workspace READMEs` wording literally,
  update `crates/lkjagent-app/src/workspace_scaffold.rs` so generated READMEs
  include purpose, record shape where applicable, allowed agent actions,
  source-of-truth rules, and index behavior.
- If keeping current generic README generation, no source edit is required;
  instead narrow `docs/workspace/filesystem-layout.md` to match it.
- If enforcing debug hiding beyond current bounded rendering, update
  `crates/lkjagent-app/src/workbench_render.rs` and snapshot/status selection so
  raw prompt/provider/debug bodies cannot appear in append/pane output.

## Tests To Add Or Update

- Add/update workspace scaffold tests for generated README content if the richer
  README contract is kept.
- Add a docs/source sync test for canonical record paths listed in
  `docs/workspace/filesystem-layout.md` against `workspace_record_paths.rs`.
- Add/update workbench renderer tests proving raw debug/provider/prompt bodies
  are absent while bounded status refs remain visible.
- Add a context test proving prompt-admitted workspace record/index context is
  bounded metadata/summary, not arbitrary full file contents.

## Commands To Run

- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- `cargo run -p lkjagent-xtask -- check-files`
- `cargo run -p lkjagent-xtask -- check-style`
- `cargo test -p lkjagent-core workspace_record`
- `cargo test -p lkjagent-app owner_turn_records record_wrappers workspace_evidence`
- `cargo test -p lkjagent-app workbench_viewport tui_state tui_transcript_identity`
- `cargo test -p lkjagent-app context_no_json prompt_frame`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`

## Risks

- Updating docs without matching scaffold behavior would preserve a hidden
  docs/source disagreement around workspace READMEs.
- Treating packet snapshot claims as current could reintroduce already-fixed
  path behavior.
- Adding maintenance language too broadly could violate the single-control-plane
  and no-hidden-idle-work contract.
- Workbench debug-hiding must not hide row-backed status evidence needed for
  honest operation.

## Acceptance Items Affected

- Docs sync gates: current-state freshness, stale demo-contract removal, file
  line limits, and docs/source agreement.
- Workspace gates: canonical record paths, transcript/inbox traces, README/index
  behavior, and workspace evidence rows.
- Context gates: no duplicated/stale/JSON-like/contradictory prompt context and
  bounded record/index context.
- Tool gates: prompt-visible and admission-visible tools remain identical.
- TUI gates: transcript identity, order, scroll/follow behavior, mixed-width
  input, and debug/output cleanliness.
- Final gates: all xtask checks, targeted tests, `quiet verify`, and Docker
  Compose verification after the last implementation change.

## Verification Performed For This Report

- Read required packet files and all candidate docs.
- Inspected source/test hotspots for daemon routing, workspace paths/indexes,
  prompt context planning/rendering, tool views/admission, and workbench/TUI.
- Ran `wc -l` on candidate docs; all inspected candidate docs are under 200
  lines.
- Did not run product gates because this lane was scoped to report-only and made
  no product docs or source changes.
