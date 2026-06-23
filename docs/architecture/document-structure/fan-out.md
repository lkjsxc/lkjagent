# Fan Out

## Purpose

This file owns direct-child fan-out rules for generated documentation and generated workspaces.

## Contract

A generated authored tree keeps at most eight direct children per directory, counting `README.md`. Each directory
contains `README.md` plus at least two semantic siblings. The structure controller splits an over-cap directory by
meaningful ownership before writing another child.

Workspace roots follow the same shape when the owner asks for a navigable artifact. Valid roots normally contain a
README, local agent instructions, docs or content, checks or tests when needed, data or assets when needed, `tmp/`,
and ignore rules for local debris.

## Inputs

- root path.
- maximum direct authored children.
- minimum sibling count.
- file line limits.
- README, catalog, relation, and manifest requirements.
- artifact kind and owner objective.

## Outputs

- structure plan with moves, creates, deletes, README updates, catalog updates, and relation updates.
- audit findings with exact over-cap directories.
- repair action naming the grouping rationale.

## Invariants

- Sequence-only groups are invalid unless the owner explicitly requested sequence files.
- Every README links every sibling child.
- Every catalog or manifest path matches the repaired tree.
- Relation pages receive backlinks when they name cross-topic edges.
- The controller prefers a smaller meaningful tree over a large scaffold.

## Failure Cases

- Generic `overview`, `architecture`, `guides`, `operations`, and `reference` trees appear without objective need.
- A generated root has many direct children and no semantic groups.
- A README links stale paths after a repair move.
- A catalog claims coverage for files that no longer exist.

## Verification

- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- structure controller tests for over-cap roots, missing README links, and move-only repairs.

## Status

design-only.
