# Tool Admission

## Purpose

Define decision-bound validation, effect dispatch, and bounded observations.

## Binding

Admission receives the persisted decision spec and parsed envelope. It accepts
only that decision's grammar, visible descriptor, exact fields, field bounds,
state affordances, budgets, and workspace policy. Rejection records a typed
reason and creates no effect row.

Dispatch receives only an admitted stable effect key and typed values. A
separate switch cannot grant a tool omitted by the registry.

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
row commit before filesystem mutation. Read-only operations also persist
admission and observation when their output affects later work.

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
