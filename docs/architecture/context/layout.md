# Layout

## Purpose

Specify the shape of the context window and its exact mapping onto the
chat-completions message list.

## Two Regions

```
+--------------------------------------------------+
| prefix   (one system message, stable)             |
|   identity and rules                              |
|   protocol grammar and tool registry              |
|   skill index (name + trigger per skill)          |
|   workspace brief (the workspace's AGENTS.md)     |
|   memory digest (rebuilt only at compaction)      |
+--------------------------------------------------+
| log      (appended messages, never edited)        |
|   owner frames, action turns, observations,       |
|   notices, loaded skill bodies                    |
+--------------------------------------------------+
| reserve  (generation headroom, never occupied)    |
+--------------------------------------------------+
```

The prefix changes only at compaction or daemon restart. The log only grows.
The reserve is subtracted before any budget math.

## Message Mapping

| Frame | Chat role | Content shape |
| --- | --- | --- |
| prefix | system | one document per [../protocol/system-prompt.md](../protocol/system-prompt.md) |
| model turn | assistant | optional think preamble plus one act block, verbatim |
| observation | user | `<observation>` block per [../tools/registry.md](../tools/registry.md) |
| owner message | user | `<owner>` block per [../runtime/queue-intake.md](../runtime/queue-intake.md) |
| notice | user | `<notice>` block, kinds in [hygiene.md](hygiene.md) |
| skill body | user | `<skill>` block per [../skills/loading.md](../skills/loading.md) |

Consecutive harness frames between two model turns are concatenated into one
user message in arrival order, so the list strictly alternates assistant and
user after the system message. Serialization is deterministic: same state,
same bytes, which is what makes prefix caching work
([caching.md](caching.md)).

## Immutability

Once a message is sent to the endpoint it is frozen: corrections, truncation
notes, and retractions are new frames that refer to old ones, never edits.
The model may see contradictory history; the newest frame wins, and the
contradiction itself stays visible. This trades a few tokens for a hot cache
and a truthful transcript, deliberately.

## Token Accounting

The engine tracks the token length of every frame at append time using the
endpoint tokenizer counts returned with each completion, plus a conservative
local estimate for unsent frames. The ledger feeds
[budgets.md](budgets.md) and the compaction trigger, and is reported by
`lkjagent status` per [../../product/observability.md](../../product/observability.md).

## Status

design-only.
