# Personal Operations

## Purpose

State the owner-visible behavior for diary, schedule, and TODO management.

## Owner Experience

The owner can ask lkjagent to record diary notes, add or update schedule items,
list upcoming events, add TODOs, update TODO status, and search personal records.
The daemon uses the same one-action protocol and fixed registry as every other
owner task.

## Source Of Truth

SQLite is canonical for personal records. Generated Markdown in `journal/`,
`schedule/`, and `todos/` is for reading and navigation. Status, recurrence,
due dates, and record IDs do not depend on parsing arbitrary Markdown.

## Bounded Output

List and search output is compact. The owner can narrow by kind, status, date
range, project, tags, or query. Large personal workspaces remain searchable
without flooding the model context.

## Completion

A personal-record task completes only when store-backed evidence exists: a
created record ID, an update event, a listing row, or a search result. The agent
must not claim a personal record was created from a projected file alone.
