# Live Proof

## Purpose

Define the Aurora Ledger live endpoint proof and capture rules.

## Success Criteria

A live proof passes only when all of these are true:

- the requested root is `stories/aurora-ledger`;
- chapter files `chapter-01.md` through `chapter-10.md` exist under the
  requested manuscript directory;
- measured manuscript words satisfy `objective.total-words=10000` or more;
- the task state is `closed` through passed engine checks;
- no human intervention occurs between send and terminal task state;
- a proof bundle is captured under `tmp/`.

## Command Shape

The operator starts the daemon, sends the Aurora Ledger objective, watches
status until terminal state, then runs proof collection. Endpoint credentials
come from the environment and are never committed.

## Honest Failure

Anything less than the criteria above is a failed proof. The ledger records the
exact failing criterion, captures fixtures, and names the next fix task. A
blocked task with precise evidence is an acceptable outcome; fake completion is
not.
