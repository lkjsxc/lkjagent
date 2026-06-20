# Manifest

## Purpose

Define the artifact manifest that prevents duplicate roots and section roles.

## Fields

The manifest records:

- artifact key.
- kind.
- title.
- root.
- owner objective hash.
- nodes.
- roles.
- required files.
- content minimums.
- audit state.
- completion state.

The manifest stores identity and audit metadata, not huge raw content.

## Location

Use `.lkj-artifact.md` or an artifact-specific extension of
`.lkj-doc-graph.md`. The runtime may read either while migrating toward one
manifest format.

## Status

partially implemented through `.lkj-doc-graph.md`; stable artifact manifest
fields remain open.
