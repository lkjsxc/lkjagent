# Lifecycle

## Purpose

Define the ordered states for semantic content artifacts and the evidence that
moves an artifact between states.

## States

An artifact moves through these states in order:

```text
Planned -> Scaffolded -> ContentWritten -> LocallyAudited -> Verified -> Complete
```

## State Meaning

- `Planned`: durable artifact identity, root, kind, and intended sections exist.
- `Scaffolded`: README indexes, manifest, and initial path structure exist.
- `ContentWritten`: meaningful leaf content was written under planned paths.
- `LocallyAudited`: artifact audit passed topology, manifest, line, drift, and
  semantic-readiness checks that apply to the artifact kind.
- `Verified`: required verification gates passed and named the artifact or root.
- `Complete`: the completion gate accepted the verified evidence.

Scaffold is never content evidence for cookbooks, stories, guides, knowledge
bases, or long reports. A write can move a planned or scaffolded artifact to
`ContentWritten`; only audit can move it to `LocallyAudited`; only verification
can move it to `Verified`.

## Evidence Owners

- `artifact.plan` owns planned identity.
- `artifact.apply` and `doc.scaffold` own scaffold evidence.
- `fs.write` and `fs.batch_write` own written-path evidence.
- `artifact.audit` and `doc.audit` own local audit evidence.
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
state names exact missing paths and the next executable action.

## Status

partially implemented. Scaffold, write-path recording, audit output, weak-path
tracking, and artifact-aware completion refusal exist. Full close-path coverage
for every artifact state remains open.
