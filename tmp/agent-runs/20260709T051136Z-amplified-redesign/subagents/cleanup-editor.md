# Cleanup Editor Report

## Scope

Report-only lane from
`tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/cleanup-editor.md`.
I read `docs/current-state.md`, the packet README, `README.md`, `AGENTS.md`,
`docs/decisions/*`, `docs/engine/*`, candidate source anchors, existing run
evidence in this stamped directory, and sibling subagent reports. I did not edit
product docs or source.

## Current Facts

- `docs/current-state.md` now states the workspace-first contract, July 9
  continuation evidence, record-family paths, XML action parsing, model-free
  workspace effects, state-selected tool views, artifact response-path gating,
  recovery cells, and workbench coverage.
- The Rust workspace currently includes `lkjagent-core`, `lkjagent-store`,
  `lkjagent-llm`, `lkjagent-effects`, `lkjagent-app`, and `lkjagent-xtask`.
- `crates/lkjagent-core/src/workspace_record_paths.rs` implements dated journal
  paths, state-grouped TODO paths, dated calendar paths, month finance paths,
  work/project paths, artifact/proof paths, and unknown-kind note paths.
- `crates/lkjagent-app/tests/owner_turn_records.rs` proves record-like owner
  turns write workspace files without creating tasks, including journal
  `entry.md` paths and no `unix:` in paths.
- `crates/lkjagent-app/tests/record_wrappers.rs` proves friendly wrappers write
  generic records for todo, development, finance, calendar, note, and project.
- `crates/lkjagent-store/src/plan_schema.rs` still creates and uses bridge
  `tasks`, `steps`, `attempts`, `check_results`, `events`, `memory`, and
  state-ledger tables, so plan-family bridge language is not purely stale.
- `crates/lkjagent-app/src/runtime_projection.rs` still projects bridge snapshots
  into state cells with source `plan-bridge`.
- `crates/lkjagent-app/src/snapshot_state.rs` still persists
  `matter:snapshot/<id>` cells from task snapshots.
- `README.md` and `docs/engine/*` still use matter and bridge terminology;
  source supports that terminology today, but owner-facing docs should not make
  bridge rows sound like the product center.
- Existing run artifacts in this stamp show `check-docs`, `check-lines`,
  `check-files`, `check-style`, `quiet verify`, core/app/store/xtask tests,
  protocol experiment, smoke replay, workspace shape, SQLite sanitize, and proof
  collection passed.
- Existing run artifacts also show packet static scripts
  `audit_repo.py` and `prompt_context_lint.py` failed against historical
  `tmp/` and `data/logs` artifacts, not necessarily product source.
- `docker compose run --rm verify` is required by the packet final gates, but I
  did not find a Docker Compose output in this stamped directory.

## Contradictions

- `README.md:41-43` calls listed commands "target owner and developer surfaces"
  and says transitional bridge commands may still read plan-family rows until no
  longer needed. The commands are implemented now, and the sentence mixes stable
  user docs with implementation-transition language.
- `docs/engine/README.md:5-6` defines the engine docs as "transitional
  plan-family helpers". Current source still has a bridge, but this top-level
  purpose underplays the durable state-ledger runtime contract.
- `docs/engine/plan-and-steps.md:9-12` says "The current checkout stores bridge
  plans as ordered row bodies. The target keeps..." This target/current split is
  confusing after `docs/current-state.md` says state-ledger decisions and
  blockers are already proven.
- `docs/engine/matter-bridge.md:32-36` says new features should write semantic
  rows directly while bridge code remains for continuity. Source still creates
  bridge tasks for artifact requests, inspection, system-operation blocks, and
  generic matters, so the removal-direction wording is aspirational.
- `docs/engine/templates/README.md:5` calls templates transitional; source still
  uses templates and tests them, so "transitional" is stale unless paired with a
  precise bridge boundary.
- `docs/decisions/personal-as-templates.md:15-16` says personal work uses
  generic Markdown records under `workspace/records/`; current paths are more
  specific, e.g. `records/life/journal/YYYY/MM/DD/entry.md` and
  `records/work/projects/<slug>/<id>.md`.
- `docs/current-state.md:129-136` labels "Known Gaps" but lists mostly evidence
  scope and adoption decisions. Rename or split into "Evidence Limits" plus true
  open gaps to avoid making proven behavior look incomplete.
- Packet snapshot `01-current-snapshot/summary.md` says journal paths still use
  generic ids and Unix-like timestamps; current source/tests supersede that.
- Existing `prompt_context_lint.py` output flags JSON-like `prompt-frame.json`
  audit files. This conflicts with docs only if the no-JSON rule is interpreted
  as applying to audit artifacts instead of normal model-visible context.

## Exact Docs Edits

- `README.md:41-44`: replace with:
  "The commands above are current owner and developer surfaces. Some developer
  proof and inspection paths still read bridge rows, but owner-facing behavior is
  governed by durable state rows, workspace files, and persisted
  `RuntimeDecision` rows. `docs/current-state.md` is the evidence ledger."
