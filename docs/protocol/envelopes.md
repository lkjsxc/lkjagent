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
| `<finish>...</finish>` | `explore` | finish summary |
| `<message>...</message>` | `respond`, `ask` | owner-facing prose |
| `<verdict>...</verdict>` | judged `verify` | pass or fail plus reason |

The envelope count is owned by `protocol.envelopes.count=6`.

## Rules

No prose may appear outside the envelope. Tags have no attributes. Parameter
names inside `<action>` must be unique. JSON and implicit envelopes are not part
of the protocol.

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
