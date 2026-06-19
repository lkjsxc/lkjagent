# Principles

## Purpose

These principles rank every design trade in lkjagent. When two contracts
conflict, the higher principle wins. When a principle and convenience
conflict, the principle wins.

## Ranked Principles

1. Honesty. The system never fakes success, never shows placeholder results,
   and never claims verification that did not run.
   Canonical rule: [../agent/honest-state.md](../agent/honest-state.md).
2. Smallness. Fewer features, fewer crates, fewer lines. Every addition must
   pay for itself; removal is always in season. Files stay at or below 200
   lines per [../repository/line-limits.md](../repository/line-limits.md).
3. Context is sacred. The 32k window is the scarcest resource. Nothing enters
   it without a budget and an owner.
   Canonical rule: [../architecture/context/hygiene.md](../architecture/context/hygiene.md).
4. Cache discipline. The prompt prefix is append-only between compactions so
   the endpoint prefix cache stays hot. Designs that mutate the prefix lose.
5. Pure core, effectful edge. Decisions are pure functions over plain data;
   IO lives in thin adapters. See
   [../repository/functional-style.md](../repository/functional-style.md).
6. One rule, one owner. Every contract lives in exactly one file; everything
   else links to it. Duplication is drift waiting to happen.
7. Self-improvement over feature growth. Maintenance work goes to
   distilling memory and improving source skills, not speculative features.
8. The agent is the user. Docs, errors, and formats are optimized for LLM
   reading first. Humans arrive through agents.

## Applying the Ranking

- A feature that pollutes context loses to principle 3 even if useful.
- A clever cache trick that hides failures loses to principle 1.
- A convenience dependency that doubles build size loses to principle 2.
- A duplicate explanation loses to principle 6; replace it with a link.
