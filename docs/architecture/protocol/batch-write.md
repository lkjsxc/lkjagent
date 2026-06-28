# Batch Write

## Purpose

Define the exact model-facing `fs.batch_write` payload forms accepted inside a
singular `<action>` envelope.

## Canonical Action Form

The canonical prompt example uses paired tags with the dispatcher line protocol.
This line protocol is the only prompt-facing batch form. No top-level JSON
action output and no nested `<file>` child blocks appear in model-facing prompt
cards:

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

The dispatcher receives blocks separated by `-- lkjagent-next-file --`. Prompt
contracts may name paths and sections before this action, but they do not
prefill body prose.

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
parameter names are unique inside one action. A payload such as
`<files><file><path>...</path><content>...</content></file></files>` is a schema
fault and is refused before mutation. If the model repeats adjacent
`<files>...</files>` wrappers for one `fs.batch_write`, the parser merges those
chunks into the canonical delimiter payload before dispatch. No other repeated
parameter name is normalized.

## Object-Literal Refusal

The model protocol is not top-level JSON, and JSON text inside `<files>` is not
a live `fs.batch_write` payload. Object-literal file arrays, object wrappers,
and provider-native tool calls are refused before mutation. Recovery records a
compact schema fault and renders a line-protocol example for the current root or
weak path.

The observation records the accepted input format as `line-protocol` only.

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
example uses that path instead of a placeholder. After a second equivalent
child-tag batch fault, recovery changes action class to `artifact.next`,
`graph.state`, deterministic inspection, or blocked handoff.

## Limits

- Maximum files per action: 20.
- Maximum bytes per file: 1,800.
- Maximum bytes per batch: 6,000.
- Duplicate paths are refused.
- Append mode is not part of `fs.batch_write`.
- Validation runs before file mutation, including stored write-contract path
  checks when an artifact cursor is active; a later filesystem error can still
  leave earlier writes in place and is reported as a tool error.

## Artifact Scope

Payload-too-large recovery keeps the original artifact identity. Splitting a
large artifact into unrelated drift files does not satisfy artifact readiness.
Artifact audit records unexpected paths as weak paths under the active root.

## Invariants

- The documented canonical example parses, validates, admits, and dispatches
  when authority admits `fs.batch_write`.
- Path-shaped unknown parameters normalize only under the safe recovery rules in
  this contract.
- Nested `<file>` child tags inside `<files>` refuse as a schema fault.
- Missing `content:` in any line-protocol block refuses the whole action.
- Duplicate paths refuse the whole action.
- Oversized file or batch payloads refuse the whole action before mutation.
- A batch write can move an artifact to content-written evidence only after the
  artifact ledger records the written paths.
- Content artifact writes under an active cursor must match stored contract
  paths before any file is mutated.

## Verification

Tests cover the canonical delimiter example, paired-tag `<files>` payloads,
object-literal refusal, missing content, duplicate paths, oversized payloads,
and artifact-ledger recording.

## Status

open for this redesign. The target prompt-facing and dispatcher contract is
line protocol only; any object-literal recovery path must stay outside prompt
context and must not mutate files.
