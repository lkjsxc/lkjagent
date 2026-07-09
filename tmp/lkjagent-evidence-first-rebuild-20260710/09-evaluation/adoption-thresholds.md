# Adoption Thresholds

## Hard Correctness Floors

Every adopted cell and every final campaign must have:

- zero false closes, fabricated artifacts, path escapes, lost owner bytes,
  duplicate effects, unresolved required obligations, or wrong-project facts;
- every anchored semantic check passed on current bytes;
- every scheduled owner turn causally present in events and SQLite;
- zero repeated failure with the same operation, fault, strategy, budget, and
  context fingerprint;
- zero TUI duplicate IDs, causal inversions, blank-range excess, diagnostic
  leakage, or composer loss;
- 100 percent recall for scenario-declared required sources and zero unsupported
  deterministic claims;
- at least 98 percent first-pass parse and 97 percent first-pass admission over
  the full endpoint action corpus, with no malformed action producing an effect;
- PTY p95 measured input-to-render latency at or below 250 milliseconds.

A candidate failing one floor is rejected or conditional regardless of cost,
speed, or aesthetic appeal.

## Loop Floors

Each 15-minute run has at least eight real decisions, five useful decisions,
three progress-producing decisions, three scheduled owner turns, 600 seconds of
decision span, 600 seconds of owner-goal span, and 840 seconds of raw session
span. Useful and progressed are derived from changed obligations, verified
artifacts, new evidence, answered blockers, or strategy-changing recovery; idle
polls and synthetic state rewrites never count.

## Comparative Rule

Predeclare primary scenario checks before running a cell. Compare baseline,
isolated candidate, and integrated candidate with the same model, seeds, owner
schedule, budgets, and fault schedule. Use at least three independent runs per
cell and five when any outcome differs across the first three.

An integrated candidate may become default only if it passes every hard floor
in every repeat and either:

- improves weighted primary-task success by at least ten percentage points over
  baseline; or
- ties perfect task success while improving median rendered tokens, endpoint
  calls, or recovery time by at least fifteen percent without worsening another
  protected metric by more than five percent.

Weight daily-life, multi-project, and long-recovery scenarios equally. Report
per-scenario results; an aggregate cannot hide a regression.

## Decisions

`adopt` means enabled in the final flat configuration. `conditional` means the
interaction remains documented and tested but is disabled by default. `reject`
means its product branch is removed after raw evidence is retained. Multiple
adopted factors must also pass as one integrated configuration; isolated wins
cannot be assembled without a combination run.
