# Diary Records

## Purpose

Define append-friendly date-scoped diary entries.

## Contract

A diary entry records a date, title, body, tags, source case ID, created time,
and update time. The date is a local calendar date. If the model omits the date,
the runtime may use the current owner-local date only when the configured owner
timezone exists.

Diary entries are append-friendly. Amendments create record-event rows with a
summary of the change. The current body may be updated, but the event history
must preserve that an amendment happened.

## Status

Diary status is `open` while it is active and `done` when the owner or task marks
it complete. Diary entries do not require due dates, end dates, recurrence, or
priority.

## Projection

Generated journal files group entries by year, month, and date. Each rendered
entry names the stable record ID so later amendments target the store record,
not the Markdown location.
