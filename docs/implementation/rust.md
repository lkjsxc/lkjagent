# Rust

## Purpose

Rust owns lkjagent's implementation substrate: workspace crates, typed domain
values, pure reducers, explicit error values, and Docker Compose gates.

## Contract

- Product crates avoid panic paths and keep effects at adapters.
- Pure functions own state reduction, audit decisions, prompt-frame selection,
  and tool authorization.
- Source files stay below the repository line cap.
- Docker Compose verification proves the committed repository from a clean
  build context.

## Links

- Functional core: [functional-core.md](functional-core.md).
- State vector: [../state/state-vector.md](../state/state-vector.md).
- Relation: [../relations/project-model-implementation.md](../relations/project-model-implementation.md).

## Status

implemented
