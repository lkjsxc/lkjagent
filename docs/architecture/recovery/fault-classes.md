# Fault Classes

## Purpose

Define the recovery classes and the first action shape for each class.

## Classes

| Class | Signature | First Action |
| --- | --- | --- |
| `parse_missing_act` | endpoint output lacks an action block | render one exact valid action |
| `parse_unclosed_tag` | content payload has an unclosed tag | smaller schema-rendered batch |
| `parameter_missing_required` | required field absent | schema example for same tool |
| `parameter_unknown_field` | field not in registry | schema example without field |
| `tool_not_admitted` | dispatch policy refuses tool | select admitted escape tool |
| `preferred_action_contradiction` | preferred action names blocked tool | record policy fault, choose valid action |
| `repeat_action_refused` | same action refused again | change action class |
| `audit_failed` | doc or artifact audit failed | repair exact gaps |
| `weak_artifact_content` | readiness found weak leaves | artifact batch repair |
| `false_completion_attempt` | close requested while gates fail | block close and repair |
| `compaction_resume_missing` | resume lacks required fields | structured handoff or re-observe |
| `maintenance_preempt_required` | owner work appears during maintenance | owner mission |

## Invariants

- Every class has observation tools, repair tools, retry budget, and handoff.
- Audit and weak-content classes enter repair before completion.
- Policy contradictions are recorded as runtime faults.

## Verification

Run `cargo test -p lkjagent-runtime recovery_controller`.

## Status

design-only for the complete table.
