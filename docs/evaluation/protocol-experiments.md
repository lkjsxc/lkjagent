# Protocol Experiments

## Purpose

Define controlled context, prompt, grammar, tool-view, and recovery experiments.

## Precommitment

[evaluation/experiment-plan.tsv](../../evaluation/experiment-plan.tsv) declares
outcome-free hypotheses, exact factors, cells, controls, scenarios, repeats,
metrics, and the anchored threshold hash. Its commit must be a strict ancestor
of every tested candidate commit. A later plan or undeclared factor is invalid.

## Factors

Factors include retrieval strategy, lane budgets, compaction, stable-prefix
layout, tool-view size, action fields, concrete example, endpoint grammar
constraint, recovery ladder, and evidence-derived replanning.

## Cells

Run the anchored baseline, diagnostic one-factor changes, strongest pairwise
combinations, at least three integrated candidates, and the final winner against
baseline. Each cell uses the same model, seeds, owner schedule, budgets, fault
schedule, and scenario checks except for declared factors.

Noisy cells require three independent endpoint runs and five when the first
three semantic outcome fingerprints differ. The fingerprint covers parse
outcome, action identity and admission, observations, blockers, and recovery.
Latency and token variance remain metrics rather than changing classification.
Deterministic parser, store, filesystem, and TUI invariants use replay.

## Trial Boundary

Domain trials use each anchored seed and first scheduled owner goal with a fresh
store, a clean locked build, the production endpoint adapter, and an Online
Backup snapshot. The runner isolates Cargo and runtime configuration, retains
raw and recursively redacted exchanges, exports every durable table and
workspace identity, and binds all bytes in run and campaign manifests. It
allows bounded native setup and at most one provider exchange per run. A goal
that closes before any decision or exchange is retained as a rejected
`no-provider-exchange` outcome rather than fabricated endpoint evidence. A
pre-endpoint configuration rejection is also a source-bound rejected row with
no invented database. Any made exchange must complete successfully.
Fault-schedule and recovery
branches remain explicitly unexercised unless durable events prove otherwise;
task success, semantic checks, and the 840-second floor remain unmeasured. Such
rows are conditional on fault and frozen live campaigns. A parse or admission
failure can reject an idea, but probe evidence can never adopt it.

## Metrics

Probe rows recompute endpoint calls, parse result, action identity, admission,
tokens when reported, latency, observations, blockers, and recovery events.
Recall, unsupported claims, repeated failure, recovery time, semantic checks,
and task success carry `not-measured` rather than invented values until their
scheduled branches run. Per-scenario results cannot be hidden by an aggregate.

## Adoption

Predeclare hard floors and comparative thresholds before outcomes. Adopt only a
complete integrated configuration that passes every floor and improves primary
task success or protected efficiency without a material regression. Record
rejected and conditional factors with source commit, configuration fingerprint,
raw evidence hashes, and rationale.
