# Semantic Merge

## Purpose

Define non-exact memory merge behavior.

## Candidate Search

Merge candidates are selected by kind, normalized title, tag overlap, and
content shingle overlap. Punctuation-heavy owner queries must be sanitized so
FTS search does not fail before candidate ranking.

## Merge Rules

Semantic merge preserves real facts and source row IDs. The implemented prune
path merges same-kind, same-title, high-overlap rows into the oldest row,
appends source row IDs, refreshes FTS, and deletes superseded rows. Rewriting
a vague row into a clearer row remains open and may only use facts present in
the source rows.

## Evidence

Every merge result reports changed row IDs, source row IDs, and the reason.
No product-facing message may claim semantic pruning unless the store changed.
