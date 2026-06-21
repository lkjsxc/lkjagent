# Recovery Policy

## Purpose

Define how runtime authority enters and leaves deterministic recovery.

## Decision Owner

`lkjagent-runtime` owns recovery policy. The model receives the selected repair
shape but does not choose retry budgets or escape tools.

## Inputs

Recovery reads fault class, last invalid action, last valid observation, retry
count, admitted tools, artifact gaps, graph state, and compaction snapshot
state.

## Output

The output names recovery class, allowed observation tools, allowed repair
tools, retry budget, required next action shape, and fallback handoff shape.

## Classes

Classes include parse faults, parameter faults, tool-admission contradictions,
repeat-action faults, audit failures, weak artifact content, false completion,
compaction resume gaps, and maintenance preemption.

## Prohibited States

- The same invalid action repeats after budget is exhausted.
- Recovery blocks every read, audit, or repair tool that can escape.
- Parameter recovery renders an example dispatch later rejects.
- Recovery enters completion without a repair or structured handoff.

## Fixture

`parse_fault_unclosed_content`, `parameter_fault_memory_save`, and
`repeat_action_refused` prove the controller changes action shape.

## Verification

Run `cargo test -p lkjagent-runtime recovery_controller`.

## Status

design-only for the full fault-class table.
