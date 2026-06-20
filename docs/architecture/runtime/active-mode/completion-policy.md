# Completion Policy

## Purpose

Define close admission for each active mode.

## Owner Task

Owner completion requires plan evidence, observation evidence, verification
or audit evidence, no blocking unrecovered fault, and no pending checks unless
the not-run reason is recorded. Content artifacts also require artifact
readiness for the requested root and kind.

## Recovery

Recovery completion is illegal until the fault route is resolved or a blocked
handoff records failed gates, evidence, and a next executable action.

## Maintenance

Maintenance may close with one real maintenance outcome or a bounded no-op
that writes no duplicate memory row and sets cooldown.

## Compaction And Idle

Compaction is runtime-only and cannot be completed by `agent.done`.
`ClosedIdle` has no endpoint action and cannot call `agent.done`.

## Failed Close

A refused close produces a structured handoff naming active mode, failed gate,
missing evidence, existing evidence, and the next admitted action.
