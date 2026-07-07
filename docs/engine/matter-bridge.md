# Matter Bridge

## Purpose

Define how transitional plan-family rows map into semantic matters without
remaining the owner-visible product model.

## Bridge Rows

Existing plan-family tables may store ordered bodies, checks, attempts, and
continuity evidence. New runtime selection treats those rows as evidence that
projects into state cells and relation edges.

## Semantic Surface

The owner sees matters, records, artifacts, decisions, events, and proof.
Numeric bridge row ids may appear in developer proof refs, but they are not the
primary owner interface.

## Projection

A bridge projection creates or updates:

- `matter:snapshot/<id>` cells for summary and lifecycle state;
- operation cells for pending model calls, effects, checks, or recovery;
- record or artifact refs for generated files;
- relation edges for blockers, dependencies, and check evidence.

Projection is idempotent. If a state cell already owns the active operation,
selectors use that cell and do not recreate bridge work.

## Removal Direction

New features should write semantic rows directly. Bridge code may remain only to
read existing bridge data, compare behavior, and prove continuity while semantic
rows take over.

## Failure This Prevents

Old row storage can continue to hydrate evidence without making every owner turn
look like a numbered job.
