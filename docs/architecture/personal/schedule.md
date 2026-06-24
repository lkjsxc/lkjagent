# Schedule Records

## Purpose

Define timezone-aware schedule items.

## Contract

A schedule item requires a title and `start_at`. `end_at` is optional, but when
present it must be after `start_at`. Date-times use RFC3339 with timezone
offsets. The store may also keep the owner timezone name that produced the
value.

Do not hard-code the owner's timezone. Configuration owns the default owner
timezone. Tests use `Asia/Tokyo` as a fixture because it proves offset-aware
behavior without assuming every owner uses that zone.

## Status

Schedule status is typed: `open`, `doing`, `waiting`, `done`, or `canceled`.
Conflict detection is advisory unless the task contract says a conflict is a
hard refusal. Observations name overlaps compactly.

## Recurrence

Recurrence starts as a small documented grammar or an RFC5545-like string.
Unsupported recurrence text is refused before mutation. Expanding recurrence
for listing must stay bounded by the requested date range and limit.
