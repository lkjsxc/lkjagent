# Current Blockers

## Purpose

The dependency-ordered implementation queue. Each row blocks every row
below it unless its task file states otherwise. A session takes the first
open row. Rows move to done only with the evidence their task file's
Acceptance section names.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 6 | No endpoint client | [tasks/llm-client.md](tasks/llm-client.md) | open |
| 7 | No skill validator, index, or loader | [tasks/skill-runtime.md](tasks/skill-runtime.md) | open |
| 8 | No tool dispatch or adapters | [tasks/tool-runtime.md](tasks/tool-runtime.md) | open |
| 9 | No agent loop composing the crates | [tasks/agent-loop.md](tasks/agent-loop.md) | open |
| 10 | No binary: daemon entry, send, status, log | [tasks/queue-cli.md](tasks/queue-cli.md) | open |
| 11 | Idle time does nothing | [tasks/self-maintenance.md](tasks/self-maintenance.md) | open |
| 12 | No image, compose services, or CI final gate | [tasks/compose-final-gate.md](tasks/compose-final-gate.md) | open |

## Ordering Notes

- Rows 3 and 4 are pure crates with no workspace dependencies; if row 2 is
  blocked on an owner question, rows 3 and 4 may proceed under the interim
  checks, recorded as such.
- Row 9 is the integration point: it must not start while any of rows 3
  through 8 are open.
- Row 12 closes the loop: after it, every later change verifies through
  the final gate per [../operations/verification.md](../operations/verification.md).

## Done

| # | Blocker | Task | Closing commit |
| --- | --- | --- | --- |
| 1 | No cargo workspace exists | [tasks/bootstrap-workspace.md](tasks/bootstrap-workspace.md) | Enable concrete Rust workspace bootstrap |
| 2 | Doc and style rules are not machine-enforced | [tasks/xtask-checks.md](tasks/xtask-checks.md) | Make repository gates enforce their contracts |
| 3 | The action grammar has no parser or renderer | [tasks/protocol-parser.md](tasks/protocol-parser.md) | Implement strict action parser and renderer |
| 4 | No context engine: budgets, admission, compaction decisions | [tasks/context-engine.md](tasks/context-engine.md) | Implement pure context engine decisions |
| 5 | No store: queue, events, memory, state | [tasks/sqlite-store.md](tasks/sqlite-store.md) | Implement SQLite store boundary |
