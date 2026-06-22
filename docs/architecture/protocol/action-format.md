# Action Format

## Purpose

The grammar of a model turn. The model writes plain text inside one act
envelope. The preferred form is line-oriented `name: value` fields; the older
paired-tag form is still parsed by the same pure parser. JSON never appears as
an action payload.

## A Turn

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
before validation. It never executes a partially parsed batch.

## Control Actions

Task control uses the same shape; contracts in [../tools/control.md](../tools/control.md):

```
<act>
tool: agent.done
summary:
Renamed the flag in both call sites; cargo test passes 41/41.
</act>
```

## Design Properties

- No attributes, no escaping, no nested actions: the grammar fits in a small
  prompt card.
- Every token the model emits is either thought or exactly one decision; there
  is no place to hide a second side effect.
- The format degrades loudly: any deviation maps to one recovery case in
  [recovery.md](recovery.md), never a partial execution.

## Status

implemented.
