# Caching

## Purpose

Specify the prefix-cache discipline: how the harness guarantees that the
endpoint re-evaluates only new tokens on every turn between compactions.

## The Contract

llama.cpp-class servers cache the KV state of the longest matching token
prefix of the previous request. The harness exploits this with one rule:

The serialized request is byte-identical to the previous request plus
appended tokens, except at compaction or restart.

Everything else follows from that rule.

## What Keeps the Bytes Stable

- The prefix is one frozen system message ([layout.md](layout.md)); it is
  rebuilt only at compaction and daemon restart.
- Log frames are immutable once sent; corrections append.
- Serialization is deterministic: stable field order, stable whitespace,
  no timestamps inside frames (times live in the transcript store, not in
  the window).
- Sampling parameters and the model name are constant within a session
  ([../llm/sampling.md](../llm/sampling.md)); changing them requires a
  restart, because some servers key cache state on them.
- The chat template is applied by the server; the harness sends the same
  message-list shape every turn, so templated bytes also extend monotonically.

## Lawful Invalidation

| Event | Why it invalidates | Frequency |
| --- | --- | --- |
| compaction | the prefix is rebuilt by design | per [compaction.md](compaction.md) trigger |
| daemon restart | prefix rebuilt from durable state | rare |
| config change | model, sampling, or budget change | requires restart, owner-driven |
| skill saved | index changes are deferred to next compaction | never mid-session |

Anything else that would invalidate the cache is a bug. The context engine
asserts append-only serialization in tests by diffing consecutive request
bodies; a non-suffix diff fails the test suite.

## Measured Honesty

The endpoint reports prompt token counts and cache hits per request where
supported; the daemon records both in the transcript and exposes them in
`lkjagent status`. If the cache stops hitting, the owner sees it as numbers,
not as vague slowness, per
[../../product/observability.md](../../product/observability.md).

## Status

design-only.
