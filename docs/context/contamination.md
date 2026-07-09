# Contamination

## Purpose

Define material excluded from normal prompts.

## Classes

Context contamination classes include clean, stale, superseded,
unverified-model-claim, failed-model-output, refused-action, raw-tool-log,
external-raw, recovery-only, and sensitive-owner-data.

## Normal Prompt Rule

Normal prompts include clean current items and bounded unresolved-conflict
summaries. They exclude failed model bodies, refused actions, stale logs,
obsolete proposals, raw large tool output, and unverified model claims.

## Recovery Prompt Rule

Recovery prompts may include bounded structured diagnoses and selected excerpts
when needed. They do not quote large faulty bodies. They mark recovery scope and
the concrete change required.

## Observation Rule

Tool observations are bounded before storage and bounded again before rendering.
Only the current relevant observation is normally admitted. Old observations
remain source evidence, not transcript text. Shell observations are
`external-raw`, failed observations are `recovery-only`, and observations with
secret-like owner data such as tokens or passwords are `sensitive-owner-data`.
The effects edge redacts those token bodies before writing observation or
context candidate text.

## Failure This Prevents

A bad model pattern cannot become the strongest token pattern in the next
prompt.
