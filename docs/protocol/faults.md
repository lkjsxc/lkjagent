# Faults

## Purpose

Define parse faults, contamination rules, and retry hints for model output.

## Fault Taxonomy

| Fault | Meaning | Retry hint |
| --- | --- | --- |
| `wrong_block` | missing or different selected envelope, including outside prose | restate expected tags |
| `unclosed` | closing tag absent before EOF | close the named tag |
| `crossed_tag` | nested tags close out of order | preserve stack order |
| `attribute` | tag contains attributes | remove attributes and use child tags |
| `unknown_tag` | tag is not allowed in this envelope | remove or rename the tag |
| `duplicate_tag` | scalar tag appears more than once | keep one scalar value |
| `empty` | expected simple block body is blank | provide required content |
| `unknown_tool` | tool name absent from the decision view | choose a rendered tool name |
| `bad_params` | missing, unknown, or wrong argument values | show exact argument tags |
| `json_like` | action body starts as JSON or embeds braced data syntax | rewrite as XML-like tags |
| `bad_plan_line` | bridge plan line is unparseable or invalid | quote failing line and grammar |

Fault names are stable data, but the parser validates against the current
`RuntimeDecision`, not a hidden global registry.

## Attempt Effect

Every parse fault records an attempt with outcome `parse_fault`, the fault name,
decision id, a one-line diagnosis, and a `recovery.failure` state cell. The raw
faulty output is stored in exchange logs and marked contaminated. Normal retry
prompts include only bounded diagnosis, invalid-excerpt hash, required change,
and the minimal corrected XML-like shape when the decision expects an action.

## Envelope Desk Check

- Valid content for a write decision parses as content.
- Leading or trailing prose outside content yields `wrong_block`.
- A message during a write decision yields `wrong_block`.
- A missing closing content tag yields `unclosed`.
- Empty content yields `empty`.
- An action with attributes yields `attribute`.
- An action with duplicate `tool_name` yields `duplicate_tag`.
- An action with `{ "tool_name": ... }` yields `json_like`.
- An action with a tool absent from the decision view yields `unknown_tool`.
- An action with an unsupported argument yields `bad_params`.
- A bridge plan line without required fields yields `bad_plan_line`.

## Failure This Prevents

Faults become bounded data for the retry ladder, not more prompt text that
teaches the model to repeat the faulty body.
