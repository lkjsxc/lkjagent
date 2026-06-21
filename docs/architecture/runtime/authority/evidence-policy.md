# Evidence Policy

## Purpose

Define the evidence ledger that authority uses before progress, repair, and
completion decisions.

## Decision Owner

Runtime authority owns evidence classification. Graph evidence records may
supply raw facts, but authority decides whether a requirement is satisfied.

## Inputs

Evidence policy reads required evidence, observed tool results, artifact audit
results, document audit results, verification output, recovery faults, and
unsupported completion claims.

## Output

The output separates present evidence, missing evidence, stale evidence,
contradicted evidence, and the next tool that can produce the highest-priority
missing fact.

## Requirements

Artifact tasks require plan evidence, observation evidence, artifact identity,
structure audit, content readiness, verification evidence, and a completion
summary tied to observed effects. Document-structure evidence is satisfied by
`doc.audit`; artifact-readiness evidence is satisfied by `artifact.audit`.
Recovery tasks require the current fault class, retry count, last invalid
action, and admitted escape action.

## Prohibited States

- A scaffold, README, or manifest counts as content readiness.
- A verification note counts when the verified file or audit result is absent.
- A graph completion node overrides missing artifact evidence.
- Unsupported claims become completion evidence.

## Fixture

`cookbook_missing_evidence` and `false_completion_after_scaffold` prove evidence
gaps block completion and select repair.

## Verification

Run `cargo test -p lkjagent-runtime artifact_completion_gate`.

## Status

partially implemented.
