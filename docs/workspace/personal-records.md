# Personal Records

## Purpose

Define record families for daily life while keeping the ledger as authority.

## Families

Known personal `kind` values include:

- `journal`: dated owner notes, reflections, and status reports;
- `calendar`: events, appointments, deadlines, and reminders;
- `todo`: tasks with state, due dates, blockers, and evidence;
- `finance`: budget entries, bills, receipts, and reviews;
- `note`: general notes and long-lived knowledge;
- `contact`: owner-provided people and organizations;
- `reference`: owner-provided external facts and citations;
- `routine`: recurring checklists or scheduled workflows.

## State Links

Records may create state keys such as `todo:open/<id>`, `calendar:due/<id>`,
`finance:review/<month>`, or `routine:ready/<id>`. These keys inform selector
candidates, but a turn still executes only after one `RuntimeDecision` is
persisted.

## Evidence

Personal records can reference owner messages, artifacts, checks, and other
records. Sensitive owner data is never admitted to prompts unless the selected
decision and tool policy allow it.
