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

Run three bounded configured protocol probes for every declared cell/scenario:

```sh
cargo run --locked -p lkjagent-xtask -- experiment run --endpoint-file FILE
```

Only response hashes and parse outcomes are retained. Tool-named envelopes,
broad registries, and non-exact edit forms are statically rejected because the
production grammar cannot safely admit them. Probes are interaction evidence,
not semantic success; all five development campaign classes must also pass
before integrated cell K can win.

## Floors

Reject path escape, symlink write, stale overwrite, unrelated change, false
success, missing file, hidden-tool execution, unjournaled effect, JSON/prose
action acceptance, duplicate message, context contamination, or recoverable idle.

## Selection

Primary metric is deterministic task success. Efficiency can win only without a
material success regression. Secondary metrics include parse/admission rate,
turns, recovery, tokens, latency, prompt duplicates, and source precision.

Adopt integrated cell K only when at least three quarters of K probes parse,
all parse failures remain safely inert, and all development campaign classes
pass. Record per-scenario interactions, then remove all losing parser,
configuration, source, and stale runner paths. The result ledger remains
tracked after candidate code is gone.
