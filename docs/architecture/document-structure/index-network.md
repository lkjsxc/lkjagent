# Index Network

## Purpose

This file owns the graph-like index network required for authored documentation and generated artifacts.

## Contract

Filesystem shape is not enough. Each authored tree has index layers that let the runtime and an LLM verify coverage
without scanning every file. The layers are local READMEs, machine catalog metadata, relation pages, ownership maps,
state transition tables, artifact manifests, and benchmark fixture indexes where the tree is a fixture.

## Inputs

- local README tables of contents.
- catalog entries under `docs/_meta/catalog/`.
- relation records and backlinks.
- repository path ownership map.
- state transition tables.
- artifact manifests and benchmark fixture indexes.

## Outputs

- complete local navigation for each directory.
- catalog coverage for each authored Markdown page.
- relation records with source, target, relation type, rationale, audit mention, and repair route.
- orphan reports for authored pages outside the index network.

## Invariants

- A README links every sibling child.
- Every authored Markdown page appears in the catalog.
- Relation targets that require backlinks contain the backlink.
- State transition docs link the reducer or test surface that proves them.
- Artifact manifests name the same root the catalog names.

## Failure Cases

- A page exists only by filesystem discovery and is absent from the README path.
- A moved page keeps its old catalog path.
- A relation page points at a target with no backlink.
- A generated artifact lacks a manifest or fixture index.

## Verification

- `cargo run -p lkjagent-xtask -- check-docs`
- catalog orphan tests for moved, missing, and unlinked authored Markdown.
- relation backlink tests once relation enforcement is implemented.

## Status

design-only.
