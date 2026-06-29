# Root Repair

## Purpose

Define the route from missing-root observations to root identity writes.

## Route

A missing root follows this route:

```text
doc.audit or artifact.next observes root missing
-> RuntimeEvent::ArtifactRootMissing
-> Obligation::RootIdentity
-> ResolverPlan::SemanticWriteContract
-> fs.batch_write
-> doc.audit
```

The route never sends the same missing-root fact back to `doc.audit`. Audit is
useful after write progress, not as repeated recovery for the same absent root.

## Admission

While the latest fact digest still says root missing, same-root `doc.audit` is
blocked and `fs.batch_write` is admitted with the stored root identity contract.
If the model cannot produce a valid contracted batch after the retry budget, the
runtime records a blocked handoff with exact missing paths and contract limits.

## Evidence

Root repair creates files only. It does not directly record
document-structure or artifact-readiness evidence. Those requirements remain
pending until `doc.audit` and `artifact.audit` observe passing facts for the
current root.