- `docs/engine/README.md:5-6`: replace purpose text with:
  "Define how ordered work, bridge rows, runtime decisions, checks, and templates
  cooperate inside the state-ledger runtime."
- `docs/engine/plan-and-steps.md:9-12`: replace target/current split with:
  "Ordered artifact work is represented by bridge plan rows projected into
  `plan:*`, `work:*`, `check:*`, and completion state cells. Exactly one
  runnable operation may be selected by a persisted runtime decision while other
  state cells remain active."
- `docs/engine/matter-bridge.md:32-36`: change heading to
  `## Boundary` and say bridge rows are current compatibility storage for ordered
  work, while new owner-visible contracts must be stated as matters, records,
  artifacts, decisions, events, and proof.
- `docs/engine/templates/README.md:5`: replace "transitional templates" with
  "pure templates".
- `docs/decisions/personal-as-templates.md:15-18`: replace generic path wording
  with canonical families: journal, todo, calendar, finance, notes, routines,
  contacts, work projects/development, knowledge refs/notes, artifacts, and
  proof.
- `docs/current-state.md:129`: rename `## Known Gaps` to
  `## Evidence Limits And Open Gaps`; split adoption and operator-session limits
  from implemented workspace evidence.
- `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/01-current-snapshot/summary.md`:
  do not edit packet history; downstream reports should cite it as stale snapshot
  evidence superseded by current checkout tests.

## Exact Source Edits

- None made in this lane.
- No source edit is required for the wording cleanup above if docs are narrowed
  to current source behavior.
- If the desired contract is "no bridge rows for new owner-visible flows", then
  source edits are larger than cleanup: replace bridge task creation in
  `daemon_route_effects.rs`, `classify.rs`, and `snapshot_state.rs` paths with
  direct semantic rows and add compatibility readers. That is not a cleanup-only
  slice.
- If the no-JSON rule is intended to include audit `prompt-frame.json` files,
  change `prompt_bridge.rs` artifact format and proof tooling; otherwise update
  docs/scripts to exclude audit artifacts.

## Tests To Add Or Update

- Add a docs/source sync test that extracts canonical record paths from
  `docs/decisions/personal-as-templates.md` and compares them with
  `workspace_record_paths.rs` examples.
- Add a docs wording lint that rejects "target", "transitional", or "until the
  implementation" in stable top-level docs except in explicitly historical or
  bridge-boundary sections.
- Add a test or xtask check that `docs/engine/README.md` does not describe the
  engine as only plan-family helpers.
- If scripts remain acceptance gates, update `prompt_context_lint.py` to ignore
  audit/proof JSON or make it scan only model-visible prompt text.

## Commands To Run

- After doc cleanup: `cargo run -p lkjagent-xtask -- check-docs`
- After doc cleanup: `cargo run -p lkjagent-xtask -- check-lines`
- After doc cleanup: `cargo run -p lkjagent-xtask -- check-files`
- After doc cleanup: `cargo run -p lkjagent-xtask -- check-style`
- For confidence that wording did not drift from source:
  `cargo test -p lkjagent-core --test workspace_record --test owner_turn`
- For app behavior anchors:
  `cargo test -p lkjagent-app --test owner_turn_records --test record_wrappers --test queue_routing`
- Before claiming packet completion: `cargo run -p lkjagent-xtask -- quiet verify`
- Required final gate before completion claim: `docker compose run --rm verify`

## Risks

- Removing all bridge wording would be inaccurate because bridge tables and
  projection code still exist and are exercised.
- Leaving target/transitional wording in root docs makes implemented behavior
  look speculative and weakens the docs-as-contract rule.
- Tightening packet scripts without excluding historical `tmp/` evidence may
  turn useful proof archives into false cleanup blockers.
- Rewriting engine docs too broadly could hide a real architectural boundary:
  bridge storage is still compatibility machinery, not a second control plane.
- Renaming known gaps without updating acceptance ledgers may make evidence
  limits harder to track.

## Acceptance Items Affected

- Docs and code agree: affected by stale target/transitional wording.
- Final gates pass: affected because doc edits require check-docs, check-lines,
  check-files, check-style, quiet verify, and Docker Compose verification.
- Workspace records proven: wording should cite current canonical paths and
  tests, not stale packet snapshot claims.
- Diary path proven: current source/tests prove it; docs should stop implying
  generic id/timestamp paths.
- Context lint proven: script failure against audit JSON needs contract
  clarification or script scoping.
- Tool view parity, recovery campaigns, and TUI gates: no direct cleanup edits
  identified in this candidate set, but final current-state wording should not
  overclaim beyond sibling lane findings.
- Live campaigns or honest skip evidence: current-state already states standard
  endpoint evidence and adoption limits; keep this as evidence/adoption wording,
  not an implementation gap.
