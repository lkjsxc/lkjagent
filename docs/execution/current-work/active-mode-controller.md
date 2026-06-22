# Active Mode Controller

## Purpose

Own the work that makes one runtime mode determine model-visible tool policy,
recovery policy, compaction behavior, maintenance behavior, and completion
semantics at every endpoint turn.

## Owner Evidence

Uploaded run logs show maintenance and graph policy contradicting each other,
compaction requiring `memory.save` while graph policy refuses it, recovery
blocking the tool needed to escape, and completion closing without the
requested artifact.

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

If a pending owner queue row exists, select `OwnerTask`. Else, if an active
owner case exists and is not closed, select `OwnerTask` or `Recovery`. Else,
if compaction is required to keep the loop recoverable, select `Compaction`.
Else, if maintenance is due, select `Maintenance`. Otherwise select
`ClosedIdle`.

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

Hard compaction is runtime-owned. It snapshots graph state, recovery state,
artifact state, fault state, and missing evidence without asking the model to
run `memory.save`.

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

open; pure turn authority selection, store-backed mode snapshots, endpoint
decisions, one-card endpoint rendering, cached dispatch authority, completion
policy, mode rendering, dispatch-level effective policy, and effective
`agent.done` completion refusal exist. Stale-action refusal before dispatch,
artifact-aware close gates, durable authority snapshots, and richer compaction
snapshots remain open.
