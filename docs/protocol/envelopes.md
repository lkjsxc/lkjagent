# Envelopes

## Purpose

Define the block envelopes accepted from the model for a persisted decision.

## Envelope Selection

Exactly one block is accepted per model turn. The `RuntimeDecision` fixes the
expected block, stop sequence, and tool view when a tool call is allowed.

| Block | Common operation | Content |
| content block | write or revise | full file body |
| plan block | plan helper | plan lines |
| JSON action block | tool operation | one JSON tool call |
| message block | respond or ask | owner-facing prose |
| verdict block | judged check | pass or fail plus reason |

Known helpers use these blocks, but future operation keys may select the same
blocks without becoming a new central step enum.

## Tool Call Completion

Finish-like operations are not separate envelopes. They are JSON tool calls only
when the current `ToolSetView` exposes them. A valid finish action carries the
schema discriminator, decision id, tool name `finish`, summary args, and the
context-frame fingerprint inside the dedicated action delimiter.

## Tool Call Cards

A tool-call prompt card shows the decision id, context-frame fingerprint,
decision-visible tools, and either a safe filled JSON example or a schema-only
shape. A safe filled example is copyable only when the same `RuntimeDecision`
can parse and admit it. A schema-only shape is labelled non-copyable; unchanged
placeholder values such as `FIELD_VALUE` must parse to a known admission
rejection before any effect runs.

## Ask Semantics

An ask decision expects a message block. Accepting that message records a
question event, clean context item, and waiting state. A tool operation cannot
emit an ask block unless the decision selected that grammar.

## Rules

No prose may appear outside the envelope. The JSON action body must carry a
schema discriminator, decision id, tool name, args object, and context-frame
fingerprint. Duplicate keys at any object level, unknown top level fields,
missing required args, unknown args, wrong primitive types, stale decision ids,
and tools absent from the persisted decision view are faults. Placeholder values
such as `...`, `PATH`, `TODO`, `VALUE`, `FIELD_VALUE`, `<path>`, or `[path]`
are not executable and are rejected before effects. Implicit envelopes and old
XML tool fields are not part of the active daemon prompt protocol.

## JSON Action Parser

The active action parser accepts one JSON action object inside the dedicated
action delimiter pair. The parser returns typed errors and never executes a
partially parsed action.

## Examples

Valid write:

```text
<content>
Daily report body.
</content>
```

Invalid write fault `wrong_block`:

```text
<message>The report is ready.</message>
```

## Repair Cards

A retry prompt names the decision id, expected JSON shape, bounded fault,
invalid-excerpt hash, allowed tool, and required change. It never quotes the
full failed body into normal context.

## Failure This Prevents

The parser either accepts the expected decision block or returns a bounded fault;
it does not guess, repair, or normalize conflicting formats.
