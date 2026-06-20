# Semantic Merge

## Purpose

Define non-exact memory merge behavior.

## Candidate Search

Merge candidates are selected by kind, normalized title, tag overlap, and
content shingle overlap. Punctuation-heavy owner queries must be sanitized so
FTS search does not fail before candidate ranking.

## Merge Rules

Semantic merge preserves real facts and source row IDs. It may rewrite a vague
row into a clearer row only when the source rows contain those facts. It may
delete superseded rows only through a real store operation.

## Evidence

Every merge result reports changed row IDs, source row IDs, and the reason.
No product-facing message may claim semantic pruning unless the store changed.
