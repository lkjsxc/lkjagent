# Workspace

## Purpose

Define the owner-readable workspace tree that is linked to the SQLite runtime
ledger.

## Table of Contents

- [filesystem-layout.md](filesystem-layout.md): directory contract and source
  ownership.
- [records.md](records.md): generic Markdown record shape and identity rules.
- [indexes.md](indexes.md): generated views, staleness, and rebuild rules.
- [personal-records.md](personal-records.md): journal, calendar, TODO,
  finance, contacts, references, and routines.
- [development-records.md](development-records.md): projects, repositories,
  software work, and proof artifacts.
- [transaction-protocol.md](transaction-protocol.md): atomic writes,
  settlement, idempotency, recovery, and path identity.

## Core Contract

The workspace is the local file tree the owner can read and edit as auxiliary
memory. Runtime data and workspace content use separate configured roots. Compose
mounts them at `/data` and `/workspace`. SQLite is the runtime ledger and
decision authority. Documents, conversation, artifacts, indexes, state, checks,
and proof reference stable IDs and fingerprints. Neither files nor prompts
become a second control plane.

## Write-Through Rule

Anything the owner asks lkjagent to record is written under the configured
workspace root before success is reported. Create only the branch needed by real
content. A successful recording report names document ID, path, revision,
fingerprint, and index state. A failure records that no file was written and why.
Artifact work is incomplete until final bytes, revision, effect observation,
checks, manifest, and response path evidence agree.
