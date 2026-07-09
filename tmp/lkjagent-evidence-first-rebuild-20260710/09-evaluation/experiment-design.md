# Experiment Design

## Factors

- context selection strategy;
- matter capsule use;
- stable-prefix layout;
- tool-view size;
- action field grammar;
- concrete example presence;
- endpoint grammar constraint;
- recovery ladder;
- static versus evidence-derived replanning.

## Cells

Do not test every theoretical Cartesian product. Use:

1. current baseline;
2. one-factor changes for diagnosis;
3. strongest pairwise combinations;
4. at least three integrated candidate sets;
5. final winner against baseline.

## Repeats

Use at least three real endpoint repeats for noisy cells and deterministic
replays for parser, store, filesystem, and TUI invariants. Fresh isolated data
prevents cross-run contamination.

## Controls

Keep model, sampling, scenario seed, budgets, and environment fixed within a
comparison. Record every deviation.

Each candidate run binds the exact ancestor commit that contained it. After
cleanup, rerun the adopted integrated configuration on the frozen source;
rejected code need not remain merely to make old evidence reproducible.

## Decision

Adopt only from task-level improvement, not one metric. A strategy that reduces
tokens but increases false closure, lost evidence, or malformed actions is
rejected. Apply every numeric floor and comparative rule in
adoption-thresholds.md before seeing outcomes.
