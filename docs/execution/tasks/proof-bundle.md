# Proof Bundle

## Purpose

Provide a deterministic evidence collector for live and smoke runs so future
agents can inspect runtime state without copying SQLite databases or raw model
transcripts into context.

## Status

done in this working tree with `proof collect`, empty-store and seeded-artifact
focused tests, README coverage, and doc gates.

## Depends On

- [../../operations/verification.md](../../operations/verification.md)
- [live-manuscript-proof.md](live-manuscript-proof.md)
- [large-artifact-engine.md](large-artifact-engine.md)

## Files To Read

- `docs/operations/verification.md`
- `docs/execution/tasks/live-manuscript-proof.md`
- `crates/lkjagent-xtask/src/README.md`
- `crates/lkjagent-store/src/schema.rs`

## Files To Touch

- `docs/operations/verification.md`
- `docs/execution/tasks/proof-bundle.md`
- `crates/lkjagent-xtask/src/proof.rs`
- `crates/lkjagent-xtask/src/proof/` (new)
- `crates/lkjagent-xtask/src/README.md`

## Focused Gate

```sh
cargo test -p lkjagent-xtask proof
cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
```

## Acceptance

- `proof collect` succeeds without endpoint access.
- The bundle writes Markdown and text summaries only, never database copies or
  raw model transcript bodies.
- The collector reports status rows, queue counts, latest authority decisions,
  readiness projections, active contracts, recent transcript metadata, model-log
  file index, workspace tree, artifact word counts, and warnings for missing rows.
- Empty-store and seeded-artifact tests prove the command works at both
  boundaries.
- The live manuscript proof task uses the bundle path as required evidence.

## Must Not

- Do not print raw queue content, transcript content, model-log bodies, or file
  contents into the proof bundle.
- Do not require endpoint configuration.
- Do not treat the proof bundle as completion evidence by itself.
