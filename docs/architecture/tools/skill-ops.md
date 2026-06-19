# Skill Operations

## Purpose

The contract for skill.use: loading a source-owned skill into the window.
Skill structure is owned by
[../skills/format.md](../skills/format.md), loading mechanics by
[../skills/loading.md](../skills/loading.md), and the improvement cycle by
[../skills/lifecycle.md](../skills/lifecycle.md). Canonical parameter
table: [registry.md](registry.md).

## skill.use

| Parameter | Rule |
| --- | --- |
| name | required |

Appends the skill body as an immutable skill frame per
[../skills/loading.md](../skills/loading.md).

| Error | Response |
| --- | --- |
| unknown name | tool error |
| already loaded | notice pointing at the earlier skill frame |
| concurrent skill budget exceeded | refused per [../context/budgets.md](../context/budgets.md) |

## Source Ownership

The runtime registry does not expose a skill-writing tool. Skill changes
are repository edits to the source library, validated by the same gates as
other source files, then picked up by restart or prefix rebuild.

Recursive-knowledge tasks constrain writes to the seeded docs map. shell.run
is read-only after the auto-scaffolded nucleus, and fs writes outside the
allowed docs top-levels are refused. Expansion proceeds by small fs.write
batches named in the expansion queue, followed by map and rebalance updates.

## Status

implemented.
