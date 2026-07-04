# Faults

## Purpose

Define parse faults, contamination rules, and retry hints for model output.

## Fault Taxonomy

| Fault | Meaning | Retry hint |
| --- | --- | --- |
| `wrong_block` | missing or different envelope, including prose outside it | restate expected tags |
| `unclosed` | closing tag absent before EOF | shorten to fit budget |
| `empty` | expected body is blank | restate minimum content |
| `unknown_tool` | action tool absent from the decision view | list rendered tool names |
| `bad_params` | missing, duplicate, or unknown action parameter | show exact tool form |
| `bad_plan_line` | unparseable or invalid plan line | quote failing line and grammar |

Fault names are stable data, but the parser validates against the current
`RuntimeDecision`, not a hidden global registry.

## Attempt Effect

Every parse fault records an event with outcome `parse_fault`, the fault name,
decision id, and one-line diagnosis. The raw faulty output is stored in exchange
logs and marked contaminated. Normal retry prompts include only bounded
diagnosis and required change.

## Envelope Desk Check

- Valid `<content>` for a write decision parses as content.
- Leading or trailing prose outside `<content>` yields `wrong_block`.
- `<message>` during a write decision yields `wrong_block`.
- A missing closing `</content>` yields `unclosed`.
- Empty `<content></content>` yields `empty`.
- An action with a tool absent from the decision view yields `unknown_tool`.
- An action with duplicate `<path>` parameters yields `bad_params`.
- An action with an unsupported parameter yields `bad_params`.
- A plan line without `words=` yields `bad_plan_line`.

## Failure This Prevents

Faults become bounded data for the retry ladder, not more prompt text that
teaches the model to repeat the faulty body.
