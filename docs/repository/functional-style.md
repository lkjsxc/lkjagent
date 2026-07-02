# Functional Style

## Purpose

Define the Rust style contract for pure cores and effect adapters.

## Core Rules

- Business logic is pure functions over plain data.
- Effects are returned as values or interpreted by adapters at crate edges.
- Product crates contain no `unwrap`, `expect`, `panic`, `todo`, or
  `unimplemented` paths.
- Illegal states are represented with enums and validated constructors.
- Plain structs and free functions beat trait hierarchies unless a seam is
  needed for real tests.
- No global mutable state or singleton runtime state.
- Newtypes protect meanings that must not mix, such as ids, token counts, and
  byte offsets.

## Crate Shape

| Module | Holds |
| --- | --- |
| `model` | data types and enums |
| pure modules | transitions, rendering, parsing, checks, planning |
| `adapter` | filesystem, SQLite, HTTP, shell, or clock interpreters |
| `error` | error enums and conversions |

## Tests

Pure cores get table-driven tests. Adapters get narrow tests against real local
resources such as temp directories and in-memory SQLite. A bug fix lands with
the test that would have caught it.
