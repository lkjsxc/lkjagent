# Audit

## Purpose

This file owns deterministic documentation topology and catalog audit rules.

## Facts

- `crates/lkjagent-xtask` runs repository-level doc checks.
- The catalog lives under [../../_meta/catalog/](../../_meta/catalog/README.md).
- Ignored runtime output under `data/` is not part of authored docs.

## Design

The audit checks Markdown shape, README topology, local child links, relative
links, path hygiene, catalog coverage, catalog parent fields, line caps, and
banned release or compatibility wording. Failures name the path, check, and
repair text. Generated graph Markdown is not required; graph output is derived
from the catalog only after the catalog check passes.

## Checks

- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
