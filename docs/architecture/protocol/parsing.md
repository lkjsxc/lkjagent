# Parsing

## Purpose

The strict rules turning a completion into one action. The parser is a pure
function in lkjagent-protocol:

```
parse : CompletionText -> Result<Action, ParseFault>
```

ParseFault variants map one-to-one onto the taxonomy in [recovery.md](recovery.md).

## Rules

1. Scan line by line. Tags are recognized only when a line, after trailing
   whitespace removal, is exactly an opening or closing tag. Indented or
   inline tag-like text is content, not structure.
2. Everything before `<act>` is preamble. A `<think>` pair is allowed there;
   unmatched or repeated think tags are preamble text, not errors.
3. `<act>` must appear exactly once. Absent: ParseFault::MissingAct. A
   duplicate is ParseFault::MultipleAct.
4. The first element inside act must be a tool pair naming a known registry
   entry. Unknown name: ParseFault::UnknownTool. Missing: ParseFault::MissingTool.
5. Each subsequent opening tag inside act collects raw lines until the first
   line exactly matching its closing tag. If bytes follow the opening tag on
   the same line, those bytes are the first value line. Unclosed at end of input:
   ParseFault::UnclosedTag with the tag name.
6. Single-line form `<path>value</path>` is recognized when open and close
   sit on one line; the value is the inner text, untrimmed.
7. Parameter names must be unique within an act. Duplicate:
   ParseFault::DuplicateParam.
8. Parameter sets are validated against the tool's contract from
   [../tools/registry.md](../tools/registry.md): missing required or
   unknown names give ParseFault::BadParams listing every offender at once,
   so one retry can fix all of them.
9. Values are bytes. No unescaping, no entity decoding, no trimming beyond
   the structural newline after an opening tag and before a closing tag.

## Non-Goals

- Not an XML parser: no attributes, namespaces, comments, CDATA, or
  entities. The grammar is a fixed two-level tag language that happens to
  look like XML.
- No repair heuristics: the parser never guesses intent. Recovery is the
  loop's job, with the model in the loop, per [recovery.md](recovery.md).

## Testing

The parser ships with a table of recorded completions: clean turns, every
fault variant, pathological content (tag-like lines inside values, giant
single lines, abrupt cutoffs). Tests assert exact Action values and exact
fault variants. The table grows whenever live operation produces a new
shape; transcripts make every failure reproducible.

## Status

implemented.
