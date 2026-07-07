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

## Core Contract

The workspace is the local file tree the owner can read and edit. SQLite is the
runtime ledger and decision authority. Records, artifacts, indexes, state cells,
edges, checks, and proof rows reference each other through stable ids and
fingerprints. Neither files nor prose prompts become a second control plane.

## Write-Through Rule

Anything the owner asks lkjagent to record is written under `data/workspace` by
default. A successful recording report names the path, fingerprint, record id,
and index state. A failure report says no file was written and records the
reason.
