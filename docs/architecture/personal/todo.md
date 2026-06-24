# TODO Records

## Purpose

Define typed task records for owner TODO management.

## Contract

A TODO requires a title. Details, due date, priority, project, and tags are
optional. Due dates use RFC3339 with timezone offsets when they include a time.
Date-only input may be normalized through the configured owner timezone.

## Status

TODO status is one of `open`, `doing`, `waiting`, `done`, or `canceled`.
Closing a TODO sets `closed_at` and appends a status event. Reopening clears
`closed_at` and appends another event.

## Priority And Project

Priority is typed and bounded. The initial priorities are `low`, `normal`,
`high`, and `urgent`. Project names are short owner-facing strings used for
lists and generated project views.

## Completion Evidence

A TODO task completes only after the store confirms the created or updated ID,
status, and changed fields. A Markdown checklist line is not canonical state.
