# Envelopes

## Purpose

Define the block envelopes accepted from the model.

## Envelope Set

Exactly one block is accepted per model turn. The selected step kind fixes the
expected block and the stop sequence is its closing tag.

| Block | Used by | Content |
| --- | --- | --- |
| `<content>...</content>` | `write`, `revise` | full file body |
| `<plan>...</plan>` | `plan` | plan lines |
| `<action>...</action>` | `explore` | one tool call |
| `<message>...</message>` | `respond`, `ask` | owner-facing prose |
| `<verdict>...</verdict>` | judged `verify` | pass or fail plus reason |

The envelope count is owned by `protocol.envelopes.count=5`.

## Explore Completion

Explore completion is not a separate envelope. It is an action using the
registered `finish` tool:

```text
<action>
<tool>finish</tool>
<summary>Enough evidence was gathered.</summary>
</action>
```

## Ask Semantics

An `ask` step expects `<message>...</message>`. Accepting that message records a
question event and parks the task as `waiting`. Explore output cannot emit an
ask block or otherwise bypass the planned ask step.

## Rules

No prose may appear outside the envelope. Tags have no attributes. The body may
not be empty. Parameter names inside `<action>` must be unique and must be legal
for the selected tool. Unknown tools and unknown parameters are faults. JSON and
implicit envelopes are not part of the protocol.

## Examples

Valid write:

```text
<content>
The vault opened under the aurora.
</content>
```

Invalid write fault `wrong_block`:

```text
<message>The vault opened.</message>
```

## Failure This Prevents

The parser either accepts the expected block or returns a bounded fault; it does
not guess, repair, or normalize conflicting formats.
