# State Graph Runtime

## Purpose

Implement lkjagent-graph: typed graph definitions, task cases, transition
admission, context package selection, evidence requirements, compaction
preservation, completion gates, and source graph tests.

## Status

done

## Depends On

[protocol-parser.md](protocol-parser.md), [context-engine.md](context-engine.md),
[sqlite-store.md](sqlite-store.md).

## Files To Read

- [../../architecture/state-graph/README.md](../../architecture/state-graph/README.md)
- [../../architecture/state-graph/model.md](../../architecture/state-graph/model.md)
- [../../architecture/state-graph/transitions.md](../../architecture/state-graph/transitions.md)
- [../../architecture/state-graph/context-packages.md](../../architecture/state-graph/context-packages.md)
- [../../architecture/state-graph/completion.md](../../architecture/state-graph/completion.md)
- [../../decisions/state-graph-runtime.md](../../decisions/state-graph-runtime.md)

## Files To Touch

- crates/lkjagent-graph/src/: typed graph model, source graph, validation,
  routing, rendering, completion, compaction, and maintenance modules.
- crates/lkjagent-graph/tests/: validation, routing, slice rendering,
  completion, and compaction fixtures.

## Focused Gate

```sh
cargo test -p lkjagent-graph
cargo clippy -p lkjagent-graph -- -D warnings
```

## Acceptance

- Source graph validation rejects duplicate nodes, duplicate edges, and edges
  whose endpoints do not exist.
- Intent classification opens a planning case with evidence requirements and
  selected context packages before endpoint execution.
- Graph slice rendering is deterministic and budgeted.
- Completion is refused when required evidence is missing and accepted when
  the evidence exists.
- Compaction plans preserve active case state, evidence, missing evidence,
  selected packages, recovery strategy, and completion guard.
- Blocker row 7 done; state-graph status moves in the ledger.

## Must Not

- Do not leave task behavior as flat prompt reminders.
- Do not make graph state stringly typed where enums fit.
- Do not add IO to lkjagent-graph; runtime and store interpret decisions.
