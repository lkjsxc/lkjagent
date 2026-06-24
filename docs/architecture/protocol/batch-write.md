# Batch Write

## Purpose

Define the exact model-facing `fs.batch_write` payload forms accepted by the
protocol parser and file tool dispatcher.

## Canonical Action Form

The canonical prompt example uses paired tags with the dispatcher line protocol.
No JSON appears inside `<files>` in default prompt cards:

```text
<act>
<tool>fs.batch_write</tool>
<files>
path: docs/example-a.md
content:
# Example A

Concrete content.
-- lkjagent-next-file --
path: docs/example-b.md
content:
# Example B

Concrete content.
</files>
</act>
```

The dispatcher receives blocks separated by `-- lkjagent-next-file --`.

## Paired-Tag Form

The paired-tag action form may wrap the same dispatcher payload in `<files>`:

```text
<act>
<tool>fs.batch_write</tool>
<files>
path: docs/example.md
content:
# Example

Concrete content.
</files>
</act>
```

Repeated `<file>` child tags are not valid in the paired-tag grammar because
parameter names are unique inside one action.

## JSON Envelope Form

The JSON envelope is valid when it is the whole model response:

```json
{
  "schema": "lkj-action",
  "action": {
    "tool": "fs.batch_write",
    "params": {
      "files": [
        { "path": "docs/example.md", "content": "# Example\n\nConcrete content." }
      ]
    }
  }
}
```

## JSON Inside Files Recovery

fs.batch_write canonical payload is line protocol inside `<files>`. The
dispatcher also accepts a JSON array inside `<files>` when each object contains
`path` and `content`. It also accepts a JSON object with a `files` array of the
same objects. JSON-in-files is not preferred, but it is a supported recovery
normalization. Objects without `path` and `content` are refused before mutation.

The observation records the normalized input format as `line-protocol`,
`json-array`, or `json-object-files`.

## Path-Shaped Parameter Recovery

A live schema fault can arrive as a missing `files` parameter plus unknown
parameters whose names look like relative file paths. Safe normalization may
convert that shape into a `files` payload only when all of these conditions are
true:

- every unknown parameter name is a relative path under the current admitted
  root or admitted weak-path set;
- no unknown parameter is absolute, parent-traversing, empty, or duplicated;
- every unknown parameter value is supplied content, not a nested action or
  schema fragment;
- file and batch size limits pass;
- no semantic content is invented.

Unsafe shapes are refused before mutation. The refusal example must be concrete
and path-scoped. When the current artifact root or weak path is known, the
example uses that path instead of a generic `VALUE` placeholder.

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
envelope arrays, JSON text inside `<files>`, missing content, duplicate paths,
oversized payloads, and artifact-ledger recording.

## Status

partially implemented for parser normalization, JSON envelope arrays,
JSON-in-files recovery, dispatcher limits, duplicate-path refusal, placeholder
refusal, and artifact write-path recording. Path-shaped unknown parameter
normalization and route-level recovery for every batch schema fault remain open.
