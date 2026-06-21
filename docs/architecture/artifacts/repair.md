# Repair

## Purpose

Define repair after audit or adoption finds gaps.

## Contract

Repair is a bounded plan over exact paths. It may add missing README indexes,
fix local links, write a missing manifest, split sequence-only files into
semantic files, or request a bounded content batch for weak leaves.

Repair never invents a completion claim. It records changed paths and then
requires a new audit before completion evidence can pass.

## Repair Card

Failed audit and repair output must include a copyable card:

```text
mission: ArtifactContentRepair
artifact_id: current semantic artifact id
root: artifact root
weak_paths:
- path: missing requirement labels
required_next_tool: artifact.next or fs.batch_write
next_valid_action: one admitted action block
blocked_completion: missing evidence and audit blockers
```

## Next Action

The next action must be admitted by the same active policy, usually
`artifact.next`, `fs.batch_write`, `doc.audit`, or `artifact.audit`.
`graph.recover` guidance is derived from the currently admitted tools and plan
state; it does not name `graph.plan` after the plan is ready or when that tool
is blocked.

## Invariants

- Repair targets exact paths and exact missing fields.
- Repair can use bounded write batches for weak leaves.
- Recovery mode must admit repair tools when content is missing.
- Completion stays blocked until repair output is audited.
- Repair persists progress after each file or batch.
- Readiness evidence is not created from planning evidence.

## Failure Cases

- Repair recommends `fs.batch_write` while active policy blocks it.
- Recovery loops over graph inspection instead of writing missing content.
- A repaired file is not re-audited before completion.
- Weak cookbook paths are rewritten with generic scaffold prose.

## Verification

Repair tests assert failed audits produce an admitted next action, weak paths
are exact, completion remains refused until repaired content passes audit, and
recovery never blocks `artifact.next`, `artifact.audit`, `doc.audit`,
`fs.batch_write`, `fs.write`, `fs.read`, or `fs.tree` when needed.

## Related Files

- [write-batches.md](write-batches.md)
- [audit.md](audit.md)
- [completion-gates.md](completion-gates.md)
