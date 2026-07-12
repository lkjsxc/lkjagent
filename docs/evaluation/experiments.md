# Model Experiments

## Purpose

Select model-sensitive protocol and context choices from complete repeated tasks.

## Plan

`../../evaluation/experiment-plan.tsv` defines concrete cells A-M. It is
committed before outcomes and is a strict ancestor of candidate evidence.

Early cells compare generic/tool-named envelopes, state-narrow/broad views,
likely-tool/no example, exact/line-range/diff edits, required/recent context,
compact/excerpt observations, stable/state-first ordering, and fault-only or
fault-plus-example recovery.

Safety, persisted decision specs, automatic native checks, final claim admission,
and factual receipts are fixed foundations rather than experiment switches.

## Repeats

Run at least three fresh attempts for each declared cell/scenario. If semantic
fingerprints differ, run two more. The fingerprint includes final bytes, changed
paths, parse/admission outcomes, tool sequence, checks, final state, and
unsupported claims.

Stop a cell after a hard safety failure and record remaining attempts as not run
due to rejection.

## Floors

Reject path escape, symlink write, stale overwrite, unrelated change, false
success, missing file, hidden-tool execution, unjournaled effect, JSON/prose
action acceptance, duplicate message, context contamination, or recoverable idle.

## Selection

Primary metric is deterministic task success. Efficiency can win only without a
material success regression. Secondary metrics include parse/admission rate,
turns, recovery, tokens, latency, prompt duplicates, and source precision.

Adopt one integrated profile. Record per-scenario results and factor interactions,
then remove all losing parser, config, and source paths. The result ledger remains
tracked even when candidate code is gone.
