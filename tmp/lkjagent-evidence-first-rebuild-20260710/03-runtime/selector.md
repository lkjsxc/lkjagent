# Selector

## Candidate Construction

Build candidates from active state cells and graph edges. Candidate families
include owner intake, clarification, native effect, model content, tool action,
observation, check, recovery, report, timed wake, and maintenance.

## Eligibility

A candidate is eligible only when dependencies pass, cooldown is due, budgets
remain, required context is available, and no material conflict blocks it.
Ineligible work produces an explicit wait or blocked-report candidate with a
wake condition; it never falls through to idle.

## Priority

Use deterministic ordering:

1. settle interrupted effects;
2. deliver owner answers and cancellations;
3. repair safety or consistency faults;
4. continue active owner work;
5. verify completed effects;
6. report verified completion;
7. start queued matters;
8. run due deterministic maintenance.

Ties use persisted priority, causal sequence, and stable operation ID.

## Quiescence Test

The daemon is quiescent only when no eligible candidate, due wake, pending
owner turn, interrupted effect, or due maintenance exists. Quiescence is derived
and not stored as a closed matter.
