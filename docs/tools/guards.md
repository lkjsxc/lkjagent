# Guards

## Purpose

Define the engine-side guards that remain for explore tools.

## Path Guard

All file paths resolve inside the workspace. Absolute paths and `..` segments
return an effect error. The guard name is `tools.guard.path-inside-workspace`.

## Budget Guard

Each explore step has an action budget. The default is
`engine.explore.action-budget=20`, and templates may set lower caps. Budget
exhaustion invokes the retry ladder in
[../engine/retry-and-escalation.md](../engine/retry-and-escalation.md).

## Repeat Guard

A byte-identical action to the previous explore action is not executed. The
attempt records diagnosis `repeated action; state the next different action or
finish`. The guard key is `tools.guard.repeat-adjacent=true`.

## Terminal Effect

A guard failure affects only the active explore step. It does not change tool
admission for unrelated steps and does not add global recovery text.

## Failure This Prevents

Repeated explore actions cannot form a fixed point. The guard produces one
bounded diagnosis, and the retry prompt changes.
