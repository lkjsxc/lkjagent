# Scaffold

## Purpose

Define how artifact scaffolding creates or repairs a semantic tree.

## Behavior

Before writing, the runtime scans the target root, known related roots, and
existing manifests. It adopts an equivalent root, repairs missing files, and
rejects a generic scaffold when an artifact-specific profile exists.

## Writes

Scaffold writes directories, README files, manifest files, semantic section
files, and audit placeholders only when the profile requires them. It does not
count empty placeholders as completed content.

For content artifacts, scaffold output must be followed by bounded content
write batches and a new audit before completion. See
[write-batches.md](write-batches.md) and [repair.md](repair.md).

## Duplicate Prevention

Duplicate detection uses artifact kind, normalized title, owner objective
hash, root role, README title, manifest artifact key, and section role.

## Status

partially implemented for story and cookbook profile paths and `artifact.apply`.
Root adoption and repair planning remain open.
