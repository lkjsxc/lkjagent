# Functional Style

## Purpose

The Rust style contract: a pure functional core with effects at the edges,
so that every decision the harness makes is a testable function and every
effect is an inspectable adapter.

## Core Rules

- Business logic is pure functions over plain data: `(State, Input) ->
  (State, Effects)`. The loop's transition function in
  [../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md)
  is the pattern for the whole workspace.
- Effects (file IO, shell, HTTP, SQLite, clocks) are values returned by the
  core and interpreted by thin adapters at the crate edge. Adapters contain
  no decisions; cores contain no IO.
- No panic paths in product crates: unwrap, expect, panic, todo,
  unimplemented, and indexing that can panic are banned. Errors are enum
  values carried in Result; the check-style gate scans for offenders.
- Model states as enums so illegal states are unrepresentable: task state,
  parse faults, notice kinds, and stop reasons are closed enums, never
  strings or booleans in disguise.
- Plain data over abstraction: structs and enums with public fields,
  constructed by free functions. No trait hierarchies for things with one
  implementation; a trait earns its place only as a genuine seam (the
  endpoint client, the clock) needed by tests.
- No global mutable state, no statics with interior mutability, no
  singletons. Everything a function needs arrives as a parameter.
- Newtypes for meanings that must not mix: token counts, turn numbers,
  queue ids, byte offsets.

## Shape Within a Crate

| Module | Holds |
| --- | --- |
| model | data types and enums, no logic |
| (core modules) | pure transitions and constructors, named by what they decide |
| adapter | effect interpreters, one per external system |
| error | the crate's error enum and conversions |

Every module fits the 200-line cap; the split recipe is in
[line-limits.md](line-limits.md).

## Tests

- Cores get table-driven tests: rows of input and expected output, growing
  whenever live transcripts surface a new case per
  [../architecture/memory/transcripts.md](../architecture/memory/transcripts.md).
- Adapters get narrow integration tests against real local resources
  (tempdir filesystems, in-memory SQLite); never against fakes that
  hand-wave the contract, per [../agent/honest-state.md](../agent/honest-state.md).
- A bug fix lands with the test that would have caught it.

## Dependencies

Few and load-bearing: an HTTP client, a SQLite binding, serde for the wire
format only, a TOML reader. Each new dependency is a decision recorded in
[../decisions/rust-workspace.md](../decisions/rust-workspace.md) terms:
what it replaces and why hand-rolling loses.
