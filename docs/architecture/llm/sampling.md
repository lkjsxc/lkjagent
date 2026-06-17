# Sampling

## Purpose

Fix the sampling contract: the exact values sent with every request, the
rationale for each, and the rule that they stay constant within a session.

## Initial Contract

| Parameter | Value |
| --- | --- |
| temperature | 0.3 |
| top_p | 0.9 |
| max_tokens | 1024 |
| stop | `["</act>"]` |

## Rationale

- temperature 0.3: action selection is a precision task, not a creativity
  task. The model picks one tool and one set of params per turn, and drift
  here costs a turn. The think preamble gives the model room to explore in
  text before committing, so low temperature does not starve deliberation.
- top_p 0.9: trims the improbable tail without narrowing ordinary word
  choice; it backstops the low temperature rather than competing with it.
- max_tokens 1024: the generation reserve from
  [layout.md](../context/layout.md). One think preamble plus one act block
  fits; a completion that hits this limit is the oversize case in
  [../protocol/recovery.md](../protocol/recovery.md).
- stop `["</act>"]`: the action grammar ends every turn at the act close
  tag, so the server cuts generation there instead of letting the model run
  past its own action.

## Constant Within a Session

Sampling values and the model name are constant within a session. Changing
any of them requires a daemon restart, because some servers key cache state
on them; a mid-session change could silently discard the prefix cache. The
cache discipline is specified in [caching.md](../context/caching.md), and a
config change is a lawful invalidation there precisely because it forces a
restart.

## Ownership

The lkjagent-llm client owns these four values as request constants. The
runtime passes the model name and endpoint base URL into the client; config
file loading belongs to the later runtime and CLI slices.

## Status

implemented.
