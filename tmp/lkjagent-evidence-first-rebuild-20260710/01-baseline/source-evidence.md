# Source Evidence

## Runtime Authority

- app/src/daemon.rs selects from a TaskSnapshot, calls context preparation
  before decision creation, and executes one legacy Work value.
- app/src/runtime_cell.rs projects the next task step into a state cell.
- app/src/runtime_projection.rs labels that projection as plan bridge.
- core/src/runtime_transition.rs defines typed transitions but has no production
  app caller.
- core/src/engine.rs and engine_steps.rs still mark task steps and summaries.

## Premature Completion

- core/src/engine_steps.rs special-cases finish before normal explored-action
  admission and marks any parsed response message done.
- core/src/engine_completion.rs permits generic closure with no objective-level
  checks.
- app/src/daemon_intake.rs represents no work as a closed synthetic task.

## Recovery

- app/src/recovery_bridge.rs counts prior same-kind failures without proving
  strategy lineage.
- core/src/engine.rs maps recovery resolution to a no-op.
- current data/logs matter two contains two identical output-limit failures.

## Context And Tools

- app/src/context_bridge.rs inserts workspace context before state decision and
  renders all selected clean items.
- core/src/runtime_context_plan.rs records ranks and lane budgets without using
  them to sort or spend tokens.
- app/src/runtime_cell.rs assigns one broad exploration tool view.
- core/src/runtime_tool_cards.rs renders the first tool skeleton, which can be
  finish.
- app/src/explore.rs can convert a failed native tool dispatch into an
  observation while the attempt remains successful, allowing a recovery card
  with a success diagnosis.
- admission and observation ordinals are derived from different command
  positions, so an observation can reference no existing admission.

## Workspace

- core/src/owner_record.rs emits canned diary content.
- app/src/record_files.rs writes files, rows, state, and indexes across separate
  operations.
- effects/src/workspace.rs mixes canonical and original relative roots.
- store/src/context_rows.rs selects a small recent metadata window, not relevant
  bodies.
- app/src/workspace_scaffold.rs eagerly creates generic navigation.

## TUI And Live Proof

- app/src/tui_snapshot.rs merges queue and selected task events into messages.
- app/src/tui_transcript.rs can deduplicate IDs, not duplicate logical rows with
  different source IDs.
- app/src/tui_reduce.rs scrolls logical lines while ratatui wraps visual rows.
- xtask/src/experiment_live.rs runs for wall time, overwrites final state with
  synthetic idle, and treats no outer error as ran.
