# Tool Admission

## Purpose

Define decision-bound validation, effect dispatch, and bounded observations.

## Binding

Admission receives the persisted decision spec and parsed envelope. It accepts
only that decision's grammar, visible descriptor, exact fields, field bounds,
state affordances, budgets, and workspace policy. Rejection records a typed
reason and creates no effect row.

Dispatch receives only an admitted stable effect key and typed values. A
separate switch cannot grant a tool omitted by the registry. `write_record`
persists the common `workspace.record` effect path, then dispatches one exact
shape: journal or memory with no report optionals, short report with no
optionals, long report map with canonical `slug`, `unit=index`, ordered unique
`children`, and positive bounded `minimum_words`, or long child with the same
slug plus one semantic non-index `unit`. Every other family or mixed shape is
rejected before an effect row or filesystem mutation. The model supplies no
record path, date, frontmatter, or section links.

## Path And Revision Guards

Paths are normalized relative names opened through the workspace capability.
Reject traversal, absolute names, symlinks, special files, reserved internal
names, and case ambiguity.

Edit requires the latest successful read for the same path and revision. Create
requires observed absence. Admission checks zero/multiple exact matches,
unchanged replacement, output size, allowed changed paths, and matter/effect
budgets.

## Effect Boundary

Accepted effect admission, exact target revisions, stage metadata, and journal
row commit before filesystem mutation. A missing-parent record create lists the
file first and every absent parent as `mkdir` targets in that transaction. Only
then may the no-follow `mkdirat` edge create and fsync those directories.
Read-only operations also persist admission and observation when their output
affects later work.

## Observation

Exactly one bounded immutable observation settles each attempted effect. It
stores decision/admission/effect identity, status, source path/revision,
truncation/continuation, receipt fingerprint, contamination class, and timestamp.

The next prompt renders an observation once. Failed output bodies and raw state
JSON do not become context.

## Final Admission

Final wording is validated against current checked paths and receipts. Unsupported
path, command, effect, or check claims and readiness phrases are rejected. A
native factual receipt remains available after bounded wording faults.
