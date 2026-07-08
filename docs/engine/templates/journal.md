# Journal Template

## Purpose

Define personal record capture as plain workspace file writes.

## Selection

The deterministic router selects this template for journal, schedule, todo, and
small daily record requests when the owner intent is explicit enough to write
without clarification.

## Shape

- Journal entries append or create dated Markdown under `records/life/journal/YYYY/MM/DD/entry.md`.
- Todo entries write under `records/life/todo/open/` with open state.
- Calendar-like entries write under `records/life/calendar/YYYY/MM/DD/`.
- Finance entries write under `records/life/finance/YYYY/MM/`.
- Each write records a workspace metadata row, fingerprint history, and index
  staleness event.

## Checks

The completion check verifies the file exists under the workspace root, the
body contains the owner-provided fact, and the fingerprint matches the ledger
row. No model call is needed for explicit record capture.
