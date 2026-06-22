# Non Repetition

## Purpose

Define how recovery prevents repeated invalid actions.

## Contract

Recovery stores an action fingerprint with tool name, normalized parameters,
active mission, and refusal class. A refused fingerprint cannot be selected
again while the same fault class is active unless the retry budget explicitly
allows one schema-repaired retry.

## Alternate Actions

Alternates include graph transition, document audit, artifact audit,
artifact.next, bounded batch write, focused read, or structured handoff. The
alternate must be admitted by authority.

## Invariants

- Repeat refusal changes tool, parameters, or mission.
- Action fingerprints survive compaction.
- Alternate action selection never names a blocked tool.

## Fixture

`tool_admission_graph_plan_contradiction` proves preferred action repair does
not repeat a blocked graph action. The dispatcher repeat-refusal fixture proves
a repeated tool receives a copyable alternate action from the effective policy
instead of only a generic denial.

## Verification

Run `cargo test -p lkjagent-runtime repeat_scope` and
`cargo test -p lkjagent-tools --test repeat_refusal`.

## Implemented Slice

Live dispatcher repeat refusals now name the active mode, forbidden repeated
tool, action-shape change requirement, preferred alternate, and exact registry
example when the effective policy can provide one.

## Status

partially implemented.
