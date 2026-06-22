# Context

## Purpose

This directory specifies the context engine: how graph-selected packages,
runtime observations, memory digest, and workspace brief are laid out,
budgeted, compacted, kept cache-hot, and kept clean. The window is the
scarcest resource in the system; every other subsystem bends to the rules
here. Decision:
[../../decisions/append-only-context.md](../../decisions/append-only-context.md).
Owned by the lkjagent-context crate.

## Table of Contents

- [context-frame.md](context-frame.md): state-derived decision frame for model turns.
- [layout.md](layout.md): prefix and log, and their mapping onto chat messages.
- [budgets.md](budgets.md): the token ledger for every window region.
- [compaction.md](compaction.md): the explicit event that distills and rebuilds.
- [caching.md](caching.md): prefix-cache discipline and what may invalidate it.
- [hygiene.md](hygiene.md): the allowlist of what may ever enter the window.
