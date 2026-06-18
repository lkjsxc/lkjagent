# Commit Protocol

## Purpose

The commit message shape and cadence. Commits are the project's narration
for future agents; each one states intent and verification truthfully.

## Shape

```
<intent line: why this change exists, not which files moved>

Constraint: <the rule or budget that shaped the approach, when one did>
Rejected: <alternative> | <why it lost>            (when a real fork existed)
Tested: <gates and commands that actually ran, with results>
Not-tested: <known gaps and why>
```

- The intent line is imperative, at most 72 characters, and answers why.
- Tested and Not-tested are required on every commit. Tested names only
  commands that ran in this working tree; claiming an unrun gate violates
  [../agent/honest-state.md](../agent/honest-state.md). Docs-only commits
  with no checker yet record the interim checks they ran.
- Constraint and Rejected appear when they carry information; empty
  ceremony is worse than absence.

## Cadence

- Commit at every coherent boundary: one subtree, one contract, one slice.
  A commit that needs "and" in its intent line is usually two.
- Never batch a day of work into one commit; the history is how future
  agents replay reasoning.
- Docs and the code they govern move in the same commit, including
  [../current-state.md](../current-state.md) when behavior moves, per
  [workflow.md](workflow.md).

## Boundaries

- No merge commits from local work; keep history linear.
- No amending published commits; corrections are new commits that say so.
- The git identity is whatever the environment provides; agents never edit
  git config.

## The Runtime Mirror

The running agent follows the same protocol when it commits in
/data/workspace or in its skill library
([../architecture/skills/lifecycle.md](../architecture/skills/lifecycle.md)):
intent first, honest Tested trailers, small steps. One protocol, both worlds,
per the unified principle in [../decisions/unified-skills.md](../decisions/unified-skills.md).
