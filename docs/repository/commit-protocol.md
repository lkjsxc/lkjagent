# Commit Protocol

## Purpose

Define commit message shape and cadence.

## Shape

```text
<intent line: imperative, at most 72 characters>

Constraint: <rule or budget that shaped the approach>
Rejected: <alternative> | <why it lost>
Tested: <commands that actually ran, with results>
Not-tested: <known gaps and why>
```

`Tested` and `Not-tested` are required. `Constraint` and `Rejected` appear when
they carry real information. The intent line says why the change exists.

## Cadence

Commit coherent slices: one contract group, one crate seam, one behavior, or one
fix. Do not batch unrelated work. Docs and code that move behavior land
together, including [../current-state.md](../current-state.md) when truth moves.

## Honesty

A command belongs in `Tested` only if it ran in this checkout. A failing command
is still evidence and is recorded with the result.
