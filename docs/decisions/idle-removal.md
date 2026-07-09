# Quiescence Is Derived

## Purpose

Record that waiting for work creates no synthetic runtime object.

## Context

The daemon serves owner work and scheduled deterministic maintenance. Empty
polls are neither progress nor evidence.

## Decision

Quiescence is derived when no eligible operation, due wake, pending owner turn,
interrupted effect, or due maintenance exists. The daemon updates its lease and
waits without an endpoint call, workspace inspection, memory rewrite, or
self-assigned matter.

## Consequences

Waiting owner and waiting external conditions remain durable and visible.
Scheduled maintenance uses ordinary operations, budgets, checks, and effect
settlement. Elapsed quiescence cannot satisfy campaign duration or matter
completion.

## Rejected Alternatives

A stored idle operation or self-created maintenance matter could overwrite the
last meaningful state and create false progress.
