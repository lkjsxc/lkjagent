# Artifact Ledger

## Purpose

Define artifact identity and the manifest that connects semantic records to
workspace files.

## Identity

Artifact identity is:

```text
artifact_id = normalized_owner_objective + root_path + artifact_kind
```

Artifact kinds include documentation tree, content artifact, story, cookbook,
guide, corpus, report, and repair bundle. The exact enum may be smaller, but
long stories and cookbooks must be classified as content artifacts.

## Manifest

Every artifact root carries `.lkj-artifact.md` or an equivalent extension of
`.lkj-doc-graph.md`. The manifest records artifact id, root, kind, title,
section roles, paths, required flags, content policy, and last audit result.

## Status

partially implemented; graph classification assigns content-artifact roots for
long stories and cookbooks, and `.lkj-doc-graph.md` records scaffold paths.
Stable `.lkj-artifact.md` identity, adoption, and repair decisions remain open.
