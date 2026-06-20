# Recovery And Maintenance Loop Redesign

## Purpose

Own the work to remove contradictory recovery, maintenance, compaction, and
graph-policy loops. The target is one deterministic active mode per turn, with
Rust data selecting allowed tools, next action classes, and completion gates.

## Owner Evidence

Uploaded GPT-5.5-Pro logs show repeated parse faults, invalid parameter loops,
invalid `graph.note` kinds, invalid `memory.save` kinds, duplicate memory
entries, blocked `memory.save` during maintenance, blocked `doc.scaffold`
during recovery, repeated `graph.next`, premature `agent.done`, and completed
tasks that did not create the requested long story or cookbook.

## Failure Modes

- Maintenance continues while owner work is queued or active.
- Maintenance asks the owner for internal transcript or memory decisions.
- Maintenance saves the same lesson repeatedly.
- Maintenance has no delete/update tool but claims pruning completed.
- Graph policy refuses `memory.save` during maintenance.
- Maintenance policy refuses `graph.state` while graph policy recommends it.
- Compaction says `memory.save` only while graph policy refuses it.
- Long content task attempts giant `fs.write` and hits max tokens.
- Unclosed content tags repeat after max-token truncation.
- `graph.note` examples or guesses use invalid kinds.
- `graph.evidence` receives note kinds such as decision or risk.
- `graph.plan` examples omit checks or paths that dispatch requires.
- `graph.transition` refusal recommends impossible target nodes.
- Complete node is reached while requested artifact does not exist.
- `agent.done` succeeds after planning only, without artifact evidence.

## Desired Runtime Contract

- Owner tasks preempt maintenance before the next endpoint turn.
- Maintenance never runs while owner work is active or recoverable.
- One active mode owns graph, maintenance, compaction, and recovery policy.
- Rendered examples parse, validate, and dispatch.
- Completion requires artifact evidence or a blocked handoff.
- Long content is built as semantic artifact trees, not one giant write.
- Duplicate document trees and duplicate memory rows are refused or repaired.
- Repeated invalid actions force a different action class.
- Recovery never suggests a tool that active policy rejects.

## Active Mode Contract

`OwnerTask`, `Recovery`, `Maintenance`, `Compaction`, and `ClosedIdle` are the
only modes. Each mode defines allowed tools, blocked tools, preferred next
action, valid examples, completion condition, preemption, fault route, and
which policy layer applies. Only one mode renders policy in a turn.

## Recovery Contract

Recovery belongs to the active owner task unless explicitly recovering
maintenance. It shrinks the tool surface deterministically, records repeated
faults, normalizes safe aliases, renders copyable valid examples, and forces a
transition, smaller scope, alternate native tool, or blocked handoff after
repeated diagnostics.

## Maintenance Contract

Maintenance opens only when no owner task is active, no owner task is
recoverable, and the queue is empty. It is bounded background bookkeeping. It
may not ask the owner internal runtime questions, may not write duplicate
memory rows, and may not claim pruning unless delete, rewrite, or merge ran.

## Compaction Contract

Hard compaction is runtime-owned. It preserves active owner case state,
evidence, touched paths, graph state, and fault route without requiring a
model-authored `memory.save`. Optional model-authored lessons belong to
maintenance mode and still pass memory deduplication.

## Completion Contract

`agent.done` closes owner work only when plan, observation, verification, and
artifact evidence are ready. Content artifacts require a root, README,
manifest, semantic children, content-bearing files, and a passing audit. A
blocked close must name failed checks and preserve blocked status.

## Implementation Hooks

- `crates/lkjagent-runtime/src/*`
- `crates/lkjagent-runtime/src/step/*`
- `crates/lkjagent-tools/src/dispatch.rs`
- `crates/lkjagent-tools/src/dispatch/validate.rs`
- `crates/lkjagent-tools/src/dispatch/graph_tools.rs`
- `crates/lkjagent-tools/src/memory.rs`
- `crates/lkjagent-store/src/memory.rs`
- `crates/lkjagent-graph/src/transition.rs`
- `crates/lkjagent-graph/src/source_edges.rs`
- `crates/lkjagent-graph/src/source_recovery.rs`
- `crates/lkjagent-graph/src/state_track.rs`
- `crates/lkjagent-protocol/src/registry_spec.rs`

## Tests

Focused tests must cover active-mode exclusivity, maintenance preemption,
deduplicated memory writes, punctuation-safe memory search, semantic examples,
illegal-transition refusal, content-artifact routing, completion readiness,
owner-question refusal, and benchmark fixtures from the uploaded logs.

## Verification

Run focused crate tests as each slice lands. The redesign is not implemented
until `cargo run -p lkjagent-xtask -- quiet verify` and
`docker compose run --rm verify` pass in the final handoff.

## Status

open

Implemented so far: pure active-mode selection and policy rendering exist,
maintenance/compaction modes do not render graph-policy refusals, and internal
owner questions are refused with valid next-action examples. Graph-policy
refusals now render `graph.transition` examples only with admitted targets.
Remaining work: full loop selection before endpoint turns, idempotent
maintenance, richer structured compaction snapshots, transition selector
runtime integration, artifact planning, completion gates, and benchmark
fixtures.
