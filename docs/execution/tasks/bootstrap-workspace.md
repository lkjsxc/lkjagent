# Bootstrap Workspace

## Purpose

Create the cargo workspace so every later slice has a home: the workspace
manifest, the nine crate skeletons with their README maps, lint
configuration, and the Dockerfile that builds the binary.

## Status

done with Enable concrete Rust workspace bootstrap

## Depends On

Nothing. First blocker.

## Files To Read

- [../../repository/layout.md](../../repository/layout.md)
- [../../repository/functional-style.md](../../repository/functional-style.md)
- [../../decisions/rust-workspace.md](../../decisions/rust-workspace.md)
- [../../architecture/sandbox/container.md](../../architecture/sandbox/container.md)

## Files To Touch

- Cargo.toml (new): workspace members, shared lints (clippy unwrap_used,
  expect_used, panic, todo, unimplemented denied for product crates),
  shared profile.
- crates/lkjagent-{protocol,graph,context,store,llm,tools,runtime,cli,xtask}/
  (new): each with Cargo.toml, src/lib.rs or src/main.rs containing only
  what compiles honestly (type and module declarations belong to later
  tasks; an empty lib is honest, a stubbed function is not), and README.md
  per the layout convention.
- Dockerfile (new): multi-stage build to the runtime image described in
  the container contract.
- rustfmt.toml (new), .gitignore (extend if needed).

## Focused Gate

```sh
cargo build --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
docker build .
```

## Acceptance

- All four commands exit 0 from a clean checkout.
- Every crate directory has a README.md naming its doc contract.
- No function body in any crate contains todo-style or stubbed logic; empty
  modules are the only permitted emptiness.
- [../../current-state.md](../../current-state.md) row for the workspace
  moves to implemented; blocker row 1 moves to done with the commit.

## Must Not

- Do not add product logic; this task is structure only.
- Do not add dependencies beyond the decided set; the bootstrap needs
  almost none of them yet.
- Do not create files over 200 lines, including generated manifests.
