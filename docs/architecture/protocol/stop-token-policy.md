# Stop Token Policy

## Purpose

Define how provider stop handling interacts with the `<act>...</act>` action
contract and how closure repair is represented.

## Policy

Provider requests use `</act>` as the stop sequence. Some providers omit the
matched stop text from `assistant.content`. The wire decoder restores the suffix
only when all of these conditions are true:

- finish reason is `stop`.
- content contains `<act>`.
- content does not already contain `</act>`.

The restored content is the normalized content passed to the parser. The raw
response body remains available to provider exchange logging.

## Closure Modes

Parser logging records one closure mode per turn:

- `Natural`: the assistant content already contained a closed action block.
- `StopSequenceClosed`: the wire decoder appended `</act>` after provider stop.
- `Unclosed`: no deterministic closure was available.

`StopSequenceClosed` is a visible repair. It is not silent parser forgiveness.
The parse record stores the raw content hash, normalized content hash, and the
finish reason that justified the repair.

## Fault Handling

A length-limited response without a closed action is a completion-oversize fault.
An empty content response is `EmptyContent`. A response with no `<act>` block is
`MissingActBlock`. A response with more than one action block is
`MultipleActBlocks` unless a single admitted tool carries its own internal batch
payload.

Only `assistant.content` is parsed. Provider reasoning fields are logged and
ignored for action dispatch.

## Prompt Contract

The prompt frame still requires exactly one action block. It never asks the
model to omit the closing tag. The stop sequence is a transport detail between
the provider client and the wire decoder.

## Invariants

- Stop suffix restoration applies only to provider `stop` finishes.
- Stop suffix restoration never creates a missing tool, parameter, or action.
- Every restored suffix is recorded in `parsed-action.json`.
- A repaired action still passes normal parser, schema, admission, repeat, and
  payload gates before dispatch.
- The raw provider response is preserved in the provider exchange log.

## Verification

Tests cover natural closure, restored stop closure, length finish without
closure, empty content, missing action, multiple actions, and reasoning fields
that contain action-like text.

## Status

implemented for provider requests, wire decoding, and per-turn parse logging.
The provider request includes `</act>` as a stop sequence, the wire decoder
restores stripped closure for provider stop finishes, and `parsed-action.json`
records the resulting closure mode.
