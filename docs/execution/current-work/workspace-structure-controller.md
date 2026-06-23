# Workspace Structure Controller

## Purpose

This task implements recursive structure planning for generated documentation artifacts and `/data/workspace` roots.

## Contract

The controller consumes a root path, fan-out cap, README rules, catalog rules, relation rules, line limits, artifact
kind, and owner objective. It returns a structure plan with moves, creates, deletes, README updates, catalog updates,
relation updates, backlinks, and audit actions.

## Inputs

- document-structure fan-out and index-network contracts.
- doc audit and profile code in `lkjagent-tools`.
- generated workspace evidence under `data/workspace/`.
- benchmark fixture requirements for generic scaffold rejection.

## Outputs

- reusable `StructurePlan` data shape.
- audit failures for over-cap directories, missing README links, missing catalog paths, and missing backlinks.
- repair plan that creates semantic groups only when ownership is real.
- fixture proving generic scaffold roots fail readiness.

## Invariants

- A move updates links and catalog entries in the same action.
- Every generated child name is kebab-case.
- Sequence-only part files are forbidden unless explicitly requested.
- Recursive repair can stop with a smaller meaningful tree rather than a large scaffold.
- `tmp/` remains ignored unless a tracked fixture explicitly owns the files.

## Failure Cases

- `/data/workspace` root lacks README or agent instructions.
- A generated docs tree has too many direct children.
- README, catalog, and relation surfaces disagree after a move.
- Generic topic buckets replace objective-specific content.

## Verification

- structure audit tests for README, catalog, backlink, fan-out, and line caps.
- benchmark fixture `workspace-generic-scaffold-rejected`.
- `cargo run -p lkjagent-xtask -- check-docs`

## Status

open.
