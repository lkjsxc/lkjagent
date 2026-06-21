# Recovery Deadlocks

## Purpose

Define fixtures for malformed action loops and blocked recovery escape tools.

## Contract

Known-bad traces repeat parse faults, parameter faults, unavailable preferred
actions, or rejected repeat actions. Known-good traces classify the fault,
render one schema-valid repair example, change action class after budget, and
keep observation or repair tools admitted.

## Required Cases

- `parameter_fault_memory_save`.
- `parse_fault_unclosed_content`.
- `repeat_action_refused`.
- Recovery blocks exact escape tool.

## Pass Condition

The same invalid action is not repeated past budget, and recovery produces
either an admitted repair action or a structured handoff.

## Verification

Run `cargo test -p lkjagent-runtime recovery_controller`.

## Status

partially implemented.
