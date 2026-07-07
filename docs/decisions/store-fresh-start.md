# Store Fresh Start

## Purpose

Record the decision to start the SQLite schema fresh.

## Context

The product owns one local data directory and one SQLite store. The target
schema is small and every table has a named reader.

## Decision

Existing store files are not converted. The daemon creates the schema in
`data/lkjagent.sqlite3` and ignores unrelated database files.

## Consequences

Runtime evidence needed for evaluation is preserved through fixtures and proof
bundles, not schema conversion. Operators who need prior workspace files keep
the workspace directory separately.

## Rejected Alternatives

Converting previous tables would carry unused categories into the product and
make the store harder to audit.
