# Lifecycle

## Purpose

Specify how skills are born, sharpened, and retired. The library is a
living asset: its quality compounds because idle time is spent on it per
[../runtime/self-maintenance.md](../runtime/self-maintenance.md).

## Creation

A skill is created with skill.save when the agent recognizes a repeatable
procedure: a task took shell exploration that the next occurrence should
skip. Validation enforces [format.md](format.md) at save time; an invalid
skill is refused with every violation listed, so one retry fixes all.

The bar for creation: the procedure ran successfully at least once, and the
Checks section quotes evidence that actually appeared. Speculative skills,
written before the procedure ever worked, violate
[../../agent/honest-state.md](../../agent/honest-state.md).

## Refinement

The refine-skills maintenance directive picks the skill with the oldest
refinement stamp or the worst recent outcome and sharpens it against the
transcript record of its last uses:

- tighten Trigger if the skill was loaded and unused, or missed when needed;
- repair Procedure steps that observations contradicted;
- extend Must Not with any new failure mode actually observed;
- shrink everything: shorter skills load cheaper and read truer.

Each use of a skill stamps the state table with outcome evidence (loaded,
task closed cleanly or not); refinement reads those stamps rather than
guessing.

## Retirement

A skill is retired by deleting its file during refine-skills when its
trigger has not fired across a long horizon or its procedure is superseded
by a better skill. Retirement writes a memory row (kind lesson) naming what
replaced it, so the knowledge of why survives the file.

## Provenance

The library directory is a git repository on the data volume; every
skill.save commits with the acting context (task or maintenance cycle) in
the message. Diff history is the agent's own record of how its capabilities
evolved, inspectable by the owner with ordinary git tools.

## The Builder Mirror

The same lifecycle governs [../../agent/skills/](../../agent/skills/README.md),
executed by the coding agents through the repository workflow instead of
skill.save: same format, same refinement pressure, same retirement bar, with
git history as provenance in both worlds.

## Status

design-only. The format validator exists; skill.save, refinement, and
retirement actions land with later tool and maintenance slices.
