# Output Budget

## Purpose

Bound each model turn so the local endpoint usually returns one valid action.
Large deliverables scale through semantic micro-batches, not larger completions.

## Default

The default maximum output is 512 tokens. It is configurable through the same
context reserve path used by endpoint request construction, but repository
defaults stay compact.

## Prompt Contract

The runtime card names the active budget, for example:

```text
<budget>512 output tokens</budget>
```

The exact next action must fit the budget. `artifact.next` examples should name
one rich file or a very small path-specific batch.

## Oversize Recovery

A provider `finish_reason=length` without a complete action is a completion
oversize fault. Recovery changes the next shape to a smaller action such as
`artifact.next`, a one-file `fs.batch_write`, or deterministic audit. It must
not retry the same oversized batch.

## Rules

- Do not raise the default cap to make long content fit one turn.
- Use `artifact.next` to choose the next weak path.
- Use `fs.batch_write` only for one rich file or a tiny semantic batch.
- Stop generation at `</action>`.
- Record output token usage when the provider reports it.

## Verification

Focused tests prove the default request `max_tokens` is compact and an oversize
fixture routes to a smaller next action.

## Status

open for this redesign.
