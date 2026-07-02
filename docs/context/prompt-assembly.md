# Prompt Assembly

## Purpose

Define the prompt layout for one model call.

## Principle

A prompt is a projection of durable state for the active step. It is not an
append-only transcript. Failed model output is never quoted back, except for
bounded deliberate inputs such as a revise target or continuity tail.

## Layout

| Region | Cap |
| --- | ---: |
| identity and honesty | `context.system.identity-tokens=250` |
| output grammar for the selected kind | `context.system.grammar-tokens=300` |
| objective and task brief | `context.system.brief-tokens=450` |
| plan digest | `context.user.plan-digest-tokens=400` |
| step frame and inputs | `context.user.step-frame-tokens=4000` |
| retry diagnosis | `context.user.retry-frame-tokens=250` |

The whole request is capped by `context.request.hard-cap-tokens=8000`.
Typical scripted work targets `context.request.typical-tokens=6000`.

## Admission Rules

Every region has an owner and a cap. Oversize inputs are truncated
head-and-tail with an explicit marker. Observations are bounded before they
enter the renderer and bounded again during rendering.

## Failure This Prevents

A repeated failure cannot become the strongest token pattern in the next prompt,
because only the diagnosis survives and it is bounded.
