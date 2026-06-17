# Action Format

## Purpose

The grammar of a model turn. The model writes plain text with paired,
attribute-free tags; the harness stops generation at the closing act tag.
JSON never appears in either direction.

## A Turn

```
<think>
The test expects a trailing newline; the renderer drops it. I will read the
renderer before changing anything.
</think>
<act>
<tool>fs.read</tool>
<path>crates/lkjagent-protocol/src/render.rs</path>
<start>1</start>
<count>60</count>
</act>
```

- The think preamble is optional, free-form, and unparsed; it exists for the
  model's own chain of thought and stays in the log verbatim.
- Exactly one act block per turn. The stop sequence `</act>` makes a second
  block impossible to generate.
- The first child of act is always tool, naming an entry in
  [../tools/registry.md](../tools/registry.md). The remaining children are
  that tool's parameters, in any order.
- All tags sit alone on their own lines. Parameter values are the raw lines
  between a tag pair; single-line values may sit inline between the tags.

## Multi-Line Values

```
<act>
<tool>fs.write</tool>
<path>notes/findings.md</path>
<content>
# Findings

The renderer drops trailing newlines on every block write.
</content>
</act>
```

The value of content is everything between its tags, byte-exact including
inner blank lines. One reserved case exists: a payload that itself contains
a line consisting exactly of a closing parameter tag cannot be expressed;
the model routes such payloads through shell.run with a heredoc per
[../tools/shell.md](../tools/shell.md). The limitation is stated in the
system prompt rather than papered over with escaping rules small models
would misapply.

## Control Actions

Task control uses the same shape; contracts in [../tools/control.md](../tools/control.md):

```
<act>
<tool>agent.done</tool>
<summary>
Renamed the flag in both call sites; cargo test passes 41/41.
</summary>
</act>
```

## Design Properties

- No attributes, no escaping, no nesting below parameters: the grammar fits
  in a dozen lines of the system prompt.
- Every token the model emits is either thought or exactly one decision;
  there is no place to hide a second side effect.
- The format degrades loudly: any deviation maps to one recovery case in
  [recovery.md](recovery.md), never a partial execution.

## Status

implemented.
