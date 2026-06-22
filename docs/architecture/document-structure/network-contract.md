# Network Contract

## Purpose

This file defines the documentation graph data used for audit and repair.

## Facts

- Authored docs are indexed in [../../_meta/catalog/](../../_meta/catalog/README.md).
- Generated graph views are transient and belong under `tmp/`.
- README files are table-of-contents pages, not graph manifests.

## Design

The graph is derived from catalog entries. Nodes are doc paths. Parent and child
fields define containment edges. Source links and checks attach implementation
and verification evidence without repeating boilerplate in every page. A graph
view is valid only when the catalog checker passes first.

## Checks

- `cargo run -p lkjagent-xtask -- check-docs`
