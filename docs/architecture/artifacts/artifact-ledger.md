# Artifact Ledger

## Purpose

This file owns durable artifact identity, lifecycle, weak paths, audit status, atom graph rows, and write contracts.

## Contract

A requested artifact receives a semantic identity made from case id, artifact kind, normalized topic, and requested
scale. Completion reads the artifact ledger, not raw file existence or raw audit strings. Adoption, scaffold
attachment, content planning, readiness repair, audit, and completion update the ledger.

The ledger stores document topology and artifact readiness as separate fields. Topology records whether the root,
README files, manifest, links, and path shape are coherent. Readiness records whether atom rows satisfy the artifact
kind, role, owner objective, weak-path repair contract, count floors, and latest audit. A `document audit passed` string
can only update topology. It can never set readiness to `passed`.

## Inputs

- owner objective and normalized topic.
- artifact kind, requested scale, profile, and candidate root.
- manifest, README topology, content readiness, objective match, and audit results.
- changed paths from `fs.write` and `fs.batch_write`.
- batch cursor, weak path, atom, edge, contract, event, assembly, and readiness records.

## Outputs

- artifact ledger record with identity, root, lifecycle state, topology state, readiness state, objective state,
  latest audit id, weak path count, and timestamps.
- weak path records with role, missing requirements, weak signals, semantic mismatch, repair contract id, and retry
  counts.
- readiness evidence tied to current artifact id and the audit that created it.
- atom graph projection with next atom, next path, active contract, measured total, and completion blockers.

## Invariants

- Equivalent existing roots are adopted and repaired, not duplicated.
- Adoption never grants completion by itself.
- Repair writes mark changed paths and require a later audit before close.
- Readiness evidence is valid only for the current semantic artifact id.
- `artifact.audit` resolves kind from the ledger or root before audit.
- Story readiness is role-aware; owner terms and role labels are signals, not proof.
- Completion fails when the ledger has unresolved weak paths.

## Failure Cases

- A scaffold root is treated as finished because the directory exists.
- A weak existing root is ignored and a duplicate root is created.
- A repair write closes the case without re-audit.
- A Japanese cookbook passes with unrelated bread content.
- A story bible passes because one page lists all required labels without role-specific content.

## Verification

- `cargo test -p lkjagent-store --test artifact_cursor`
- `cargo test -p lkjagent-tools --test artifact_ledger_tools`
- `cargo test -p lkjagent-tools --test artifact_next`
- `cargo test -p lkjagent-tools --test artifact_next_ledger`
- `cargo test -p lkjagent-tools --test artifact_write_ledger`
- artifact adoption, weak-path, and completion reducer tests.
- benchmark fixtures for scaffold-only and semantic drift failures.

## Status

implemented. SQLite schema and store APIs persist artifact identity, lifecycle
status, readiness status, weak path counts, weak path requirement labels, batch
cursor rows, durable plans, atoms, atom edges, write contracts, atom events,
assembly runs, and readiness projections. `artifact.plan`, `artifact.audit`,
and `artifact.next` update both the identity ledger projection and the atom graph.
`fs.write` and `fs.batch_write` mark planned cursor paths and active contracts
after successful contract-matching writes. Audit-owned evidence and
artifact-aware completion read ledger rows on the daemon path.
