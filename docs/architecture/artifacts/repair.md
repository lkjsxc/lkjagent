# Repair

## Purpose

Define repair after audit or adoption finds gaps.

## Contract

Repair is a bounded plan over exact paths. It may add missing README indexes,
fix local links, write a missing manifest, split sequence-only files into
semantic files, or request a bounded content batch for weak leaves.

## Limits

Repair does not invent a completion claim. It records changed paths and then
requires a new audit before completion evidence can pass.

## Next Action

Failed audit and repair output must include a copyable next action admitted by
the same active policy, usually `artifact.next`, `fs.batch_write`,
`doc.audit`, or `artifact.audit`.

## Invariants

- Repair targets exact paths and exact missing fields.
- Repair can use bounded write batches for weak leaves.
- Recovery mode must admit repair tools when content is missing.
- Completion stays blocked until repair output is audited.

## Failure Cases

- Repair recommends `fs.batch_write` while active policy blocks it.
- Recovery loops over graph inspection instead of writing missing content.
- A repaired file is not re-audited before completion.

## Verification

Repair tests assert failed audits produce an admitted next action and that
completion remains refused until the repaired content passes audit.

## Related Files

- [write-batches.md](write-batches.md)
- [audit.md](audit.md)
- [completion-gates.md](completion-gates.md)
