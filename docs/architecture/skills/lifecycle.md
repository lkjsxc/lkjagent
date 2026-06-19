# Lifecycle

## Purpose

Specify how skills are born, sharpened, and retired. The library is a
living asset sharpened through explicit maintenance paths per
[../runtime/self-maintenance.md](../runtime/self-maintenance.md).

## Creation

A skill is created by editing the source library and committing the change.
Validation enforces [format.md](format.md) in tests and documentation
gates; an invalid skill fails before it ships.

The bar for creation: the procedure ran successfully at least once, and the
Checks section quotes evidence that actually appeared. Speculative skills,
written before the procedure ever worked, violate
[../../agent/honest-state.md](../../agent/honest-state.md).

## Refinement

Skill refinement is a source change driven by transcript evidence:

- tighten Trigger if the skill was loaded and unused, or missed when needed;
- repair Procedure steps that observations contradicted;
- extend Must Not with any new failure mode actually observed;
- shrink everything: shorter skills load cheaper and read truer.

Each use of a skill may stamp outcome evidence (loaded, task closed cleanly
or not); refinement reads those stamps rather than guessing.

## Retirement

A skill is retired by deleting its file during refine-skills when its
trigger has not fired across a long horizon or its procedure is superseded
by a better skill. Retirement writes a memory row (kind lesson) naming what
replaced it, so the knowledge of why survives the file.

## Provenance

The repository git history is the provenance for skill changes. Runtime
data holds task state and memory; it does not hold skill files.

## The Builder Mirror

The same lifecycle governs [../../agent/skills/](../../agent/skills/README.md):
same format, same refinement pressure, same retirement bar, with git
history as provenance.

## Status

implemented for source-owned skills. Runtime self-refinement remains out of
scope.
