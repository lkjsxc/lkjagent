# Line Limits

## Purpose

The cap that keeps every file readable in one model glance: 200 lines,
everywhere, with splitting rules so the cap produces structure instead of
fragmentation.

## The Cap

| File class | Limit |
| --- | --- |
| every Markdown file, including README.md and AGENTS.md | 200 lines |
| every Rust source file | 200 lines |
| every script, config, compose, and workflow file | 200 lines |
| graph context package docs | 200 lines |

There are no exemptions and no generated-file escape hatch: generated
content that cannot meet the cap does not get committed.

## How to Split

Split by ownership, never by arbitrary halving:

- A doc approaching the cap becomes a directory: a README as the map plus
  short children, each owning one aspect. The topology rules are in
  [documentation-standards.md](documentation-standards.md).
- A Rust module approaching the cap splits along data, pure transitions,
  and adapters, per [functional-style.md](functional-style.md); the parent
  module re-exports the public surface.
- A table outgrowing its file is usually two tables with different owners;
  find the second owner before reaching for a wider file.
- When a large edit is coming, split first in its own commit with behavior
  unchanged, then land the edit in the new shape.

## Why 200

A 200-line file fits in roughly 2,000 tokens: small enough that the agent
reading it pays one observation, small enough to diff-review whole, and
small enough that two files never silently own the same idea. The cap is
load-bearing for the context budgets in
[../architecture/context/budgets.md](../architecture/context/budgets.md):
the system the harness runs on assumes its own sources honor it.

## Enforcement

The check-lines gate fails any file over its cap, listing every offender
with its count. Until the xtask exists, the interim shell check in
[../operations/verification.md](../operations/verification.md) does the same.
