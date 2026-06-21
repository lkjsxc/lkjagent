# Runtime Authority

## Purpose

Define the runtime-owned authority layer that decides active mission, tool
admission, compaction ownership, maintenance eligibility, and completion
closure.

## Contract

Runtime authority is the only source of action truth. Model output is intent.
Graph transitions are guidance. Context pressure, maintenance ticks, verifier
results, and completion requests are events. The authority reducer turns those
facts into one decision before effects run.

## Invariants

- The reducer has no I/O.
- Every decision names active mission, admitted tools, blocked tools, missing
  evidence, and next valid action.
- Completion must pass the same gate on every close path.
- Graph policy cannot trap repair, audit, or compaction.

## Failure Cases

- A graph completion node blocks `fs.read`, `artifact.audit`, or repair tools.
- Recovery refuses the exact tool needed to leave recovery.
- Maintenance runs while owner work or verification is pending.
- Compaction depends on model-authored `memory.save`.

## Verification

Authority is verified by pure reducer tests, dispatcher admission tests,
completion-refusal tests, compaction snapshot tests, and uploaded run-log
benchmark fixtures.

## Table of Contents

- [reducer.md](reducer.md): pure snapshot, event, and decision contract.
- [missions.md](missions.md): owner, recovery, verification, maintenance, compaction, and idle rules.
- [turn-authority.md](turn-authority.md): single turn decision object.
- [mode-priority.md](mode-priority.md): deterministic mission priority order.
- [tool-admission.md](tool-admission.md): explainable admission and refusal data.
- [tool-policy.md](tool-policy.md): admitted and blocked tool classes.
- [evidence-policy.md](evidence-policy.md): evidence ledger requirements.
- [completion.md](completion.md): close eligibility and refusal contract.
- [completion-policy.md](completion-policy.md): close gate inputs and refusal output.
- [recovery-policy.md](recovery-policy.md): fault recovery ownership.
- [maintenance.md](maintenance.md): idle-only maintenance policy.
- [maintenance-policy.md](maintenance-policy.md): maintenance eligibility and effect rules.
- [compaction.md](compaction.md): runtime-owned snapshot and resume contract.
- [compaction-policy.md](compaction-policy.md): hard and soft compaction authority.
- [exact-examples.md](exact-examples.md): schema-rendered valid action examples.

## Related Files

- [../active-mode/README.md](../active-mode/README.md)
- [../../state-graph/README.md](../../state-graph/README.md)
- [../../artifacts/README.md](../../artifacts/README.md)
