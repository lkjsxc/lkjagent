# Action Format

## Purpose

The grammar of a model turn. The model emits exactly one action envelope.
Line-oriented fields, paired tags, and the JSON `lkj-action` envelope all parse
through the same pure action model before dispatch.

## Line-Oriented Turn

```
<think>
The test expects a trailing newline; the renderer drops it. I will read the
renderer before changing anything.
</think>
<act>
tool: fs.read
path: crates/lkjagent-protocol/src/render.rs
start: 1
count: 60
</act>
```

- The think preamble is optional, free-form, and unparsed; it exists for the
  model's own chain of thought and stays in the log verbatim.
- Exactly one act block per turn. A second act block is a parse fault.
- The first field inside act is always `tool:`, naming an entry in
  [../tools/registry.md](../tools/registry.md). Remaining fields are that
  tool's parameters. `case:` is envelope metadata and is not a tool parameter.
- Scalar values use `name: value` on one line.
- Payload fields such as `content:`, `files:`, and `patch:` keep every
  following line byte-exact until the act closes.

## JSON Envelope

```json
{
  "schema": "lkj-action",
  "action": {
    "tool": "fs.batch_write",
    "params": {
      "files": [
        { "path": "docs/a.md", "content": "# A\n\n## Purpose\n\nA." }
      ]
    }
  }
}
```

The JSON envelope rejects unknown top-level fields, unknown action fields,
unknown params, missing required params, and null param values. `schema` is
optional; when present it must be `lkj-action`. No release-number schema field
is valid.

## Multi-Line Values

```
<act>
tool: fs.write
path: notes/findings.md
content:
# Findings

The renderer drops trailing newlines on every block write.
</act>
```

The value of content is everything after `content:` through `</act>`,
byte-exact including blank lines, code fences, quotes, shell commands, and
angle-bracket text.

## Batch File Values

```
<act>
tool: fs.batch_write
case: current
files:
-- file --
path: relative/path.md
content:
# Title

Body text.
-- end-file --
-- file --
path: another/path.md
content:
# Title

Body text.
-- end-file --
</act>
```

The parser converts file blocks into the internal batch-write line protocol
before validation. It never executes a partially parsed batch. The full batch
contract, accepted JSON envelope form, limits, and refusal cases live in
[batch-write.md](batch-write.md).

## Control Actions

Task control uses the same shape; contracts in [../tools/control.md](../tools/control.md):

```
<act>
tool: agent.done
summary:
Renamed the flag in both call sites; cargo test passes 41/41.
</act>
```

## Invalid Shapes

These shapes are faults, not partial actions:

- empty assistant content.
- missing `<act>` or missing `</act>` after stop-closure normalization.
- more than one action block.
- JSON text nested inside `<files>` instead of a whole JSON action envelope.
- repeated paired tags for unique parameters, including repeated `<file>` tags.
- parameters absent from the active tool schema.

## Design Properties

- No attributes, no escaping, no nested actions: the grammar fits in a small
  prompt card.
- Every token the model emits is either thought or exactly one decision; there
  is no place to hide a second side effect.
- The format degrades loudly: any deviation maps to one recovery case in
  [recovery.md](recovery.md), never a partial execution.

## Status

implemented.
