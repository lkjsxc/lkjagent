# Protocol Experiments

## Purpose

Define controlled context, prompt, grammar, tool-view, and recovery experiments.

## Factors

Factors include retrieval strategy, lane budgets, compaction, stable-prefix
layout, tool-view size, action fields, concrete example, endpoint grammar
constraint, recovery ladder, and evidence-derived replanning.

## Cells

Run the anchored baseline, diagnostic one-factor changes, strongest pairwise
combinations, at least three integrated candidates, and the final winner against
baseline. Each cell uses the same model, seeds, owner schedule, budgets, fault
schedule, and scenario checks except for declared factors.

Noisy cells require at least three independent live runs and five when the first
three disagree. Deterministic parser, store, filesystem, and TUI invariants use
replay.

## Metrics

Measure obligation pass rate, false closes, unsupported claims, recovery
strategy changes, prompt tokens and duplication, cache reuse, parse and
admission success, effect count, latency, PTY identity, and cost. Per-scenario
results cannot be hidden by an aggregate.

## Adoption

Predeclare hard floors and comparative thresholds before outcomes. Adopt only a
complete integrated configuration that passes every floor and improves primary
task success or protected efficiency without a material regression. Record
rejected and conditional factors with source commit, configuration fingerprint,
raw evidence hashes, and rationale.
