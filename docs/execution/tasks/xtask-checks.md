# Xtask Checks

## Purpose

Build the local verification gates so every repository rule can be checked
before the compose final gate exists: check-docs, check-lines, check-style,
quiet test, and quiet verify.

## Status

done with Make repository gates enforce their contracts

## Depends On

[bootstrap-workspace.md](bootstrap-workspace.md)

## Files To Read

- [../../operations/verification.md](../../operations/verification.md)
- [../../repository/documentation-standards.md](../../repository/documentation-standards.md)
- [../../repository/line-limits.md](../../repository/line-limits.md)
- [../../architecture/skills/format.md](../../architecture/skills/format.md)
- [README.md](README.md) (the task template check-docs must enforce)
- [../../agent/skills/verification-gate.md](../../agent/skills/verification-gate.md)

## Files To Touch

- crates/lkjagent-xtask/src/: main.rs dispatcher, one module per gate,
  each a pure judgment over collected repository facts; fixtures under
  crates/lkjagent-xtask/tests/.

## Focused Gate

```sh
cargo test -p lkjagent-xtask
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo run -p lkjagent-xtask -- check-style
cargo run -p lkjagent-xtask -- quiet verify
```

## Acceptance

- All gates pass on the repository as committed, printing exactly their ok
  lines; the interim shell checks in verification.md are retired from that
  file in the same change.
- Fixture tests cover every violation class with exact-message assertions.
- check-docs enforces: H1-Purpose shape, README topology and TOC
  completeness, the All Files manifest, ASCII, prose width, banned tokens,
  skill shape (both libraries), task shape, crate README coverage.
- Blocker row 2 done; verification.md Status moves to implemented.
  The compose final gate task owns the CI workflow.

## Must Not

- Do not special-case any real file to make a gate pass; fix the file or
  fix the rule's owner doc first.
- Do not print anything on success beyond the ok line.
- Do not let CI run any command other than the final gate.
