# Skill: Skill System

## Purpose

Change the skill format, loading, lifecycle, or library so that both
audiences move together: the runtime loader and the builder library obey
one format.

## Trigger

The skill format, loading, lifecycle, or library is changing.

## Context

- [../../architecture/skills/format.md](../../architecture/skills/format.md): the canonical shape; the single owner.
- [../../architecture/skills/loading.md](../../architecture/skills/loading.md): index, budgets, visibility rules.
- [../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md):
  creation bar, refinement, retirement.
- [../../architecture/skills/library.md](../../architecture/skills/library.md): location and seeds.
- [README.md](README.md): the builder instances that must keep validating.

## Procedure

1. Change format.md first; it is the only owner of the shape. Loading,
   lifecycle, validation code, and the check-docs skill rules all derive
   from it.
2. Update the lkjagent-skills validator and the check-docs skill-shape
   rules in lkjagent-xtask to the same new rules; they share a
   single rule table by construction, so change that table.
3. Re-validate every existing skill: all builder skills in this directory
   and all seed skills; bring each into the new shape in this same change.
4. If the index line format changed, recount the skill-index budget in
   [../../architecture/context/budgets.md](../../architecture/context/budgets.md)
   and keep degradation deterministic per loading.md.
5. Extend the validator's test table: a conforming skill, each violation
   class, and the 120-line boundary case.
6. If lifecycle stamps changed, update the state-table keys in
   [../../architecture/memory/store.md](../../architecture/memory/store.md)
   in the same change.

## Checks

- `cargo test -p lkjagent-skills` passes with the new validation table.
- check-docs validates every file in this directory against the new shape
  and prints its ok line.
- Every seed skill in the repository passes the runtime validator (one
  test loads them all).

## Must Not

- Do not fork the format: no builder-only or runtime-only headings beyond
  the optional Handoff already in the contract.
- Do not let validator and check-docs drift; one rule table feeds both.
- Do not add frontmatter, metadata blocks, or any structure outside
  markdown headings.
- Do not grow a skill past 120 lines; split the capability instead.
