# Skill: Doc Contract Edit

## Purpose

Change a contract under docs/ correctly: right owner file, topology intact,
links sound, ledger updated, nothing restated.

## Trigger

A contract under docs/ needs to change, with no code moving.

## Context

- [../../README.md](../../README.md): find the owning file in the All Files manifest; never edit a mirror.
- [../../repository/documentation-standards.md](../../repository/documentation-standards.md):
  shape, topology, banned tokens.
- [../../repository/line-limits.md](../../repository/line-limits.md): the cap and the split recipe.
- [../../current-state.md](../../current-state.md): whether the change moves a status.

## Procedure

1. Locate the single owner of the rule being changed. If two files state
   it, the edit includes collapsing one into a link.
2. Edit the owner file. Keep the H1-Purpose shape, ASCII, and prose width.
3. If the file approaches 200 lines, split into a directory with a README
   per the split recipe, in this same change.
4. Update every README table of contents the change touches: new files
   linked, removed files dropped, the All Files manifest in docs/README.md
   when paths changed.
5. If a settled decision moved, update its record under
   [../../decisions/](../../decisions/README.md) with the new rejected
   direction, in the same commit.
6. Update [../../current-state.md](../../current-state.md) when an area
   status changed.
7. Run the docs gate and commit per
   [../../repository/commit-protocol.md](../../repository/commit-protocol.md).

## Checks

- `cargo run -p lkjagent-xtask -- check-docs` and `check-lines` print their
  ok lines; until the xtask exists, the interim checks in
  [../../operations/verification.md](../../operations/verification.md)
  print nothing.
- Every relative link in changed files resolves to an existing file.

## Must Not

- Do not restate a rule that has another owner; link it.
- Do not leave a new file out of its README table of contents.
- Do not use banned tokens or compatibility framing to describe the change.
- Do not edit docs and code in separate commits when one contract binds both.
