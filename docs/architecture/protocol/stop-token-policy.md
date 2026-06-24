# Stop Token Policy

## Purpose

Define how provider stop handling interacts with the `<action>...</action>`
contract and how closure repair is represented.

## Policy

Provider requests use `</action>` as the stop sequence. Some providers omit the
matched stop text from `assistant.content`. The wire decoder restores the suffix
only when all conditions are true:

- finish reason is `stop`.
- content contains `<action>`.
- content does not already contain `</action>`.

The restored content is the normalized content passed to the parser. The raw
response body remains available to provider exchange logging.

## Closure Modes

Parser logging records one envelope mode per turn:

- `Natural`: assistant content already contained one closed action envelope.
- `StopSequenceClosed`: the wire decoder appended `</action>` after provider stop.
- `ImplicitActionEnvelope`: strict missing-opening normalization accepted one body.
- `Unclosed`: no deterministic closure was available.

`StopSequenceClosed` and `ImplicitActionEnvelope` are visible repairs. They are
not silent parser forgiveness. The parse record stores the raw content hash,
normalized content hash, finish reason, and mode that justified the repair.

## Fault Handling

A length-limited response without one closed action is a completion-oversize
fault. An empty content response is `EmptyContent`. A response with no action
envelope is `MissingActionEnvelope` unless strict implicit normalization
accepts one complete body. A response with more than one action envelope is
`MultipleActionEnvelopes`.

Only `assistant.content` is parsed. Provider reasoning fields are logged and
ignored for action dispatch.

## Prompt Contract

The prompt frame still requires exactly one action envelope. It never asks the
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
closure, empty content, missing action, multiple actions, implicit envelope
logging, and reasoning fields that contain action-like text.

## Status

partially implemented. The provider request and wire decoder still need the live
stop sequence changed to `</action>` and the parse log modes aligned with this
contract.
