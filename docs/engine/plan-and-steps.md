# Plan And Steps

## Purpose

Define the ordered plan, step records, attempts, and allowed plan mutation.

## Step Record

The plan is the ordered `steps` rows for a task. Exactly one step is active at a
time unless the engine skips past a blocked independent step.

| Field | Meaning |
| --- | --- |
| `id`, `task_id`, `ordinal` | identity and order |
| `kind` | `plan`, `write`, `revise`, `explore`, `verify`, `respond`, or `ask` |
| `title` | short label for status and prompts |
| `instruction` | step-specific ask |
| `inputs_json` | paths, references, outline beats, targets, and flags |
| `output_path` | exact write or revise target when applicable |
| `checks_json` | attached checks from [completion.md](completion.md) |
| `state` | `pending`, `active`, `done`, `blocked`, or `skipped` |
| `attempts_used` | count against `engine.attempts-per-step=3` |

## Attempt Record

One endpoint call for one step creates an attempt with prompt fingerprint,
exchange-log path, outcome, diagnosis, and token usage. Consecutive fingerprints
for the same failed step must differ; this is enforced by
`engine.prompt-fingerprint.must-change=true`.

## Plan Mutation

The model proposes plan lines only during plan steps. The engine validates each
path, check, word target, and budget before materializing steps.

Automatic mutation is limited:

- split a divisible write step after exhausted attempts;
- narrow an explore step after exhausted attempts;
- extend a content step when checks measure a shortfall;
- mark a step blocked and continue with independent work;
- synthesize one task review, capped by `engine.reviews-per-task=1`.

All mutation follows [retry-and-escalation.md](retry-and-escalation.md).

## Failure This Prevents

Path drift is impossible for scripted work because the path sits in the step
record before the model writes content. The model cannot choose a sibling path
that the harness later refuses.
