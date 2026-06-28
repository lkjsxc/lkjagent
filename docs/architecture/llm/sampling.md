# Sampling

## Purpose

Fix the sampling contract: the exact values sent with every request, the
rationale for each, and the rule that they stay constant within a session.

## Initial Contract

| Parameter | Value |
| --- | --- |
| temperature | 0.3 |
| top_p | 0.9 |
| max_tokens | 512 default |
| stop | `</action>` |

## Rationale

- temperature 0.3: action selection is a precision task, not a creativity
  task. The model picks one tool and one set of params per turn, and drift
  here costs a turn.
- top_p 0.9: trims the improbable tail without narrowing ordinary word
  choice; it backstops the low temperature rather than competing with it.
- max_tokens: `context.reserve`, default 512, from
  [output-budget.md](output-budget.md). One action envelope fits; a completion
  that hits this limit is the oversize case in
  [../protocol/recovery.md](../protocol/recovery.md).
- stop `</action>`: generation ends after one action envelope instead of
  drifting into prose or another action. The client restores the stripped
  close tag before parsing.

## Constant Within a Session

Sampling values and the model name are constant within a session. Changing
any of them requires a daemon restart, because some servers key cache state
on them; a mid-session change could silently discard the prefix cache. The
cache discipline is specified in [caching.md](../context/caching.md), and a
config change is a lawful invalidation there precisely because it forces a
restart.

## Ownership

The lkjagent-llm client owns these values as request constants. The
runtime passes the model name and endpoint base URL into the client; config
file loading belongs to the later runtime and CLI slices.

## Status

implemented.
