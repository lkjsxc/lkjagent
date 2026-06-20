# Artifact Ledger

## Purpose

Define artifact identity and the manifest that connects semantic records to
workspace files.

## Identity

Artifact identity is:

```text
artifact_key = normalized owner objective + artifact root + artifact kind
```

Artifact kinds include story, cookbook, guide, encyclopedia, knowledge base,
and project docs. Long stories and cookbooks must never fall back to generic
project documentation unless the owner explicitly asks for project docs.

## Manifest

Every artifact root carries `.lkj-artifact.md` or an equivalent extension of
`.lkj-doc-graph.md`. The manifest records artifact key, root, kind, title,
owner objective hash, node roles, required files, content minimums, audit
state, and completion state.

Adoption and repair record the source root, changed paths, source row or
manifest IDs when available, readiness gaps, and next executable action.

## Status

partially implemented; graph classification assigns content-artifact roots for
long stories and cookbooks, and `.lkj-doc-graph.md` records scaffold paths.
Stable `.lkj-artifact.md` identity, adoption, and repair decisions remain open.
