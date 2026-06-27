# Artifact Ledger Completion

## Purpose

This task makes artifact completion depend on a durable semantic ledger and profile-specific readiness reducers.

## Contract

Artifact planning creates identity and profile fields. Adoption attaches equivalent roots and audits them. Writes
update changed paths. `artifact.audit` creates readiness evidence only when the profile reducer passes. Completion
reads artifact ledger state, not raw file existence or direct graph notes.

## Inputs

- artifact contracts under `docs/architecture/artifacts/`.
- store schema for graph and artifact data.
- `artifact.plan`, `artifact.apply`, `artifact.next`, `artifact.audit`, `fs.write`, and `fs.batch_write` routes.
- completion policy in runtime authority.

## Outputs

- artifact ledger records.
- weak path records with missing requirement labels.
- current batch cursor references.
- audit evidence tied to current artifact id.
- completion refusals that name the next admitted audit or repair action.

## Invariants

- Existing equivalent roots are repaired, not duplicated.
- Scaffold, README-only, and unsupported verification claims fail readiness.
- Repair writes are re-audited before close.
- Direct `graph.evidence` cannot satisfy audit-owned artifact readiness.
- Objective drift blocks completion.

## Failure Cases

- A Japanese cookbook closes with bread-only content.
- A shallow dictionary closes on a term list.
- A generated docs scaffold passes because topology exists.
- A graph complete node closes while artifact weak paths remain.
- The long-novel scaffold root closes while 28 structure-only story pages remain
  after `doc.audit` failure.

## Verification

- `cargo test -p lkjagent-store --test artifact_cursor`
- `cargo test -p lkjagent-tools --test artifact_ledger_tools`
- `cargo test -p lkjagent-tools --test artifact_tools`
- `cargo test -p lkjagent-tools --test artifact_next`
- `cargo test -p lkjagent-tools --test artifact_next_ledger`
- `cargo test -p lkjagent-tools --test artifact_write_ledger`
- `cargo test -p lkjagent-tools --test doc_tools`
- `cargo test -p lkjagent-runtime --test artifact_ledger_completion`
- artifact completion reducer tests.

## Status

partially implemented. Ledger schema, store APIs, writes from `artifact.plan`,
`artifact.apply`, `artifact.audit`, and `artifact.next`, successful write-path
cursor completion marking, failed write-path cursor marking, audit observations
tied to `artifact_ledger_id`, invalid-root markers for audited `.md` roots,
daemon `agent.done` refusals for unresolved ledger weak paths, story semantic
readiness checks in `artifact.audit`, and completion refusal status with current
artifact plus next audit action exist. The active long-novel log proves scaffold
identity is not readiness evidence and weak story pages must drive repair before
close. Adoption repair flow and proof for every close path remain open.
