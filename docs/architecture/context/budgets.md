# Budgets

## Purpose

The token ledger: every region of the window has a cap, an owner, and an
overflow rule. Initial contract values assume a 32,768-token window and are
config-tunable; the ratios are the contract, the absolute numbers follow the
window.

## The Ledger

| Region | Cap (tokens) | Owner | On overflow |
| --- | --- | --- | --- |
| generation reserve | 1,024 | harness | never occupied; subtracted first |
| prefix: identity and rules | 768 | [../protocol/system-prompt.md](../protocol/system-prompt.md) | build-time check fails |
| prefix: grammar and tool registry | 1,024 | [../protocol/system-prompt.md](../protocol/system-prompt.md) | build-time check fails |
| prefix: skill index | 512 | [../skills/loading.md](../skills/loading.md) | oldest-stamped entries drop to name-only |
| prefix: workspace brief | 1,024 | [../sandbox/workspace.md](../sandbox/workspace.md) | head kept, truncation notice |
| prefix: memory digest | 2,048 | [../memory/distillation.md](../memory/distillation.md) | digest builder must rank within cap |
| log: owner frame | 4,096 each | [../runtime/queue-intake.md](../runtime/queue-intake.md) | head kept, truncation notice |
| log: observation | 2,048 each | [../tools/registry.md](../tools/registry.md) | head and tail kept, middle elided, truncation notice |
| log: skill body | 2,048 each | [../skills/format.md](../skills/format.md) | skill.save refuses oversized skills |
| log: loaded skills concurrent | 6,144 total | [../skills/loading.md](../skills/loading.md) | skill.use refused with notice |
| whole window trigger | 28,672 used | [compaction.md](compaction.md) | compaction at next boundary |
| post-compaction target | 8,192 | [compaction.md](compaction.md) | compaction must reach it or fail loudly |

## Rules

- Caps are enforced by the harness before frames are appended; the model
  never causes an endpoint overflow error.
- Truncation is always marked with a notice naming what was cut and how to
  retrieve the rest (a ranged fs.read, a narrower shell command, a
  memory.find query).
- The prefix total (5,376 max) plus reserve leaves at least 26,368 tokens of
  log space at the initial values; the context engine asserts this at
  startup and refuses configs that starve the log below 16,384.
- Budget arithmetic lives in pure functions in lkjagent-context with
  table-driven tests; no budget decision happens inside an IO adapter.

## Status

implemented.
