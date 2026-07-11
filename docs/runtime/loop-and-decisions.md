# Loop And Decisions

## Purpose

Define the bounded daemon cycle around the canonical reducer and selector.

## Cycle

One cycle performs these ordered actions:

1. read the durable snapshot and one next event or wake source;
2. reduce the event into validated state and append its transition record;
3. derive feasible operation candidates from current cells and typed edges;
4. select one candidate by deterministic policy;
5. persist its `RuntimeDecision` and fingerprints;
6. compile a prompt or prepare one native effect when required;
7. execute at most one endpoint call or effect;
8. convert the outcome into a new runtime event;
9. settle observations, checks, messages, and decision status.

The next cycle starts from committed rows. At most one endpoint call occurs per
cycle. Deterministic intake, inspection, check, recovery, and maintenance may
require none. The per-invocation cycle bound is not a matter-work bound: open
state resumes from durable rows across invocations. Useful progress may span
more than four decisions and ends only through checks, a wait, or a blocker.

## Selection

Candidates carry dependency, budget, context, admission, conflict, cooldown,
and wake eligibility. Ineligible work records a specific blocker. Selection
uses stable priority, causal sequence, and operation ID, independent of row
insertion order.

## Prompt Boundary

Prompt compilation begins only after the decision exists. It derives prompt
state, bounded context lanes, a small tool view, one envelope grammar, output
reserve, and recovery instructions from decision-bound facts. Compiler output
fingerprints are written back without changing the selected operation.

## Native Effects

Harness-selected native effects use the same admission validator and journal as
model-proposed effects. Read-only inspection still records an admission and
observation when its result can influence later work.
