# Record Model

## Purpose

Define the durable records used to represent many topics without producing a
pile of repeated Markdown stubs.

## Kinds

The structured record graph contains:

- `TopicRecord`
- `SubtopicRecord`
- `ArtifactRecord`
- `ArtifactNode`
- `SectionRecord`
- `ClaimRecord`
- `EvidenceRecord`
- `DecisionRecord`
- `FailureRecord`
- `RecoveryRecord`
- `MemoryRecord`
- `VerificationRecord`
- `CompletionRecord`

Each record stores kind, title, scope path, owning task id, source event id,
tags, content hash, status, and links to parent, child, prerequisite, evidence,
and supersedes records. Content text may be absent for planning records, but no
record may pretend a file or memory row exists before the effect succeeds.

## Identity

The default identity is:

```text
record_key = kind + normalized_title_slug + scope_path + content_hash_prefix
```

The slug is normalized ASCII, lower case, punctuation-collapsed, and stable
across retries. The content hash uses normalized body text when body text
exists and an empty marker when it does not.

## Ownership

Owner tasks create or adopt records. Maintenance may refine, merge, or delete
records only through runtime-owned operations that record the effect. The model
does not decide identity by free-form file name alone.

## Status

partially implemented; graph cases, document state, memory rows, evidence,
faults, and transitions exist. A unified structured-record API remains open.
