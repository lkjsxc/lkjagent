# Fan Out

## Purpose

This file owns the repository rule for direct child count in authored navigation trees.

## Contract

Authored navigation trees keep direct children small enough for one LLM pass. The target cap is eight direct
children per directory, counting `README.md`. A directory with authored children contains `README.md` and at least
two siblings. Split only by ownership: topic, state, artifact role, relation group, or runtime boundary.

Tool-required root files remain where their tools require them. Cargo, Docker, Git, GitHub, license, format, and
local environment sentinel files are valid root exceptions when moving them breaks ordinary commands.

## Inputs

- directory path.
- list of authored files and child directories.
- tool-required sentinel paths.
- README links and catalog entries.
- line-limit pressure.

## Outputs

- accepted directory shape.
- grouping action when a directory exceeds the cap.
- exception note for required root sentinels.
- README and catalog update requirements.

## Invariants

- A new authored child is not added to an over-cap directory without a grouping plan.
- A grouping directory owns a real concept and is not a numeric bucket.
- Moves update links, README tables, catalog entries, and relation edges in the same change.
- A directory with one meaningful child is collapsed unless another real sibling exists.
- Runtime output under `data/workspace/` and `data/logs/` is evidence, not authored repository navigation.

## Failure Cases

- A flat docs directory forces an agent to scan too many siblings before finding the contract.
- A synthetic bucket hides ownership and creates duplicate contract pages.
- A move leaves stale README links or catalog paths.
- A temporary prompt packet under `tmp/` is treated as authored Markdown.

## Verification

- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- focused structure audit tests once the fan-out checker is implemented.

## Status

design-only.
