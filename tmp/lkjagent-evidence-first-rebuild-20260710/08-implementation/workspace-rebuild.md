# Workspace Rebuild

## Task A: Roots And Config

Add strict flat configuration keys for workspace_root, timezone,
workspace_file_max_tokens, maintenance thresholds, prompt caps, tool limits,
and recovery budgets. Reject unknown keys, wrong types, and invalid ranges.
Prove each key has a runtime consumer.

## Task B: WorkspaceService

Move path validation, canonical root, reads, search, staged writes, atomic
rename, fingerprints, and effect-journal integration behind one service. Fix
relative-root list, tree, and search.

## Task C: Semantic Records

Implement capture, compose, update, archive, and activity trace through the
service. Use owner-local semantic dates and idempotency keys.

## Task D: Diary

Add journal-compose state, routed context, truthful reflection prompt, merge,
checks, and Japanese scenarios. Delete canned missing-detail content.

## Task E: Projects

Add project identity, selected repositories, notes, decisions, tasks, sessions,
artifact paths, Git read tools, patch effects, and verification evidence.

## Task F: Retrieval

Index bodies and metadata. Add exact path, lexical, trigram, date, project, kind,
and status filters. Validate source fingerprints before prompt admission.

## Task G: Navigation And Maintenance

Generate useful on-demand READMEs, correct incremental indexes, bounded pages,
external edit scan, validation, and safe rebalance.

## Task H: Reset And Import

Build a fresh native database from preserved workspace files. Quarantine
ambiguous old records and validate reproducible rebuild.
