# Protocol Parser

## Purpose

Implement lkjagent-protocol: the pure parser from completion text to one
action, the renderer for observation, notice, owner, and skill frames, and
the fixture table both are tested against.

## Status

done with Implement strict action parser and renderer

## Depends On

[bootstrap-workspace.md](bootstrap-workspace.md);
[xtask-checks.md](xtask-checks.md).

## Files To Read

- [../../architecture/protocol/action-format.md](../../architecture/protocol/action-format.md)
- [../../architecture/protocol/parsing.md](../../architecture/protocol/parsing.md)
- [../../architecture/protocol/recovery.md](../../architecture/protocol/recovery.md)
- [../../architecture/tools/registry.md](../../architecture/tools/registry.md)
- [../../agent/skills/protocol-change.md](../../agent/skills/protocol-change.md)

## Files To Touch

- crates/lkjagent-protocol/src/: model.rs (Action, ParseFault, frame
  types), parse.rs, render.rs, registry.rs (the parameter table shared
  with dispatch and prompt generation), error.rs.
- crates/lkjagent-protocol/tests/: fixture table of recorded and
  constructed completions.

## Focused Gate

```sh
cargo test -p lkjagent-protocol
cargo clippy -p lkjagent-protocol -- -D warnings
```

## Acceptance

- Every parsing rule in parsing.md has at least one fixture row; every
  ParseFault variant is produced by at least one fixture.
- Render-parse round-trip property holds on all fixtures.
- Parameter validation consumes the registry table and reports all
  offenders in one fault, as the contract requires.
- No IO, no dependencies beyond the standard library in this crate.
- Blocker row 3 done; protocol area status moves in the ledger.

## Must Not

- Do not implement escaping, entities, or repair heuristics.
- Do not validate against tool semantics (file exists, command sane); the
  parser knows shapes, not the world.
- Do not exceed 200 lines per module; split parse rules before that.
