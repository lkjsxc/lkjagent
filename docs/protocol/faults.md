# Faults

## Purpose

Define parse faults and retry hints for model output.

## Fault Taxonomy

| Fault | Meaning | Retry hint |
| --- | --- | --- |
| `wrong_block` | missing or different envelope | restate expected tags |
| `unclosed` | closing tag absent before EOF | shorten to fit budget |
| `empty` | expected body is blank | restate minimum content |
| `unknown_tool` | explore tool not in registry | list legal tool names |
| `bad_params` | missing or unknown action parameter | show exact tool form |
| `bad_plan_line` | unparseable or invalid plan line | quote failing line and grammar |

The fault count is owned by `protocol.faults.count=6`.

## Attempt Effect

Every parse fault records an attempt with outcome `parse_fault`, the fault name,
and one-line diagnosis. The raw faulty output is stored in exchange logs but is
not injected into the retry prompt.

## Envelope Desk Check

- Valid `<content>` for a write step parses as content.
- `<message>` during a write step yields `wrong_block`.
- A missing closing `</content>` yields `unclosed`.
- Empty `<content></content>` yields `empty`.
- An explore action with `<tool>graph.state</tool>` yields `unknown_tool`.
- A plan line without `words=` yields `bad_plan_line`.

## Failure This Prevents

Faults become bounded data for the retry ladder, not more prompt text that
teaches the model to repeat the faulty body.
