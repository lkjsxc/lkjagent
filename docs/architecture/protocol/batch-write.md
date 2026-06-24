# Batch Write

## Purpose

Define the exact model-facing `fs.batch_write` payload forms accepted inside a
singular `<action>` envelope.

## Canonical Action Form

The canonical prompt example uses paired tags with the dispatcher line protocol.
No top-level JSON action output appears in model-facing prompt cards:

```text
<action>
<tool>fs.batch_write</tool>
<files>
path: stories/chronos-fracture/catalog.toml
content:
[artifact]
root = "stories/chronos-fracture"
kind = "story"

-- lkjagent-next-file --
path: stories/chronos-fracture/README.md
content:
# Chronos Fracture

## Purpose

Navigate the story bible for Chronos Fracture.
</files>
</action>
```

The dispatcher receives blocks separated by `-- lkjagent-next-file --`.

## Paired-Tag Form

The paired-tag action form wraps the dispatcher payload in one `<files>`
parameter:

```text
<action>
<tool>fs.batch_write</tool>
<files>
path: stories/chronos-fracture/setting/timeline.md
content:
# Timeline

## Purpose

Track cause and effect for Chronos Fracture.
</files>
</action>
```

Repeated `<file>` child tags are not valid in the paired-tag grammar because
parameter names are unique inside one action.

## JSON Inside Files Recovery

The model protocol is not top-level JSON. `fs.batch_write` may still accept JSON
text inside `<files>` when recovery receives a single parameter payload. The
accepted payload is either a JSON array of objects with `path` and `content`, or
a JSON object with a `files` array of the same objects. Objects without both
fields are refused before mutation.

The observation records the normalized input format as `line-protocol`,
`json-array`, or `json-object-files`.

## Path-Shaped Parameter Recovery

A live schema fault can arrive as a missing `files` parameter plus unknown
parameters whose names look like relative file paths. Safe normalization may
convert that shape into a `files` payload only when all conditions are true:

- every unknown parameter name is a relative path under the current admitted
  root or admitted weak-path set;
- no unknown parameter is absolute, parent-traversing, empty, or duplicated;
- every unknown parameter value is supplied content, not a nested action or
  schema fragment;
- file and batch size limits pass;
- no semantic content is invented.

Unsafe shapes are refused before mutation. The refusal example must be concrete
and path-scoped. When the current artifact root or weak path is known, the
example uses that path instead of a placeholder.

## Limits

- Maximum files per action: 20.
- Maximum bytes per file: 1,800.
- Maximum bytes per batch: 6,000.
- Duplicate paths are refused.
- Append mode is not part of `fs.batch_write`.
- Validation runs before file mutation; a later filesystem error can still leave
  earlier writes in place and is reported as a tool error.

## Artifact Scope

Payload-too-large recovery keeps the original artifact identity. Splitting a
large artifact into unrelated drift files does not satisfy artifact readiness.
Artifact audit records unexpected paths as weak paths under the active root.

## Invariants

- The documented canonical example parses, validates, admits, and dispatches
  when authority admits `fs.batch_write`.
- Path-shaped unknown parameters normalize only under the safe recovery rules in
  this contract.
- Missing `content:` in any line-protocol block refuses the whole action.
- Duplicate paths refuse the whole action.
- Oversized file or batch payloads refuse the whole action before mutation.
- A batch write can move an artifact to content-written evidence only after the
  artifact ledger records the written paths.

## Verification

Tests cover the canonical delimiter example, paired-tag `<files>` payloads, JSON
text inside `<files>`, missing content, duplicate paths, oversized payloads, and
artifact-ledger recording.

## Status

partially implemented for parser normalization, JSON-in-files recovery,
dispatcher limits, duplicate-path refusal, placeholder refusal, artifact
write-path recording, and safe path-shaped unknown parameter normalization.
Top-level JSON action parsing is no longer part of live dispatch. Route-level
recovery for every batch schema fault remains open.
