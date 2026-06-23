# Known Failures

## Purpose

Record the generated-output failure classes represented by this diagnostic
workspace.

## Failure Classes

- Generic leaf prose that describes a role instead of answering the owner
  objective.
- README topology treated as completion evidence without semantic content.
- Mixed objectives under one root without a shared owner request.
- Verification claims copied into leaves without a tool result for that leaf.
- Catalog presence treated as artifact readiness.

## Required Repair Evidence

A repaired artifact needs an objective-specific root, concrete leaf content,
current catalog coverage, an audit id for the current artifact ledger head, and
explicit weak-path count of zero.
