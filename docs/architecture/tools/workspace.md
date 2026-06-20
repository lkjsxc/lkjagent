# Workspace Summary

## Purpose

Define workspace summary and index tools. They give a weak model structure
without shell listing commands.

## workspace.summary

Parameters are optional `path` default `.`, optional `depth` default `3`, and
optional `limit` default `200`. Output includes the root, Cargo workspace
signal when present, crate names when cheap to detect, docs root signal, and
sorted child paths up to the depth and limit.

The tool rejects workspace escapes and invalid limits. It is allowed in
planning, context, and execution nodes when graph policy admits inspection.

## workspace.index

Parameters match workspace.summary. Output focuses on deterministic ownership
signals: top directories, README count, manifest paths, file and directory
counts, and bounded child paths. It is preferred during initial survey and
context refresh because it is smaller than raw listings.

## Status

implemented.
