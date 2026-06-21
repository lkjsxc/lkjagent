# Semantic Identity

## Purpose

Define the artifact identity that follows a content task through adoption,
repair, compaction, and completion.

## Contract

An artifact is not just a path. Identity includes root, profile, normalized
title, requested scale, manifest key, README title, semantic section roles,
owner objective hash when available, and active case id.

## State

`ArtifactCase` records root, profile, declared scale, actual leaf paths,
required leaf paths, README paths, manifest path, weak paths, missing links,
scaffold-only paths, readiness evidence, last audit, and next batch cursor.

## Invariants

- Identity is attached before readiness or completion.
- Adoption preserves identity instead of creating duplicate roots.
- Compaction snapshots include identity fields.
- A README or manifest alone cannot make the artifact ready.

## Fixture

`cookbook_scaffold_only_foundations` proves a known root remains the same
artifact while weak foundation leaves are repaired.

## Verification

Run `cargo test -p lkjagent-tools artifact_tools` and
`cargo test -p lkjagent-runtime artifact_completion_gate`.

## Status

design-only for the full `ArtifactCase` state.
