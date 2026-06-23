# Artifact Ledger

## Purpose

This file owns durable artifact identity, lifecycle, weak paths, audit status, and batch cursor references.

## Contract

A requested artifact receives a semantic identity made from case id, artifact kind, normalized topic, and requested
scale. Completion reads the artifact ledger, not raw file existence. Adoption, scaffold attachment, content planning,
readiness repair, audit, and completion update the ledger.

## Inputs

- owner objective and normalized topic.
- artifact kind, requested scale, profile, and candidate root.
- manifest, README topology, content readiness, objective match, and audit results.
- changed paths from `fs.write` and `fs.batch_write`.
- batch cursor and weak path records.

## Outputs

- artifact ledger record with identity, root, lifecycle state, topology state, readiness state, objective state,
  latest audit id, weak path count, and timestamps.
- weak path records with missing requirements, weak signals, semantic mismatch, and retry counts.
- readiness evidence tied to current artifact id.

## Invariants

- Equivalent existing roots are adopted and repaired, not duplicated.
- Adoption never grants completion by itself.
- Repair writes mark changed paths and require a later audit before close.
- Readiness evidence is valid only for the current semantic artifact id.
- Completion fails when the ledger has unresolved weak paths.

## Failure Cases

- A scaffold root is treated as finished because the directory exists.
- A weak existing root is ignored and a duplicate root is created.
- A repair write closes the case without re-audit.
- A Japanese cookbook passes with unrelated bread content.

## Verification

- `cargo test -p lkjagent-tools --test artifact_next`
- artifact adoption, weak-path, and completion reducer tests.
- benchmark fixtures for scaffold-only and semantic drift failures.

## Status

design-only for the normalized ledger; current artifact state is partial.
