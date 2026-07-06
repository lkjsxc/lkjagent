# Live Proof

## Purpose

Define the Aurora Ledger live endpoint proof and capture rules.

## Success Criteria

A baseline live proof passes only when all of these are true:

- the requested root is `stories/aurora-ledger`;
- chapter files `chapter-01.md` through `chapter-10.md` exist under the
  requested manuscript directory;
- measured manuscript words satisfy `objective.total-words=10000` or more;
- the task state is `closed` through passed engine checks;
- no human intervention occurs between send and terminal task state;
- a proof bundle is captured under `tmp/`.

## Extended Story Proof

After state-ledger parity is complete, run an unattended structured-story proof
for about ten hours. The objective asks for recursively expanding story arcs with
measured targets near 10000, 40000, 160000, then 640000 words. The run must use
small checked units, deterministic assembly, fresh artifact checks, status logs,
and a proof bundle. If the endpoint or checks block before ten hours, capture the
blocked state honestly instead of restarting silently.

## Bounded Trial Capture

Before long proofs, run bounded trials of about 15 minutes when an endpoint is
available. Store command output, prompts, summaries, proof bundles, and rejected
ideas under `tmp/live-runs/<stamp>/`. Commit only evidence that is useful and
free of secrets.

## Command Shape

The operator starts the daemon, sends the Aurora Ledger or recursive story
objective, watches status until terminal state or the bounded proof window ends,
then runs proof collection. Endpoint credentials come from the environment and
are never committed.

## Honest Failure

Anything less than the criteria above is a failed proof. The ledger records the
exact failing criterion, captures fixtures, and names the next fix task. A
blocked task with precise evidence is an acceptable outcome; fake completion is
not.
