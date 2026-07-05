# Indexes

## Purpose

Define generated workspace views and how they stay honest.

## Index Files

Generated index files live under `workspace/indexes/`. Typical examples include
`today.md`, `agenda.md`, `open-todos.md`, `active-projects.md`, and
`budget-month.md`.

## Required Metadata

Each generated index records:

- generation time;
- input record ids;
- input artifact fingerprints;
- producing decision id;
- check results or evidence refs; and
- stale reason when inputs changed after generation.

## Rebuild Rule

Indexes are derived views. They can be deleted and rebuilt from records,
artifacts, and ledger rows. `lkjagent workspace --rebuild` writes the first
record-backed indexes and records index artifacts. A stale index may still be
shown to the owner with a warning, but it must not satisfy completion checks or
be admitted to prompts as current evidence.

## Search Rule

Full-text search may index records and artifacts, but every search result shown
to the model must be admitted as bounded context with source refs, fingerprint,
trust class, contamination class, and staleness status.
