# Tool Runtime

## Purpose

Implement lkjagent-tools: the dispatcher validating against the shared
registry table and the tool adapters, each returning bounded observations.

## Status

done

## Depends On

[protocol-parser.md](protocol-parser.md), [sqlite-store.md](sqlite-store.md),
[state-graph-runtime.md](state-graph-runtime.md), [context-engine.md](context-engine.md)
(observation caps).

## Files To Read

- [../../architecture/tools/registry.md](../../architecture/tools/registry.md)
- [../../architecture/tools/fs.md](../../architecture/tools/fs.md)
- [../../architecture/tools/shell.md](../../architecture/tools/shell.md)
- [../../architecture/tools/queue-ops.md](../../architecture/tools/queue-ops.md)
- [../../architecture/tools/memory-ops.md](../../architecture/tools/memory-ops.md)
- [../../architecture/tools/graph-ops.md](../../architecture/tools/graph-ops.md)
- [../../architecture/tools/control.md](../../architecture/tools/control.md)

## Files To Touch

- crates/lkjagent-tools/src/: dispatch.rs (validation order from the
  registry contract), fs.rs, shell.rs, queue.rs, memory.rs, graph.rs,
  control.rs, observe.rs (observation frame
  construction and truncation), error.rs.
- crates/lkjagent-tools/tests/: per-tool tables against tempdir
  filesystems and in-memory stores; shell tests against real /bin/sh.

## Focused Gate

```sh
cargo test -p lkjagent-tools
cargo clippy -p lkjagent-tools -- -D warnings
```

## Acceptance

- Every tool honors its documented parameters, defaults, and error cases;
  each has at least one ok-path and one error-path test.
- fs.edit refuses zero and multiple matches with the match count; fs.write
  observations carry path and byte count, never content.
- shell.run captures head and tail within the cap, reports exit codes,
  returns status error on non-zero exits, and enforces the timeout against a
  real slow command.
- Queue tools validate ids, statuses, and reasons; mutation tools call only
  lkjagent-store and return bounded observations.
- During maintenance, fs, shell, and queue actions have the same authority
  as task actions, bounded only by the container blast radius.
- Duplicate-read refusal and repeat-action detection produce the
  documented notices.
- Blocker row 8 done; tools area status moves in the ledger.

## Must Not

- Do not let any adapter decide policy; caps and restrictions arrive from
  the context engine and dispatcher as values.
- Do not add tools beyond the registry table; new capability is graph policy.
- Do not shell out from non-shell adapters.
