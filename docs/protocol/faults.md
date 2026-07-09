# Faults

## Purpose

Define parse, admission, and repair failures as durable causal data.

## Parse Faults

Stable parse faults include `wrong_envelope`, `unclosed`, `crossed_tag`,
`attribute`, `unknown_tag`, `duplicate_field`, `empty`, `outside_prose`, and
`json_like`. Each records decision, exchange, bounded diagnosis, invalid-output
fingerprint, and required change.

## Admission Faults

Stable admission faults include `stale_decision`, `stale_context`,
`hidden_tool`, `bad_field`, `placeholder`, `path_escape`, `budget_exceeded`,
`duplicate_effect`, and `policy_rejected`. Rejection commits without an effect
journal row.

## Contamination

Raw failed model output stays in restricted exchange evidence and is marked
contaminated. Normal prompts never quote it. Recovery receives only fault class,
fingerprint, bounded diagnosis, tried strategy, and next required change.

## Repair

The first repair restates the exact expected grammar and one concrete valid
shape bound to the current decision. Recurrence must change grammar constraint,
tool view, field set, output unit, or strategy. The same prompt, view, budget,
and failure signature cannot be retried unchanged.

## Settlement

Every failed attempt becomes a runtime event and failure-lineage row. Parse and
admission failures create no effect. An effect fault settles through exactly one
observation before recovery selection.
