# Personal Records

## Purpose

Define the common record ledger used by diary entries, schedule items, and
TODO items.

## Canonical Store

SQLite owns structured state. Every personal record has:

- stable integer ID.
- kind: `diary`, `schedule`, or `todo`.
- title and body text.
- typed status.
- tags stored in normalized text form.
- source case ID when a graph case created or changed the record.
- created, updated, and optional closed timestamps.

Record events append history. Updates never silently erase the fact that a
record changed. Links connect records with a typed relation.

## Shared Tables

The store owns records, record events, record links, and FTS rows. New SQL stays
inside `lkjagent-store`. Runtime, tools, CLI, and graph crates use typed store
APIs and do not embed personal-record SQL.

## Validation

Validation runs before mutation. Empty titles, invalid timestamps, unknown
statuses, malformed tag fields, impossible time ranges, and unsupported
recurrence strings are refused without partial writes.

## Evidence

Completion for a personal-record task requires store-backed evidence such as a
created ID, update event, listing row, or search result. Workspace files alone
cannot satisfy personal-record completion.
