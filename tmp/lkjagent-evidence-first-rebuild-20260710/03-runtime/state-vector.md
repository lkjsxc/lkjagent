# State Vector

## Concurrent Dimensions

A matter may hold several active facts at once:

- lifecycle: open, waiting-owner, waiting-external, suspended, completing,
  completed, archived;
- phase: intake, frame, inspect, plan, act, observe, verify, recover, report;
- intent: converse, capture, retrieve, create, revise, organize, develop,
  administer;
- focus: personal, project, repository, artifact, system;
- obligations: unsatisfied, in-progress, satisfied, invalidated;
- failures: parse, admission, endpoint, effect, check, conflict, no-progress;
- resources: model budget, elapsed budget, retry budget, tool budget;
- maintenance: index, compact, validate, rebalance, archive.

These are cells with typed payloads and evidence, not one large closed enum.

## Derived Prompt State

Prompt state is a pure projection of selected cells. Examples include
capture-compose, project-inspect, artifact-write-section, verify-file,
recover-output-limit, and wait-owner-date. Each state must change behavior, not
only a label.

## State Validity

- Completed cannot coexist with unsatisfied required obligations.
- Recover requires an active failure and a named repair strategy.
- Waiting requires a question or wake condition.
- Action-select requires a model decision and a small tool view but no admission
  yet.
- Effect-ready requires one accepted admission, prepared effect intent, and
  idempotency key.
- Verify cannot write owner content.
- Quiescent is daemon-level and never a matter lifecycle cell.

## Transition Proof

Every transition names source event, old fingerprint, new fingerprint, guard,
and evidence. Production calls the transition validator before commit.
