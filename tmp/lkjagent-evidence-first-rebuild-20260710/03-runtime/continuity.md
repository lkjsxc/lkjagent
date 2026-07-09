# Continuity

## Useful Work Duration

Total work is not capped by a small turn count. Each cycle remains bounded, but
an active matter continues across as many cycles as its evidence graph needs.
Configurable budgets govern tokens, elapsed active time, effect count, and
recovery cost separately.

## Progress Vector

After each decision, persist:

- newly satisfied obligations;
- new or changed artifacts;
- new source evidence;
- reduced uncertainty;
- changed failure strategy;
- changed dependency or wake condition.

If none changes across the configured window, emit no-progress and select
inspect, split, replan, clarify, or suspend.

## Waiting

Waiting-owner includes one bounded question. Waiting-external includes a due
time or observable condition. Both remain visible and can coexist with other
runnable matters.

## Crash Resume

On startup:

1. recover prepared filesystem effects;
2. settle or resume interrupted endpoint decisions;
3. rebuild derived projections if fingerprints disagree;
4. select from the fresh state vector.

Do not replay a completed effect or reuse an uncommitted model response.
