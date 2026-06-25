# Provider Anomalies

## Purpose

Define provider and wire shapes that are not model actions. These anomalies are
classified before action parsing so they cannot loop as ordinary parse faults.

## Boundary

The provider may return text, usage accounting, finish reasons, hidden reasoning
fields, malformed messages, or no usable assistant content. The runtime treats
all of that as evidence about the endpoint exchange. Only assistant `content`
that survives this classifier can become `RawModelText` for the action parser.

## Classes

| Class | Signature | Route |
| --- | --- | --- |
| `empty_content_with_usage` | assistant content is empty and completion tokens or provider output token counts are greater than zero | provider anomaly event, endpoint recovery, no parse retry |
| `empty_content_no_usage` | assistant content is empty and output token count is zero or unknown | provider anomaly event, endpoint recovery or blocked handoff after retry budget |
| `missing_content_field` | message exists but has no string `content` field | malformed provider message event |
| `reasoning_only_response` | reasoning or thought fields exist but assistant content is empty | provider anomaly event; reasoning is logged as evidence only |
| `malformed_provider_message` | choices, message, finish reason, or usage fields have unsupported types | endpoint fault with raw redacted evidence |
| `tool_call_only_response` | provider-native tool call fields exist without assistant content | malformed provider message event; product tool calls are not accepted |

## Rules

- Provider anomalies are detected in `lkjagent-llm` before protocol parsing.
- Provider anomaly records include finish reason, usage, content byte count,
  raw-message shape summary, and a redacted response hash.
- Reasoning fields are never concatenated into action text.
- Provider-native tool call fields are never converted into product actions.
- `empty_content_with_usage` is not `MissingActionEnvelope`.
- Endpoint recovery may retry the exact same request only under the endpoint
  retry budget. After the budget, the runtime records a blocked handoff or
  provider failure notice with the current case, turn, and prompt frame ids.
- A provider anomaly does not increment the parser repeat-fault budget.
- The next runtime event is `provider_anomaly`, not `parse_fault`.

## Current Fixture

The active Chronos turn at
`data/logs/model/epoch-1782344195/case-1/turn-000019` is an
`empty_content_with_usage` fixture: `response.json` has empty `content`,
`finish_reason=stop`, `closure_mode=Unclosed`, and `completion_tokens=485`.
Its `parsed-action.json` currently records `MissingActionEnvelope`; that is the
bug this contract corrects.

## Verification

Focused tests must prove:

- empty content plus nonzero completion tokens returns `empty_content_with_usage`;
- reasoning-only content is logged and not parsed as an action;
- missing or non-string content returns `missing_content_field` or
  `malformed_provider_message`;
- provider anomalies write provider-exchange evidence without admission or
  observation success records;
- provider anomalies route through endpoint recovery rather than parse recovery.

## Status

open. The active log proves the contract is not yet implemented.
