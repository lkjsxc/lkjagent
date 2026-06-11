# Rust Workspace

## Purpose

Fix the implementation language and the unit of code organization.

## Decision

lkjagent is written in Rust as one cargo workspace of small focused crates.
Each crate owns one concern, keeps every file at or below 200 lines, and
exposes a pure core with effectful adapters at its edge. The crate map is
owned by [../repository/layout.md](../repository/layout.md).

Dependencies are few and boring: an HTTP client, a SQLite binding, a TOML
reader, and serde for the endpoint wire format only. Anything a skill can do
through shell does not become a dependency.

## Consequences

- Compile-time guarantees carry much of the verification burden; the type
  system encodes protocol states instead of runtime checks.
- The 200-line cap forces module splits early; crate boundaries follow
  ownership, not layers.
- No panic paths in product crates: unwrap, expect, panic, todo, and
  unimplemented are banned by
  [../repository/functional-style.md](../repository/functional-style.md).
- Build times stay short because the dependency tree stays shallow.

## Rejected Directions

- C++ with hand-rolled HTTP and JSON: proven possible in a prior project but
  the memory-safety tax and test burden slowed every change.
- TypeScript or Go: faster to draft, but a long-running daemon on a small
  machine wants Rust's memory behavior and single static binary.
- One large crate: hides ownership and lets files grow past review size.
