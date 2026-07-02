# Idle Removal

## Purpose

Record the decision that idle does no autonomous maintenance.

## Context

The daemon serves owner-queued work. Self-assigned maintenance work consumes
endpoint budget without an owner objective.

## Decision

When there is no open task and no pending queue row, the daemon updates
heartbeat state and waits. It does not call the endpoint, inspect the workspace,
or rewrite memory.

## Consequences

Owner-visible work starts only from queue messages. Self-improvement work is an
ordinary owner task.

## Rejected Alternatives

Keeping idle maintenance would require cooldowns, directives, and a second set
of completion rules for work the owner did not request.
