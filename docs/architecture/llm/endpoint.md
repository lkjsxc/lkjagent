# Endpoint

## Purpose

Specify the wire contract between the harness and the chat-completions route:
the exact request fields sent, the exact response fields read, the
non-streaming decision, and the mapping of endpoint failures onto
[../protocol/recovery.md](../protocol/recovery.md).

## One Crate Touches the Wire

The endpoint client lives in the lkjagent-llm crate, and that crate is the
only place in the workspace that sends HTTP to the model server. Every other
crate hands it a message list and receives a completion or a classified
error. The endpoint base URL and model name come from
LKJAGENT_ENDPOINT_URL and LKJAGENT_MODEL, falling back to
data/lkjagent.json when those variables are unset. An
optional API key is read from LKJAGENT_API_KEY. The decision to depend on
exactly one OpenAI-compatible endpoint is
[../../decisions/openai-endpoint.md](../../decisions/openai-endpoint.md).
The request timeout defaults to 180 seconds and is set by
endpoint.timeout-seconds or LKJAGENT_ENDPOINT_TIMEOUT_SECONDS.

## Request Subset

The harness sends exactly these fields and no others:

| Field | Value | Why |
| --- | --- | --- |
| model | configured model name | selects the model on a server that may host several |
| messages | system, user, assistant; plain-string content | frame mapping per [layout.md](../context/layout.md) |
| max_tokens | context.reserve, default 2048 | the generation reserve in [layout.md](../context/layout.md) |
| temperature | 0.3 | precision over creativity, per [sampling.md](sampling.md) |
| top_p | 0.9 | per [sampling.md](sampling.md) |
| stop | `</action>` | stops generation after one action envelope |
| stream | false | whole completions only, see below |

No tools field, no response-format field, no logit bias: the action protocol
lives in the message text, and the parser owns it. OpenAI-compatible stop
sequences remove the matched close tag, so the client restores `</action>` when
finish_reason is stop, the content contains `<action>`, and the close tag is
absent.

## Non-Streaming, Deliberately

stream is false because nothing consumes partial tokens:

- There is no UI to feed token-by-token; the CLI reads completed events from
  the store.
- Whole-completion handling keeps the client and the recovery logic small: a
  turn either produced a completion or it did not, and a retry re-sends one
  request instead of resuming a stream.

## Response Subset

The harness reads exactly these fields:

| Field | Feeds |
| --- | --- |
| choices[0].message.content | the model turn handed to the parser |
| choices[0].finish_reason | stop is expected; length is accepted only when one complete action is already present |
| usage.prompt_tokens | the token ledger in [layout.md](../context/layout.md) |
| usage.completion_tokens | the same ledger |
| cache metrics, where provided | the transcript, for observability |

A finish_reason of length without a complete action is the completion-oversize
case in [../protocol/recovery.md](../protocol/recovery.md): it is not retried
as an endpoint outage. If the response already contains one closed action block,
the daemon accepts it and lets the action parser own validation. Cache metrics
means server-side data such as llama.cpp timings and prompt cache hit counts;
the daemon records them into the transcript so cache health is visible as
numbers, and their absence is tolerated.

## Error Mapping

Every failure classifies onto the taxonomy in
[../protocol/recovery.md](../protocol/recovery.md):

| Failure | Recovery class | Response |
| --- | --- | --- |
| connection refused or timeout | endpoint error | capped exponential backoff |
| malformed response body | endpoint error | the same backoff |
| finish_reason length without closed act | completion oversize | error with preview plus recovery notice; next turn asks for one short action |
| HTTP 4xx on context overflow | endpoint overflow | error event, forced compaction, incident memory row |

A context overflow surfacing as 4xx is treated as a harness bug, not an
endpoint fault: the window math failed, and the incident memory row keeps the
evidence. Retries re-send the identical request bytes, which preserves the
prefix cache ([caching.md](../context/caching.md)), and nothing is appended
to the context until a completion arrives.

Backoff is pure and owned by lkjagent-llm: attempt 0 waits 1 second, each
later attempt doubles the delay, and the cap is 15 minutes.

## Status

implemented.
