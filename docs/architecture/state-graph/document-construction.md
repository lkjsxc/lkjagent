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

Long-form content requests are document deliverables when the owner asks for a
long story, novel, book, large guide, corpus, many files, structured output, or
when a write attempt hits max tokens or an unclosed content tag. These tasks
must not use one giant `fs.write`.

The route creates a semantic document root, a root README table of contents,
semantic child directories or files, bounded sections, and an audit report.
Names such as part-001.md are valid only when the owner asks for numbered
parts.

## Completion

Document completion requires README or index evidence, topology evidence,
count or scale evidence, link audit evidence, content presence evidence, and a
restart or read-order signal when relevant.

The model cannot complete a large document task by saying it is done; the
completion gate requires deterministic audit evidence.

## Status

implemented.
