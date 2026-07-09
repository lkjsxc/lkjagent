# Daily Record Schemas

## Common Form

Every agent-managed Markdown file follows `record-contract.md`, then a
kind-specific attribute-free block, then concise prose. Unknown fields are an
import error. Dates use ISO local dates; instants use RFC 3339 with offset.
Document identity does not change when status moves the path.

## TODO

Path is `life/todo/<open|waiting|done>/YYYY/MM/<document-id>.md` using creation
month. The block contains exactly `status`, `priority`, `created`, and optional
`due`, `waiting_for`, `completed`. Priority is low, normal, high, or urgent.
Open cannot have completed; waiting requires waiting_for; done requires
completed. Prose has `## Task`, optional `## Notes`, and `## History` entries
linked to activity or operation refs.

## Calendar

Path is `life/calendar/YYYY/MM/DD/<document-id>.md` using local start date. The
block contains `start`, `end`, `timezone`, `all_day`, optional `location`, and
optional `recurrence`. End must follow start. All-day values are local dates;
timed values are offset instants plus a valid IANA zone. Recurrence stores a
bounded normalized rule and original owner phrase. Prose has `## Event` and
optional `## Notes`.

## Finance

Path is `life/finance/YYYY/MM/entries/<document-id>.md` using transaction date.
The block contains `date`, `direction`, `amount`, `currency`, `account`,
`category`, and optional `counterparty`. Direction is income, expense, or
transfer. Amount is a positive base-10 decimal string, never binary floating
point; currency is an uppercase ISO code. Transfers name both accounts and do
not count as income or expense. Prose has `## Transaction` and optional
`## Notes`; derived summaries never own transaction facts.

## Note And Journal

Notes live at `life/notes/<topic>/YYYY/MM/<document-id>.md` and contain `topic`
and optional `project_id`, followed by `## Note` and `## Sources`. Journals live
at the exact path in `diary.md`; their prose is model-composed, source-linked,
and fact/reflection separated. Neither body copies the capture command merely
because it was the triggering activity.

## Project Decision And Session

Project notes live below `projects/<project-id>/notes/YYYY/MM`. Decisions live
below `projects/<project-id>/decisions/YYYY/MM` with `status`, `decided`, and
`supersedes`, then `## Decision`, `## Rationale`, and `## Consequences`.
Sessions live below `projects/<project-id>/sessions/YYYY/MM/DD` with `started`,
`ended`, and `outcome`, then `## Goal`, `## Work`, `## Checks`, and `## Next`.
All project records carry the stable project ID.

## Activity

Activity records preserve owner and agent conversation truth separately from
semantic records under `activity/YYYY/MM/DD`. A long turn is split into ordered
immutable parts, each at most 512 conservative tokens, with one activity ID and
part indexes. Semantic records cite the activity ID but do not repeat raw text.

## Transitions And Indexes

Every transition is an expected-fingerprint move plus revision, relation, and
index update. Index membership is derived from kind fields: status, effective
date, project, account, category, topic, or source. Invalid records remain in
place, become diagnostics, and never silently overwrite valid source facts.
