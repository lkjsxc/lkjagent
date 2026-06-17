# Skill Runtime

## Purpose

Implement lkjagent-skills: the format validator, the library index with
deterministic degradation, the loader, and the five seed skills, written
for real and validated by the same rules.

## Status

done

## Depends On

[sqlite-store.md](sqlite-store.md) (index stamps); frame types from
[protocol-parser.md](protocol-parser.md).

## Files To Read

- [../../architecture/skills/format.md](../../architecture/skills/format.md)
- [../../architecture/skills/loading.md](../../architecture/skills/loading.md)
- [../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md)
- [../../architecture/skills/library.md](../../architecture/skills/library.md)
- [../../agent/skills/skill-system.md](../../agent/skills/skill-system.md)

## Files To Touch

- crates/lkjagent-skills/src/: model.rs, validate.rs (the rule table
  shared with check-docs), index.rs (index lines, budget degradation),
  load.rs (file to skill frame).
- crates/lkjagent-skills/seeds/: the five seed skill files from the
  library contract, each a real working procedure.
- crates/lkjagent-skills/tests/: validation table, degradation
  determinism, seed validation.

## Focused Gate

```sh
cargo test -p lkjagent-skills
cargo clippy -p lkjagent-skills -- -D warnings
cargo run -p lkjagent-xtask -- check-docs
```

## Acceptance

- The validator enforces every format.md rule; one fixture per violation
  class asserts the exact message listing all violations at once.
- Index generation is deterministic under the 512-token budget, including
  the degradation order.
- All five seeds validate, name real commands, and contain Checks sections
  with evidence that can actually appear.
- check-docs and the runtime validator consume one shared rule table;
  drift is structurally impossible.
- Blocker row 7 done; skills area status moves in the ledger.

## Must Not

- Do not write seed skills whose procedures were never executed; run each
  procedure once in the container or mark the task blocked.
- Do not implement skill search or categories; the index is the discovery
  mechanism.
- Do not let a seed exceed 120 lines.
