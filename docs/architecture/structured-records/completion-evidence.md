# Completion Evidence

## Purpose

Define the records required before owner work may close.

## Requirements

Every owner task needs plan evidence, observation evidence, verification
evidence or an accepted not-run reason, no active unrecovered fault above
threshold, and no pending owner followup.

Artifact tasks also require artifact evidence: root exists, README exists,
manifest exists, semantic children exist, content-bearing section files exist,
and audit passed. A generic scaffold cannot satisfy a content artifact.

## Blocked Handoff

Blocked closure is separate from completion. A blocked handoff records failed
checks, attempted recovery, remaining owner decision if any, and the next safe
action. It does not claim the requested artifact is done.

## Status

partially implemented; `doc.scaffold` no longer satisfies document-structure
evidence and failed `doc.audit` output does not satisfy it. `doc.audit` rejects
scaffold-only content artifacts for story and cookbook profiles. Blocked
handoff states and richer artifact manifests remain open.
