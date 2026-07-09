# Personal Records

## Purpose

Define exact daily-life document families and local-time semantics.

## TODO

Use `life/todo/<open|waiting|done>/YYYY/MM/<document-id>.md` by creation month.
Fields are status, priority, created, and optional due, waiting-for, completed.
Waiting requires a reason; done requires completion time; open forbids it.

## Calendar

Use `life/calendar/YYYY/MM/DD/<document-id>.md` by local start date. Fields are
start, end, timezone, all-day, and optional location and recurrence. End follows
start. Timed values include an offset and IANA zone; all-day values are dates.

## Finance

Use `life/finance/YYYY/MM/entries/<document-id>.md` by transaction date. Fields
are date, direction, positive decimal amount, ISO currency, account, category,
and optional counterparty. Transfers identify both accounts and are not income
or expense.

## Notes And Journal

Notes live under `life/notes/<topic>/YYYY/MM/`. Journals use
`life/journal/YYYY/MM/DD/entry.md`. A journal composition decision retrieves
same-day admitted facts and the current entry, then writes known facts and
clearly separated reflection. It never copies the command or canned
missing-detail text.

## Dates And Changes

Relative dates normalize at intake with original owner wording preserved. A
state change uses expected fingerprint, appends an immutable revision, updates
path and relations, marks index debt, and records its operation event.
