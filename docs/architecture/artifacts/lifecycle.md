# Lifecycle

## Purpose

Define the ordered states for semantic content artifacts and the evidence that
moves an artifact between states.

## States

An artifact moves through these states in order:

```text
OwnerObjective -> ArtifactIdentity -> ArtifactPlan -> WriteContract
-> ModelAuthoredBatch -> DocumentAudit -> ArtifactAudit
-> WeakPathCursor -> MoreWriteContracts -> Verification -> CompletionGate
```

## State Meaning

- `ArtifactIdentity`: durable artifact id, short root, kind, and full owner
  wording exist.
- `ArtifactPlan`: intended sections, scale, assumptions, and risks exist.
- `WriteContract`: exact paths, limits, required sections, and forbidden weak
  phrase classes are recorded.
- `ModelAuthoredBatch`: the model authored a line-protocol `fs.batch_write`
  action that matched the stored contract.
- `DocumentAudit`: document topology passed from `doc.audit`; it proves root
  shape, README coverage, manifest presence, links, and path hygiene only.
- `ArtifactAudit`: semantic readiness passed from `artifact.audit`; it proves
  profile-specific content for the current artifact id.
- `WeakPathCursor`: weak paths remain and cursor state names the next contract.
- `Verification`: required verification gates passed and named the artifact or
  root.
- `CompletionGate`: the central runtime gate accepted the evidence.

Scaffold is never content evidence for cookbooks, stories, guides, knowledge
bases, or long reports. Prompt-visible scaffold writers are not live tools.
Only audits move audit-owned evidence, and only contract-matching writes move
cursor paths to written. A structure pass cannot satisfy artifact readiness;
readiness comes from ledger-tied semantic audit evidence.

## Evidence Owners

- `artifact.plan` owns planned identity.
- `artifact.next` owns write-contract and cursor facts.
- `fs.write` and `fs.batch_write` own written-path observations after contract
  validation.
- `doc.audit` owns document-topology evidence.
- `artifact.audit` owns artifact-readiness evidence after kind resolution from
  the ledger or root.
- `verify.cargo`, `verify.xtask`, and Docker Compose gates own verification
  evidence.
- `agent.done` owns no evidence; it only requests completion admission.

## Completion Rule

Completion requires every required artifact to be `Verified` or to have an
explicit completion reducer allowance that cites equivalent evidence. Missing
recovery faults, pending compaction cursors, queued owner work, unresolved weak
paths, or missing verification gates block completion.

## Runtime

The graph routes large content work to artifact planning, scaffold or adoption,
bounded content batches, audit, repair, verification, and completion. A failed
state names exact missing paths and the next executable action. Weak paths carry
one deterministic repair contract at a time; broad rewrite prompts are not
repair evidence.

## Status

implemented for the daemon close path and focused artifact profiles. Scaffold,
write-path recording, audit output, weak-path tracking, ledger-backed readiness,
and artifact-aware completion refusal exist. Broader profile coverage remains
incremental work.
