# Budgets

## Purpose

The token ledger: every region of the window has a cap, an owner, and an
overflow rule. The default runtime window is 24,576 tokens. The supported
lower bound is 16,384 tokens, which remains usable by compacting earlier
and leaving a smaller live log.

## The Ledger

| Region | Cap (tokens) | Owner | On overflow |
| --- | --- | --- | --- |
| generation reserve | 2,048 | harness | never occupied; subtracted first |
| prefix: identity and rules | 768 | [../protocol/system-prompt.md](../protocol/system-prompt.md) | build-time check fails |
| prefix: grammar and tool registry | 1,024 | [../protocol/system-prompt.md](../protocol/system-prompt.md) | build-time check fails |
| prefix: skill index | 512 | [../skills/loading.md](../skills/loading.md) | oldest-stamped entries drop to name-only |
| prefix: workspace brief | 1,024 | [../sandbox/workspace.md](../sandbox/workspace.md) | head kept, truncation notice |
| prefix: memory digest | 2,048 | [../memory/distillation.md](../memory/distillation.md) | digest builder must rank within cap |
| log: owner frame | 4,096 each | [../runtime/queue-intake.md](../runtime/queue-intake.md) | head kept, truncation notice |
| log: observation | 2,048 each | [../tools/registry.md](../tools/registry.md) | head and tail kept, middle elided, truncation notice |
| log: skill body | 2,048 each | [../skills/format.md](../skills/format.md) | source validation rejects oversized skills |
| log: loaded skills concurrent | 6,144 total | [../skills/loading.md](../skills/loading.md) | skill.use refused with notice |
| soft compaction trigger | 18,432 used | [compaction.md](compaction.md) | narrow observations; preemptive compaction may run |
| hard compaction trigger | 21,504 used | [compaction.md](compaction.md) | compaction before owner delivery or endpoint call |
| post-compaction target | 8,192 | [compaction.md](compaction.md) | compaction must reach it or fail loudly |

## Rules

- Caps are enforced by the harness before frames are appended; the model
  never causes an endpoint overflow error.
- Truncation is always marked with a notice naming what was cut and how to
  retrieve the rest (a ranged fs.read, a narrower shell command, a
  memory.find query).
- The prefix total is 5,376 tokens. With the default 24,576-token window and
  2,048-token reserve, 17,152 tokens remain for the live log.
- With a 16,384-token window and 2,048-token reserve, 8,960 tokens remain
  for the live log. The derived policy is soft trigger 12,288, hard trigger
  13,312, and post-compaction target 7,424.
- `context.trigger` is accepted only when it is below `window - reserve`
  and above the post-compaction target. Omitted or stale trigger values are
  derived from the selected window instead of preserving larger-window
  numbers.
- Configs below a 16,384-token window fail loudly. Configs whose prefix plus
  reserve leave less than 4,096 log tokens also fail.
- Budget arithmetic lives in pure functions in lkjagent-context with
  table-driven tests; no budget decision happens inside an IO adapter.

## Status

implemented.
