# Overview

## Purpose

One-page map of lkjagent: the data flow, which crate owns which subsystem,
and the words every other document uses.

## Data Flow

```
owner --(lkjagent send)--> store.queue
store.queue --(turn boundary)--> graph case intake
graph case intake --> agent loop
agent loop --(graph-sliced messages)--> endpoint (chat completions, configurable window)
endpoint --(tag-based action, stops at </action>)--> parser
parser --> toolset (fs, shell, queue ops, memory ops, graph ops, control)
toolset --(bounded observation)--> graph evidence --> agent loop --> store.events
empty queue --> bounded graph maintenance --> agent loop
```

The loop appends every turn to the transcript; the graph decides phase,
legal transitions, evidence requirements, and selected context packages; the
context engine decides how the selected slice fits. Compaction preserves
graph state and rebuilds the prefix.

## Crate Ownership

| Crate | Owns | Contract |
| --- | --- | --- |
| lkjagent-protocol | action grammar parse and render (pure) | [protocol/](protocol/README.md) |
| lkjagent-graph | state graph, task cases, transition gates, evidence (pure) | [state-graph/](state-graph/README.md) |
| lkjagent-context | window layout, budgets, compaction decisions (pure) | [context/](context/README.md) |
| lkjagent-store | SQLite access: queue, events, memory, state | [memory/](memory/README.md) |
| lkjagent-llm | endpoint HTTP client | [llm/](llm/README.md) |
| lkjagent-tools | tool execution adapters | [tools/](tools/README.md) |
| lkjagent-runtime | daemon, loop composition, intake, maintenance helpers | [runtime/](runtime/README.md) |
| lkjagent-cli | the lkjagent binary | [../product/cli.md](../product/cli.md) |
| lkjagent-xtask | repository checks and quiet gates | [../operations/verification.md](../operations/verification.md) |

The workspace layout is owned by [../repository/layout.md](../repository/layout.md).

## Glossary

| Term | Meaning |
| --- | --- |
| harness | lkjagent itself: daemon, CLI, and crates |
| daemon | the long-lived process started by lkjagent run |
| loop | the single continuous agent loop inside the daemon |
| turn | one model call producing one action, plus its observation |
| task | unit of owner work: opened by a queue message, closed by agent.done |
| queue | persistent owner-message queue in the store |
| store | the SQLite database: queue, events, memory, state |
| transcript | the append-only event log in the store |
| event | one transcript row: owner, action, observation, notice, queue mutation, compaction |
| memory | distilled durable knowledge rows with a full-text index |
| graph case | durable task record: objective, phase, node, plan, evidence, and completion gate |
| graph slice | compact prefix section naming active node, legal transitions, packages, and missing evidence |
| context package | source-owned graph-selected instruction fragment |
| prefix | stable system message: identity, protocol, registry, graph slice, brief, digest |
| log | the appended task messages after the prefix |
| observation | bounded tool result injected after an action |
| notice | harness-injected frame: delivery, error, truncation, budget, compaction |
| compaction | explicit event distilling the log and rebuilding the prefix |
| endpoint | the OpenAI-compatible chat-completions server |
| workspace | the mounted directory tree the agent operates on |
| workspace brief | the workspace's own AGENTS.md, loaded into the prefix |
| digest | the memory digest: compact rendering of high-value memory in the prefix |
| gate | a verification command; quiet gates print exactly: ok and the gate name |

## Invariants

- One daemon per store; one loop per daemon; one endpoint per config.
- The serialized message list is append-only between compactions.
- Every byte in the window is attributable to a hygiene allowlist entry.
- Everything observable is reconstructible from the store plus source graph definitions.
