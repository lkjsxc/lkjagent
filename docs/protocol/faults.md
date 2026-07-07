# Faults

## Purpose

Define parse faults, contamination rules, and retry hints for model output.

## Fault Taxonomy

| Fault | Meaning | Retry hint |
| `wrong_block` | missing or different non-action envelope, including prose outside it | restate expected tags |
| `unclosed` | closing tag absent before EOF for simple block envelopes | shorten to fit budget |
| `empty` | expected simple block body is blank | restate minimum content |
| `unknown_tool` | action `tool_name` absent from the decision view | list rendered tool names |
| `bad_params` | invalid simple-block parameters or plan helper fields | show exact form |
| `action_json` | JSON action fault with typed reason | show minimal JSON shape |
| `bad_plan_line` | unparseable or invalid plan line | quote failing line and grammar |

Fault names are stable data, but the parser validates against the current
`RuntimeDecision`, not a hidden global registry.

## Attempt Effect

Every parse fault records an event with outcome `parse_fault`, the fault name,
decision id, and one-line diagnosis. The raw faulty output is stored in exchange
logs and marked contaminated. Normal retry prompts include only bounded
diagnosis, invalid-excerpt hash, required change, and minimal corrected JSON
shape when the decision expects an action.

## Envelope Desk Check

- Valid content for a write decision parses as content.
- Leading or trailing prose outside content yields `wrong_block`.
- A message during a write decision yields `wrong_block`.
- A missing closing content tag yields `unclosed`.
- Empty content yields `empty`.
- A JSON action with a `tool_name` absent from the decision view yields
  `unknown_tool`.
- A JSON action with duplicate keys yields duplicate-key data.
- A JSON action with an unsupported arg yields schema data.
- A plan line without `words=` yields `bad_plan_line`.

## Failure This Prevents

Faults become bounded data for the retry ladder, not more prompt text that
teaches the model to repeat the faulty body.
