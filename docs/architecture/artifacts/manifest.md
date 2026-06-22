# Manifest

## Purpose

Define the artifact metadata that prevents duplicate roots and weak section
roles.

## Fields

The metadata records artifact key, kind, title, root, owner objective hash,
nodes, roles, required files, content minimums, audit state, and completion
state. It stores identity and audit metadata, not raw content.

## Location

Generated artifact roots use `catalog.toml` for compact scaffold metadata.
Artifact-specific readiness fields are stored in the runtime state ledger until
a dedicated artifact metadata file is implemented.

## Checks

- `cargo test -p lkjagent-tools --test artifact_tools`
