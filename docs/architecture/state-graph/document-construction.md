# Document Construction

## Purpose

Define graph treatment for large, counted, or structured document tasks.

## Topology First

Document tasks route through document-profile, topology, scaffold,
section-plan, write, audit, repair, and completion-check nodes. The graph
requires topology evidence before bulk writing and audit evidence before
completion.

The document state ledger carries root, kind, language when detected, count
target, count mode, root README status, docs/main split, section map, coverage
map, first and last main path, sequence status, audit status, and repair
needs.

## Completion

Document completion requires README or index evidence, topology evidence,
count or scale evidence, link audit evidence, content presence evidence, and a
restart or read-order signal when relevant.

The model cannot complete a large document task by saying it is done; the
completion gate requires deterministic audit evidence.

## Status

implemented.
