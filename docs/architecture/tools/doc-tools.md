# Document Tools

## Purpose

Define graph-native document construction helpers for structured artifacts.
The semantic tree and graph contract lives in
[../document-structure/](../document-structure/README.md).

## doc.scaffold

Creates a compact semantic tree under `root`. Parameters are `root`, optional
`kind`, optional `count`, optional `mode` (`exact` or `approx`), required
`title`, and optional newline sections. It generates README indexes and a graph
manifest for documentation roots.

## doc.audit

Audits a document root for README presence, local child links, forbidden
sequence-only names, graph manifest presence, graph references, line caps, and
optional file-count target. Passing audits can satisfy document-structure
evidence when the active graph gate accepts that evidence.

## Status

partially implemented.
