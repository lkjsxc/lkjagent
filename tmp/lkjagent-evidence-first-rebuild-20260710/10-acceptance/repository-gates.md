# Repository Gates

## Documentation

- Every docs directory has one useful README and at least two children.
- Docs and Rust files remain within 200 lines.
- Links resolve and root read order reaches all current contracts.
- No release-style names, retired bridge claims, old demo contract, or false
  current-state proof remains.
- Each behavior claim maps to source, test, and final evidence.

## Source

- No panic, unsafe, unfinished macro, product mock, canned placeholder, or bulk
  scaffold path.
- File count and module ownership remain bounded.
- No production task, step, template, bridge, or finish authority remains.

## Build

- Cargo.lock is tracked.
- Every Docker COPY source is tracked.
- Locked build, format, lint, tests, deterministic replay, and quiet verification
  pass.
- A clean Git archive passes Docker Compose without ignored local files.

## CI

The public final commit passes the same workflow. Local success cannot replace
public CI evidence.

## Tree

Worktree is clean and contains no accidental secrets, databases, endpoint logs,
or unrelated owner changes.
