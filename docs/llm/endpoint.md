# Endpoint

## Purpose

Define the chat-completions endpoint contract.

## Client

lkjagent calls one OpenAI-compatible chat-completions endpoint. The endpoint URL,
model name, optional API key environment variable, timeout, and context length
come from `data/lkjagent.json` plus environment overrides.

The client sends the selected step prompt, sampling values, max token cap, and
stop sequence. The stop sequence is the expected closing envelope tag for the
selected step.

## Completion Record

The endpoint adapter returns structured completion data to the interpreter:

- response content or a typed endpoint outcome;
- request and response timestamps;
- prompt, completion, and cached token counts when reported;
- provider anomaly details;
- closure mode, including whether pure closing-tag repair was applied.

The interpreter persists the record through `attempts.exchange_ref`,
`token_usage`, exchange files, and bounded event or diagnosis rows. Unknown usage
fields remain unknown.

## Response Handling

The client captures prompt, completion, and cached token usage when the endpoint
returns them. Missing usage is recorded as unknown, not estimated as fact.

If a completion stops before the closing tag but contains a complete body, the
client applies `llm.closure-repair.enabled=true` only when the repair is a pure
closing-tag append for the expected envelope.

## Anomalies

Empty completions, reasoning-only completions, transport failures, and length
truncation become endpoint outcomes. They are retried by the endpoint backoff
policy before the engine consumes an attempt. A model call that reached the
endpoint counts against the task budget even when parsing later fails.
