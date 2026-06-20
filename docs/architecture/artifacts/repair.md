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
