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
  retry budget. After three consecutive provider anomalies, the daemon stops
  retrying, clears the retry deadline, pauses the active task, and records a
  provider failure notice with the current case, turn, and prompt frame ids.
- A provider anomaly does not increment the parser repeat-fault budget.
- The next runtime event is `provider_anomaly`, not `parse_fault`.

## Fixtures

The historical Chronos turn at
`data/logs/model/epoch-1782344195/case-1/turn-000019` is an
`empty_content_with_usage` regression fixture: `response.json` has empty
`content`, `finish_reason=stop`, `closure_mode=Unclosed`, and
`completion_tokens=485`. It shows the old bug where the parser recorded
`MissingActionEnvelope`.

The active checked-in run also contains turn `000078` under
`data/logs/model/epoch-1782440081/case-none/`. Its `response.json` records
`provider_anomaly.kind=reasoning_only_response` and no `parsed-action.json`,
`admission.json`, or `observation.txt` is listed in the export manifest.

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

partially implemented. `lkjagent-llm` classifies empty, missing, malformed,
reasoning-only, and tool-call-only response shapes before action parsing.
Runtime provider anomaly handling records endpoint retry state without
incrementing parse-fault counters, converts classifier output into a kernel
`provider_anomaly` event in focused coverage, and pauses after the provider
anomaly retry budget is exhausted. Full kernel-owned blocked handoff policy
remains open.
