# Workspace Summary

## Purpose

Define workspace.summary, the bounded repository map tool. It gives a weak
model structure without shell listing commands.

## workspace.summary

Parameters are optional `path` default `.`, optional `depth` default `3`, and
optional `limit` default `200`. Output includes the root, Cargo workspace
signal when present, crate names when cheap to detect, docs root signal, and
sorted child paths up to the depth and limit.

The tool rejects workspace escapes and invalid limits. It is allowed in
planning, context, and execution nodes when graph policy admits inspection.

## Status

implemented.
