# Recovery Policy

## Purpose

Define how runtime authority enters and leaves deterministic recovery.

## Decision Owner

`lkjagent-runtime` owns recovery policy. The model receives the selected repair
shape but does not choose retry budgets, fallback timing, or escape tools.

## Inputs

Recovery reads fault class, last invalid action, last valid observation, retry
count, admitted tools, artifact gaps, graph state, and compaction snapshot
state.

## Output

```text
RecoveryPlan
- recovery_class
- previous_mission
- allowed_observation_tools
- allowed_repair_tools
- retry_budget
- forced_next_action
- exact_valid_example
- fallback_action
- partial_handoff
```

## Classes

Classes include parse faults, parameter faults, tool-admission contradictions,
repeat-action faults, payload overflow, audit failures, weak artifact content,
false completion, compaction resume gaps, and maintenance preemption.

## Required Ladders

- Parse faults simplify the prompt to exactly one action.
- Parameter faults render one schema-derived example for the failed tool.
- Payload overflow blocks raw large `fs.write` and moves to batch repair.
- Invalid batch syntax retries once with the canonical example, then switches
  to normalized parse, one-file fallback, or partial handoff.
- Audit failures preserve `artifact.next`, `artifact.audit`, `doc.audit`,
  `fs.read`, `fs.tree`, `fs.write`, and `fs.batch_write` when relevant.
- Repeat faults force a different action shape.

## Prohibited States

- The same invalid action repeats after budget is exhausted.
- Recovery blocks every read, audit, or repair tool that can escape.
- Parameter recovery renders an example dispatch later rejects.
- Recovery enters completion without a repair or structured handoff.
- Recovery asks the model to solve deterministic state bookkeeping.

## Fixture

`parse_fault_unclosed_content`, `parameter_fault_memory_save`,
`uploaded-cookbook-batch-write-schema-fault`, and `repeat_action_refused`
prove the controller changes action shape.

## Verification

Run `cargo test -p lkjagent-runtime recovery_controller`.

## Status

design-only for the full fault-class table.
