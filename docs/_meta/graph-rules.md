# Graph Rules

## Purpose

This page defines how documentation graph views are derived without committing
large generated Markdown files.

## Facts

- The source of truth is [catalog/](catalog/README.md).
- Runtime graph or audit views are transient artifacts and belong under `tmp/`.
- A README is valid only when it links each immediate child in its directory.

## Design

The graph is derived from catalog parent and child fields. The checker verifies
coverage before any rendered graph is trusted. Rendered graphs must not claim
audit success; they only report observed checker output.

## Checks

- `cargo run -p lkjagent-xtask -- check-docs`
