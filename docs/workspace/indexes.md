# Indexes

## Purpose

Define useful navigation, body search, staleness, and rebuild behavior.

## Navigation

Derived pages include today, open TODOs, agenda, finance month, active projects,
recent changes, and proof runs. Shard by date, project, topic, or state before a
managed page exceeds its token budget.

Each page records generation time, input document IDs and revision fingerprints,
and stale reason. It owns no source fact and cannot satisfy an obligation while
stale.

## Search

Index current Markdown bodies, titles, dates, kinds, project, state, relations,
and revision fingerprints. Support exact document and path lookup, lexical and
trigram body search, and date, project, kind, and state filters. Search rows are
unique by document, revision, field, and byte range. Filters compose with AND;
lexical ranking uses deterministic BM25 ties broken by document ID and chunk
range.

## Retrieval

Discover before ranking. Retrieve relevant bounded UTF-8 body excerpts rather
than a fixed recent metadata window. Validate the current bytes and revision
fingerprint before prompt admission; exclude drifted rows and record selected
and excluded source refs.

## External Changes

A debounced scanner detects stable new, changed, moved, and deleted files.
Valid managed files update document and search projections. Large owner source
stays in place. Malformed managed content receives a visible import-review or
quarantine diagnostic without overwriting original bytes.

## Gate Coverage

Before workspace-retrieval-maintenance evidence can pass, its named Docker gate
must execute nonempty behavioral suites for body discovery before ranking,
filters, bounded excerpts, fingerprint drift, external edits, index predicates,
rebuild equivalence, archive, and rebalance compensation. A generated fixture,
zero-test filter, or summary line is not evidence.

## Validation

Compare index membership and search fingerprints with current documents. Missing,
stale, duplicate, wrongly classified, or unresolved debt rows fail workspace
validation. Rebuilding the same source inventory twice produces identical
canonical rows, rankings, excerpts, and fingerprints; generated timestamps are
not search inputs. All projections are rebuildable from source files and
revisions.
