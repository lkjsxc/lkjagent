# Obligations

## Purpose

Define the obligation set selected from runtime facts.

## Obligation Set

`Obligation` is one of:

- `Plan`: the case lacks plan evidence.
- `ArtifactIdentity`: the artifact kind, root, or owner identity is missing.
- `RootIdentity`: the root is missing, empty, or identity-incomplete.
- `DocumentStructure`: topology, links, path hygiene, or line limits are failed
  and no root identity write is pending.
- `ContentBatch`: a stored write contract has exact paths ready for content.
- `ArtifactReadiness`: semantic artifact audit evidence is missing or failed.
- `Verification`: verification evidence is missing or failed.
- `Completion`: the owner requested closure and all gate facts must be checked.
- `Recovery`: a fault, anomaly, stale action, or exhausted route requires repair.
- `Compaction`: context pressure requires a runtime-owned compact effect.
- `BlockedHandoff`: no safe resolver remains and exact blockers must be stored.

## Priority

Hard compaction and recovery still preempt normal owner work. Within owner work,
root identity outranks document structure. A missing root is therefore not a
document-audit obligation; it is a root identity write obligation.

## Audit-Owned Evidence

`document-structure` and `artifact-readiness` are audit-owned requirements.
Direct graph evidence cannot satisfy them. The resolver may record direct graph
evidence only for requirements whose owner is graph evidence.

## Completion

Completion remains blocked until the current artifact root has audit facts and
verification facts. Planning evidence, direct graph evidence for audit-owned
requirements, scaffold-only content, or a missing audit never closes a case.
