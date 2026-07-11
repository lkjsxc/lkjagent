# Filesystem Layout

## Purpose

Define the durable workspace directories and their source-of-truth rules.

## Tree

```text
workspace/
  README.md
  inbox/
  life/
    journal/
    todo/
    calendar/
    finance/
    notes/
  knowledge/
  projects/
  artifacts/
    documents/
    reports/
    transcripts/
    proof/
  activity/
  indexes/
  archive/
  system/
    operations/
    quarantine/
    import-review/
```

## Source Rules

`inbox/` contains ambiguous owner-turn traces that need owner review.
`life/`, `knowledge/`, and `projects/` contain owner-readable source.
`artifacts/` contains
lkjagent-generated or assembled files, including transcripts and proof bundles.
`indexes/` contains derived views that can be rebuilt. `archive/` keeps inactive
records and artifacts without deleting ledger evidence. `system/` contains
operation previews, quarantine diagnostics, and import review. System files may
enter prompts only when selected by a decision.

## Path Policy

All runtime filesystem effects stay under the configured root. Reject absolute
model paths, parent traversal, symlink escapes, control characters, reserved
system paths, and case-collision ambiguity. Use descriptor-relative no-follow
traversal or an equivalent race-resistant API; canonicalization alone cannot
prevent a symlink swap. Search inventory likewise rejects hidden paths, root
system/index/archive trees, non-UTF-8 names, non-ASCII cased names, case
collisions, and symlinks. Uncased Unicode names remain valid.

## Workspace READMEs

Create only the branch required by real content. Each new owner-facing directory
gets a README that explains purpose, record shape, allowed agent actions,
source-of-truth rules, and index behavior. These files guide humans and may be
admitted as bounded context with source refs and fingerprints.

## Rebalancing

An operation preview under `workspace/system/operations/` records schema, root
policy, archive root, affected documents, target paths, and link rules.
Pure workspace entity validation treats record ids as stable identities and
paths as movable locations. `workspace plan-rebalance` previews canonical moves
and link edits. `workspace apply-rebalance` validates paths, moves files,
checks the current file fingerprint against the ledger row, writes aliases,
repairs exact old-path links in record files when possible, updates touched
record fingerprints, rebuilds indexes, and stores audit rows. Each move records
immutable prior and intended bytes before a Linux no-clobber move. Alias and audit
rows are written in one store transaction. Startup and apply settle only a matching
moved file with an absent prior path; conflicts remain prepared. Multi-move recovery remains open.
