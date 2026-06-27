# Personal Operations Architecture

## Purpose

Define the structured personal-data domain for diary, schedule, and TODO work.
SQLite is canonical. Workspace Markdown is a bounded projection for reading,
not the source of truth for status, dates, recurrence, or IDs.

## Table of Contents

- [records.md](records.md): shared record identity, fields, events, and links.
- [diary.md](diary.md): date-scoped journal entries and amendments.
- [schedule.md](schedule.md): timezone-aware events, ranges, status, and recurrence.
- [todo.md](todo.md): typed TODO state, priority, due dates, and closure.
- [search.md](search.md): bounded listing and full-text search behavior.
- [materialized-views.md](materialized-views.md): generated Markdown views.

## Status

implemented. The store schema, tools, CLI inspection, and projection writers
are covered by focused tests and the final gates named in
[../../current-state.md](../../current-state.md).
