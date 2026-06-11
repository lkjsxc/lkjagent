# Skill: Rust Crate Slice

## Purpose

Implement one execution task's slice of a crate: contract first, pure core,
thin adapters, focused tests, honest gates.

## Trigger

An execution task calls for implementing or extending a crate.

## Context

- The task file under [../../execution/tasks/](../../execution/tasks/README.md): its Files and gates bind the session.
- The crate's doc contract from the ownership table in [../../repository/layout.md](../../repository/layout.md).
- [../../repository/functional-style.md](../../repository/functional-style.md): core-adapter split, no panic paths.
- [../../repository/line-limits.md](../../repository/line-limits.md): split before exceeding.

## Procedure

1. Read the task file end to end; read every file under Files To Read.
2. Adjust the doc contract first if the task revealed a gap; a slice built
   on a wrong contract is rework, per
   [../work-loop.md](../work-loop.md) step 4.
3. Define the data: model module enums and structs that make illegal states
   unrepresentable. No logic yet.
4. Write the pure core: transition and constructor functions over the
   model, each file under 200 lines.
5. Write table-driven tests for the core from the contract's tables and
   examples; every enum variant and every fault path gets a row.
6. Write the thinnest adapter that connects the core to its external
   system, with no decisions inside.
7. Wire the crate README: Purpose, table of contents, ownership line per
   [../../repository/layout.md](../../repository/layout.md).
8. Update the task file Status, the blocker row in
   [../../execution/current-blockers.md](../../execution/current-blockers.md),
   and [../../current-state.md](../../current-state.md) if an area moved.

## Checks

- `cargo test -p <crate>` passes; new behavior has at least one test that
  fails when the behavior is reverted.
- `cargo fmt --check` and crate clippy with warnings denied are clean.
- `cargo run -p lkjagent-xtask -- quiet verify` prints ok verify before
  handoff (skip only while the xtask task itself is still open, and say so).

## Must Not

- Do not use unwrap, expect, panic, todo, or unimplemented in product code.
- Do not put IO or clocks inside core modules; cores take values.
- Do not implement beyond the task's acceptance line; the next slice has
  its own task.
- Do not mark the task done with any gate unrun.

## Handoff

Name the crate, the task file row updated, the gates run with results, and
the next slice's task file.
