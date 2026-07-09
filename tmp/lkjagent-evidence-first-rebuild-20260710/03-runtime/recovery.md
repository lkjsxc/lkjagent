# Recovery

## Recovery Record

Every failure records:

- fault class and normalized signature;
- decision, operation, state, and context fingerprints;
- attempted strategy and changed condition;
- bounded diagnostic, never the entire failed response;
- retry count for this lineage;
- next strategy, eligibility time, and remaining budget.

## Strategy Ladders

Examples:

- output limit: reduce unit size, continue from safe boundary, split semantic
  section, then replan artifact;
- parse: exact grammar repair, one concrete valid example, constrained grammar,
  then narrower output shape;
- admission: remove hidden tool, correct primitive, select deterministic target,
  then re-inspect;
- endpoint: retry with backoff, alternate sampling limits, smaller prompt,
  reconnect, then wait-external;
- effect: inspect current filesystem state, idempotent replay, compensate,
  quarantine, then owner-visible block;
- check: inspect measured failure, repair source, rerun check, then replan.

## No Repeat Rule

The tuple of operation, prompt fingerprint, tool view, budget, and failure
signature may not recur without a changed external condition. A no-progress
event forces the next ladder step.

## Blocking

Blocking is not silent terminal idle. It names exhausted strategies, preserved
partial work, next possible owner action, and whether a timed retry remains.
