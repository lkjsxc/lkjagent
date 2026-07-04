# Prompt Assembly

## Purpose

Define the prompt layout for one model call from a persisted runtime decision.

## Principle

A prompt is a projection of durable context items and active state cells selected
by `RuntimeDecision`. It is not an append-only transcript. Failed model output is
never quoted back, except through bounded recovery-only diagnoses.

## Frame Inputs

Prompt assembly consumes the persisted decision id, expected envelope,
`ToolSetView`, context item selection, active state payloads, retry or recovery
policy, and budget caps. The resulting `PromptFrame` stores the decision id,
prompt fingerprint, context-frame fingerprint, tool-view fingerprint, and body
or body refs.

## Layout

| Region | Cap |
| --- | ---: |
| identity and honesty | `context.system.identity-tokens=250` |
| output grammar from decision | `context.system.grammar-tokens=300` |
| active state and objective | `context.system.state-tokens=650` |
| clean context items | `context.user.context-tokens=1600` |
| unresolved conflicts | `context.user.conflict-tokens=600` |
| tool view, when present | `context.user.tool-view-tokens=1200` |
| recovery or retry diagnosis | `context.user.recovery-tokens=250` |

The whole request is capped by `context.request.hard-cap-tokens=8000`.

## Admission Rules

Every region has an owner and a cap. Oversize items are truncated head-and-tail
with an explicit marker. Contaminated items are excluded from normal prompts.
Contradictions render only as unresolved-conflict summaries until resolved.
Observations are bounded before storage and bounded again during rendering.

## Failure This Prevents

A repeated failure cannot become the strongest token pattern in the next prompt,
and the model cannot see tools or facts outside the persisted decision frame.
