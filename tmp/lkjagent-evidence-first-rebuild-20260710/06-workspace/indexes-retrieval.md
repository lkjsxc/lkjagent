# Indexes And Retrieval

## Derived Indexes

Maintain bounded, correctly filtered pages:

- today: records effective on the owner-local date;
- open todos: only open and actionable items;
- agenda: future and active calendar items;
- finance month: entries for one semantic month;
- active projects: non-archived projects with current work;
- recent changes: changed managed documents;
- proof runs: current acceptance and maintenance evidence.

Shard pages by date, project, topic, or status before they exceed the document
budget.

## Search Projection

Index managed Markdown bodies, titles, dates, tags, project, status, links, and
fingerprints. Support exact path, lexical, trigram, date, project, kind, and
status filters. SQLite search is rebuildable from workspace files.

## Retrieval

Retrieve relevant body excerpts rather than the five newest metadata rows.
Validate fingerprints immediately before prompt admission. Rank after discovery,
not before body search.

## External Changes

Incremental scanner detects new, changed, moved, and deleted files. Valid managed
files update projections. Ambiguous or malformed files enter inbox or quarantine
with a visible reason.

Use debounced stable size, modification time, and fingerprint reads, plus
periodic full reconciliation for dropped watcher events. Mandatory document IDs
distinguish moves from delete/create and make duplicate-ID conflicts
deterministic.

Valid large owner source files remain in place and are indexed through bounded
excerpts. Malformed files presented as managed records are excluded from managed
admission and receive a diagnostic under system/quarantine; move original bytes
only through an explicit import operation.

## Validation

Compare index membership with semantic predicates and current files. Missing,
stale, duplicate, or wrongly classified entries fail workspace validation.
