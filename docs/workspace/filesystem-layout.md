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
    life/
      journal/
      todo/
      calendar/
      finance/
      notes/
      routines/
      contacts/
    work/
      projects/
      development/
    knowledge/
      notes/
      references/
  artifacts/
    documents/
    reports/
    transcripts/
    proof/
  indexes/
  archive/
  system/
    schemas/
    templates/
    prompts/
    manifests/
```

## Source Rules

`inbox/` contains ambiguous owner-turn traces that need owner review.
`records/` contains owner-readable source records. `artifacts/` contains
lkjagent-generated or assembled files, including transcripts and proof bundles.
`indexes/` contains derived views that can be rebuilt. `archive/` keeps inactive
records and artifacts without deleting ledger evidence. `system/` contains
schemas, templates, prompts, and manifest rules that may enter prompts only when
selected by a decision.

## Path Policy

All runtime filesystem effects stay under the configured workspace root after
canonicalization. Dotfiles, secrets, large binaries, generated caches, and
external repositories require explicit policy before prompt admission. Owner-turn
transcripts and inbox traces are workspace effects and follow the same path
checks.

## Workspace READMEs

Each major owner-facing workspace directory has a README when the directory
exists. The README explains purpose, record shape, allowed agent actions,
source-of-truth rules, and index behavior. These files guide humans and may be
admitted as bounded context with source refs and fingerprints.

## Rebalancing

`workspace/system/manifests/workspace-manifest.json` records schema, root
policy, archive root, system root, owner-facing directories, and link rules.
Pure workspace entity validation treats record ids as stable identities and
paths as movable locations. `workspace plan-rebalance` previews canonical moves
and link edits. `workspace apply-rebalance` validates paths, moves files,
updates record rows, writes aliases, repairs links when possible, and stores
audit rows. Failed moves are rolled back when possible or compensated with an
explicit audit row and validation warning.
