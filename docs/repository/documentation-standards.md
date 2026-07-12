# Documentation Standards

## Purpose

Define the shape and ownership of active project documentation.

## File Shape

- ASCII prose only.
- Kebab-case filename.
- One H1 on the first line.
- `## Purpose` is the first section.
- Prose lines are at most 100 characters.
- Authored files are at most 200 lines.
- Tables have at most six columns.
- Status belongs only in `../current-state.md`.

## Directory Shape

Every docs directory has one README with `## Table of Contents` and at least two
other direct children. The README links every direct child with one description.
Every page is reachable from `../README.md` within the configured link depth.

## Ownership

One behavior has one owning page. Other pages link rather than restating the
rule. Target contracts use direct present-tense requirements;
`../current-state.md` distinguishes source gaps and evidence.

## Active History

Delete superseded descriptions when behavior changes. Git history retains prior
text. Do not preserve retired demos, commands, diagrams, or competing read orders
in the active tree.

## Checks

`check-docs`, `check-lines`, and `check-files` verify shape, links, topology,
limits, and file budgets. A docs change runs the applicable commands and records
actual output.
