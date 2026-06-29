# Facts

## Purpose

Define the runtime facts that are derived before obligation selection.

## Artifact Root Status

`ArtifactRootStatus` is one of:

- `Unknown`: no root fact is trusted yet.
- `Missing`: the root does not exist.
- `RootIsFile`: the requested root is a file.
- `EmptyDirectory`: the directory exists without identity files.
- `IdentityIncomplete`: catalog, README, or semantic leaves are missing.
- `StructureFailed`: topology, link, path, or line checks failed.
- `StructurePassed`: document structure passed but content is not ready.
- `ContentWeak`: semantic content exists but fails profile checks.
- `Ready`: structure and semantic content are ready for verification.

## Document Audit Facts

`DocumentAuditFacts` contains:

- `root`: normalized artifact or document root.
- `status`: the derived artifact root status.
- `topology_lane`: passed, failed, or not-requested.
- `content_lane`: passed, failed, or not-requested.
- `failures`: exact audit failure strings.
- `candidate_runtime_event`: the event the observation should create.
- `candidate_contract_kind`: the write contract kind when repair is needed.

`missing_root` maps to status `Missing`, event `ArtifactRootMissing`, and a
root identity contract kind. `root_is_file` maps to `RootIsFile` and an exact
inspection or blocked handoff, not blind directory creation.

## Write Contract Facts

`WriteContractFacts` contains:

- `root`: normalized root.
- `exact_paths`: full paths the model may write.
- `max_files`, `max_file_bytes`, and `max_batch_bytes`.
- `required_sections`: section or content signals required in each file.
- `forbidden_weak_phrase_classes`: scaffold, placeholder, owner-term-only, and
  generic-example classes.
- `status`: pending, satisfied, failed, or blocked.

A content write is admissible only when these facts are stored in the current
runtime decision.

## Observation Conversion

Tool observations are converted into runtime events before the next decision.
Audit text is not merely prompt guidance. It changes facts, facts create
obligations, and obligations drive the next resolver plan.

Root status is durable. A missing-root audit, root-missing `artifact.next`
result, artifact-ledger readiness failure, or artifact batch cursor is copied
into the artifact facts and remains available across parse faults, provider
anomalies, tool errors, and graph inspections until a contracted write records
progress or a later audit changes the status. Batch cursors expose one remaining
path at a time as `candidate_action=fs.batch_write`, then expose
`candidate_action=artifact.audit` when the planned paths are complete.
