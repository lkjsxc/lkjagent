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

Index visible Markdown bodies, titles, dates, kinds, project, state, relations,
and revision fingerprints. Body and title chunks contain at most 2,048 UTF-8
bytes and overlap by 128 bytes so boundary terms remain searchable. Support exact
document and path lookup, lexical and trigram search, and
date, project, kind, and state filters. Search rows are unique by document,
revision, field, and byte range. Filters compose with AND; BM25 ties break by
document ID, field, byte range, and chunk ID.

## Retrieval

Discover before ranking. Retrieve relevant bounded UTF-8 body excerpts rather
than a fixed recent metadata window. Validate current bytes before admission;
paginate past drifted rows until 50 current hits or exhaustion, and record
selected and excluded source refs.

## External Changes

Daemon entry and explicit rebuild perform a sorted complete reconciliation that
detects new, changed, moved, and deleted Markdown. It scans root Markdown plus
`inbox/`, `life/`, `knowledge/`, `projects/`, `artifacts/`, and `activity/`, while
excluding hidden paths and the root `indexes/`, `archive/`, and `system/` trees.
Descriptor-relative no-follow reads exclude symlinks. Valid managed files update
document and search projections in the same transaction; generic Markdown gets
a deterministic path identity. Missing managed sources become archived `missing`
projections, known malformed sources become archived `import-review` projections,
and restoration reactivates source metadata. Daemon reconciliation marks
navigation stale after managed changes; explicit rebuild regenerates and clears
that marker. Malformed managed-looking files create durable active state diagnostics;
repair or removal resolves them. Daemon entry compares a durable sorted
path-size-modification manifest: unchanged inventory skips projection writes,
while changed metadata reconciles immediately. Explicit rebuild always scans.
Large owner source stays in place.

## Gate Coverage

Before workspace-retrieval-maintenance evidence can pass, its named Docker gate
must execute nonempty behavioral suites for body discovery before ranking,
filters, bounded excerpts, fingerprint drift, external edits, index predicates,
rebuild equivalence, archive, and rebalance compensation. A generated fixture,
zero-test filter, or summary line is not evidence.

## Validation

Compare index membership and search fingerprints with current documents. Missing,
stale, duplicate, wrongly classified, or unresolved debt rows fail workspace
validation. Chunks are at most 2,048 bytes and rendered excerpts at most 240
bytes. Rebuilding the same source inventory twice produces identical canonical
rows, rankings, excerpts, and fingerprints; generated timestamps are not search
inputs. All projections are rebuildable from source files and revisions.
