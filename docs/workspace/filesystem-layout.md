# Filesystem Layout

## Purpose

Define the durable workspace directories and their source-of-truth rules.

## Tree

```text
workspace/
  README.md
  inbox/
  today/
  records/
    journal/
    calendar/
    todo/
    finance/
    note/
    project/
    development/
    routine/
    contact/
    reference/
  artifacts/
    documents/
    reports/
    transcripts/
    exports/
  projects/
  repos/
  indexes/
  archive/
  system/
    schemas/
    templates/
    style/
```

## Source Rules

`records/` contains owner-readable source records. `artifacts/` contains
lkjagent-generated or assembled files. `indexes/` contains derived views that can
be rebuilt. `archive/` keeps inactive records and artifacts without deleting
ledger evidence. `system/` contains schemas, templates, and style rules that may
enter prompts only when selected by a decision.

## Path Policy

All runtime filesystem effects stay under the configured workspace root after
canonicalization. Dotfiles, secrets, large binaries, generated caches, and
external repositories require explicit policy before prompt admission.

## Workspace READMEs

Each major owner-facing workspace directory should have a README explaining its
purpose, record shape, and allowed agent actions. These files guide the owner and
may be admitted as bounded context with source refs and fingerprints.

## Rebalancing

`workspace/system/workspace-manifest.json` records the schema number, root
policy, archive root, system root, and owner-facing directories. Pure workspace
entity validation treats record ids as stable identities and paths as movable
locations. `workspace plan-rebalance` previews canonical record moves.
`workspace apply-rebalance` validates paths, moves files, updates record rows,
writes path aliases, and stores rebalance audit rows. `record show` can resolve
an old path alias to the current stable record id.
