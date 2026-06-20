# Completion

## Purpose

Define completion for content artifacts.

## Gate

Completion means the requested artifact exists and verification evidence
exists. For content artifacts, completion requires:

- artifact root exists.
- README exists.
- manifest exists.
- semantic children exist.
- content-bearing files exist.
- audit passes or blocked handoff names exact failures.
- graph plan evidence exists.
- observation evidence exists.
- verification or audit evidence exists.
- no active unrecovered fault exceeds threshold.

## Refusals

Planning alone is never completion. Generic scaffold alone is never
completion. A file tree unrelated to the owner's requested artifact is never
completion.

## Status

partially implemented; document audit and scaffold-only refusal exist. Full
artifact-aware completion readiness remains open.
