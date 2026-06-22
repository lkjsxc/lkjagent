# Documentation Catalog

## Purpose

The documentation catalog is the machine-readable index for authored project
Markdown. It replaces large generated graph files and root-level file manifests.

## Facts

- Catalog files live under [../_meta/catalog/](../_meta/catalog/README.md).
- Each catalog entry names a doc path, title, parent, immediate children, role,
  source links, and checks.
- README files remain handwritten navigation and link immediate children only.
- Ignored runtime output under `data/` is not cataloged.

## Design

The checker treats the catalog as coverage data, not prose. Every authored doc
under `docs/` must appear exactly once. A README is valid when its child list in
catalog matches the filesystem and the README links those immediate children.
Transient graph views can be rendered from the catalog into `tmp/`.

## Checks

- `cargo run -p lkjagent-xtask -- check-docs`
