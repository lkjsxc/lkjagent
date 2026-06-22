# Owner Questions

## Purpose

Define when the running product may ask the owner instead of continuing inside
runtime authority.

## Gate

`agent.ask` is admitted only when the snapshot says a concrete external owner
input is required. That fact means all of these are true: the missing
information is outside the repository, workspace, runtime state, tools, and
current evidence; no admitted inspection, audit, repair, smaller scope,
verification, recovery, or partial handoff can progress; a wrong assumption
would materially harm owner-visible work or require an unsupplied secret,
credential, endpoint, path, preference, or fact; and the question is minimal.

## Refusals

The runtime refuses questions about internal tool choice, parse or schema
retry, inspection, audits, placeholder repair, weak-path continuation,
compaction, maintenance preemption, and missing-evidence completion. Refusal
returns one internal next action or a blocked handoff route.

## Ownership

The model proposes `agent.ask` as intent only. The dispatcher validates the
question text, and runtime admission must agree before the task can wait on the
owner. Prompt text may explain the rule but cannot grant permission.

## Status

partially implemented.
