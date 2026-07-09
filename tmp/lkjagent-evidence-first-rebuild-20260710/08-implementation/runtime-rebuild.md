# Runtime Rebuild

## Task A: Domain Types

Add Matter, Obligation, OperationSpec, OperationState, WakeCondition,
FailureLineage, ProgressVector, and CompletionPredicate to the pure core. Use
small files and validate constructors.

## Task B: Event Reducer

Implement pure reduction for intake, graph edits, operation leases, effects,
observations, checks, recovery, waiting, invalidation, and completion. Add
property tests for determinism and invalid-state rejection.

## Task C: Native Store

Create native tables for matters, obligations, operations, events, decisions,
effect journal, checks, conversation messages, and projections. Write atomic
transaction APIs. Do not extend task and step tables.

## Task D: Selector

Build candidates from reduced state, edges, due time, and budgets. Make blocked
dependencies produce explicit wait or report candidates. Add fake-clock tests.

## Task E: Decision

Persist the complete decision before prompt or effect. Bind context, tools,
grammar, budgets, checks, idempotency, and exit predicate.

## Task F: App Switch

Route new owner turns directly into native matters. Run the daemon only through
native selection. Prove a matter progresses with task and step tables absent.

## Task G: Remove Old Authority

Delete production reads and writes for templates, plan parser, task snapshots,
task rows, step rows, bridge projection, and special finish. Keep only redacted
fixtures outside product code.
