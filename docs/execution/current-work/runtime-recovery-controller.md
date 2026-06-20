# Runtime Recovery Controller

## Purpose

Own the work that makes runtime recovery deterministic, productive, and unable
to trap the daemon in repeated invalid action, invalid recovery, compaction, or
completion loops.

## Contract

The runtime records each fault as typed graph state, then asks the graph
controller for the next node, action class, and missing evidence. Prompt text
may explain the route, but it is not the source of policy.

Fault classification separates parse, parameter, tool, repeat, endpoint,
budget, context, verification, compaction, payload, and completion faults.
Each class has a forced route and a bounded retry count.

Recovery node selection uses transition quality over outgoing edges. Legal
targets with useful evidence gain priority. Repeated targets, blocked guards,
and repeated action classes lose priority.

Action surface shrinkage is mandatory after repeated faults. The graph card
must render a smaller set of copyable legal actions and must not suggest a tool
that graph policy refuses.

Invalid recovery actions are state-changing evidence. A repeated invalid
`graph.note`, `graph.next`, mutation, compaction, or owner question advances
the recovery ladder instead of appending another generic warning.

`graph.next` is diagnostic only. After one diagnostic inspection for the same
fault, the next route must be `graph.recover`, a legal transition, an unused
non-mutating native inspection tool, or a smaller plan step.

Mutation blocking must have an escape. If recovery blocks `fs.write`,
`fs.batch_write`, or `doc.scaffold`, the controller must route to restored
planning, document topology, alternate inspection, or a smaller step rather
than leaving mutation as the only productive action.

Compaction is runtime-owned. Forced compaction never asks the model to run
`memory.save` when graph policy blocks it.

Partial progress is accepted only as typed evidence: attempted action, files
or graph state that exist, failed gate, recovery node, and next executable
action. It is not successful completion.

`agent.ask` is allowed only when a concrete owner-required question exists and
no internal legal route, alternate native tool, or smaller independent step can
continue.

Waiting is forbidden for internally recoverable faults.

Shell is admitted only from a shell-admitted recovery or verification node
after native inspection, alternate native tools, and smaller scope have failed.

## Failure Modes

- The model repeats `graph.next` without changing action class.
- The model uses `graph.note` with an invalid kind after a refusal.
- Compaction demands `memory.save` while graph policy blocks `memory.save`.
- A recovery node blocks the only productive tool class.
- The model attempts `fs.write` or `doc.scaffold` while recovery blocks mutation.
- Parse faults continue past escalation without a forced smaller-scope route.
- Tool faults continue past escalation without alternate native tool routing.
- A document task uses a giant `fs.write` instead of document construction.
- The agent asks the owner despite no true owner decision being required.
- The agent waits during an internally recoverable fault.

## Implementation Hooks

- `crates/lkjagent-runtime/src/step/fault_wait.rs`
- `crates/lkjagent-runtime/src/step/graph_phase.rs`
- `crates/lkjagent-runtime/src/graph_state.rs`
- `crates/lkjagent-runtime/src/task.rs`
- `crates/lkjagent-tools/src/dispatch.rs`
- `crates/lkjagent-tools/src/dispatch/validate.rs`
- `crates/lkjagent-tools/src/dispatch/graph_tools.rs`
- `crates/lkjagent-graph/src/transition.rs`
- `crates/lkjagent-graph/src/source_edges.rs`
- `crates/lkjagent-graph/src/source_recovery.rs`
- `crates/lkjagent-graph/src/source_recovery_extra.rs`
- `crates/lkjagent-graph/src/source_nodes.rs`

## Tests

- `crates/lkjagent-runtime/tests/recovery_controller.rs`
- `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- `crates/lkjagent-graph/tests/best_next_transition.rs`
- `crates/lkjagent-graph/tests/graph.rs`
- `crates/lkjagent-benchmark` recovery-loop long-story corpus fixture.

## Verification

```sh
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo test -p lkjagent-graph
cargo test -p lkjagent-tools
cargo test -p lkjagent-runtime
cargo run -p lkjagent-xtask -- benchmark check-corpus
docker compose run --rm verify
```

## Status

open until docs, code, focused tests, corpus checks, quiet verify, and Docker
Compose verification prove the controller behavior.
