# Active Mode Controller

## Purpose

Own the work that makes one runtime mode determine model-visible tool policy,
recovery policy, compaction behavior, maintenance behavior, and completion
semantics at every endpoint turn.

## Owner Evidence

Uploaded run logs show maintenance and graph policy contradicting each other,
compaction requiring `memory.save` while graph policy refuses it, recovery
blocking the tool needed to escape, completion closing without the requested
artifact, and the active long-novel log churning through no-op maintenance
cycles before owner work arrives.

## Failure Modes

- More than one policy layer renders as active.
- Maintenance continues after owner work is queued.
- Recovery suggests a blocked tool.
- Compaction asks the model to preserve state through a blocked tool.
- Closed idle still causes endpoint churn.

## Mode Model

The only active modes are `OwnerTask`, `Recovery`, `Maintenance`,
`Compaction`, and `ClosedIdle`. At every endpoint turn exactly one active mode
owns allowed tools, blocked tools, preferred action, valid examples, and
completion semantics.

## Stable Contract

The stable architecture contract is
[../../architecture/runtime/active-mode/](../../architecture/runtime/active-mode/README.md).
This current-work file tracks the implementation gap against that contract.

## Selection Rules

If hard compaction is required or an unfinished compaction cycle exists, select
`Compaction` as a runtime interrupt. The interrupt snapshots owner state and
resumes the same owner mission after compaction. Else, if a pending owner queue
row exists, select `OwnerTask`. Else, if an active owner case exists and is not
closed, select `OwnerTask` or `Recovery`. Else, if maintenance is due, select
`Maintenance`. Otherwise select `ClosedIdle`.

## Policy Rendering

The rendered policy is the effective policy. Maintenance mode does not inherit
stale graph task refusals. Owner task and recovery modes do not inherit
maintenance restrictions. Runtime-owned compaction does not render a model
tool policy.

## Preemption Rules

Owner work preempts maintenance before the next endpoint turn. Maintenance
never continues after owner work is queued. Recovery for owner work blocks
maintenance.

## Recovery Interaction

Recovery is an owner-task mode, not idle maintenance. It may narrow tools, but
must route to a node that admits the productive tool or produce a blocked
handoff with exact evidence.

## Maintenance Interaction

Maintenance runs only when no owner task is pending, active, or recoverable.
No-op maintenance sets cooldown and does not write a memory row saying nothing
changed.

## Compaction Interaction

Hard compaction is runtime-owned and precedes endpoint or tool work when the
context is unsafe. It snapshots graph state, recovery state, artifact state,
fault state, and missing evidence without asking the model to run `memory.save`.

## Completion Interaction

Completion is owned by the active mode. Owner-task completion requires the
requested artifact and verification evidence. Maintenance closes as a bounded
maintenance outcome. Closed idle has no endpoint action.

## Implementation Hooks

- `crates/lkjagent-runtime/src/mode/`
- `crates/lkjagent-runtime/src/daemon/execute_pending.rs`
- `crates/lkjagent-runtime/src/daemon/idle.rs`
- `crates/lkjagent-runtime/src/daemon/context_budget.rs`
- `crates/lkjagent-tools/src/dispatch.rs`

## Tests

Focused tests cover owner preemption, recovery blocking maintenance,
runtime-owned compaction, maintenance-only policy, closed idle without
endpoint action, and refusal text naming a single active mode.

## Verification

Run focused runtime and tool tests, then `cargo run -p lkjagent-xtask --
quiet verify`, then `docker compose run --rm verify`.

## Status

partially implemented. Pure turn authority selection, store-backed mode
snapshots, endpoint decisions, one-card endpoint rendering, cached dispatch
authority, completion policy, mode rendering, dispatch-level effective policy,
stale maintenance action refusal before dispatch, and effective `agent.done`
completion refusal exist. Focused tests prove idle maintenance, owner queue
preemption at turn boundaries, maintenance write-tool refusal, stale graph
policy isolation, and closed-idle endpoint suppression. Pure kernel coverage
now proves empty maintenance cooldown returns to closed idle and owner queue
work preempts maintenance before a model call. Broader stale-action
contradiction repair, daemon endpoint-churn proof, artifact-aware close gates,
stronger per-case authority history, and richer compaction snapshots remain
open.
