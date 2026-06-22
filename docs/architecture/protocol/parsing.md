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
4. If the first nonempty act line starts with `tool:`, the parser uses the
   line grammar. Otherwise it uses the paired-tag grammar.
5. In line grammar, `tool:` must name a known registry entry. Scalar fields use
   `name: value`; `content:`, `files:`, and `patch:` collect raw lines until
   `</act>`. The `case:` field is envelope metadata and is ignored before
   tool-parameter validation.
6. `files:` accepts canonical `-- file --`, `path:`, `content:`, and
   `-- end-file --` blocks and normalizes them into the internal batch-write
   line protocol.
7. In paired-tag grammar, each opening tag inside act collects raw lines until
   the first line exactly matching its closing tag. Single-line form
   `<path>value</path>` is recognized when open and close sit on one line.
8. Parameter names must be unique within an act. Duplicate:
   ParseFault::DuplicateParam.
9. Parameter sets are validated against the tool's contract from
   [../tools/registry.md](../tools/registry.md): missing required or unknown
   names give ParseFault::BadParams listing every offender at once, so one
   retry can fix all of them.
10. Values are bytes. No unescaping, no entity decoding, and no execution from
    partially parsed actions.

## Non-Goals

- Not an XML parser: no attributes, namespaces, comments, CDATA, or entities.
  The paired-tag grammar is a fixed two-level tag language.
- No semantic repair heuristics: the parser normalizes only the documented
  batch file delimiter form. Recovery is the loop's job, with the model in the
  loop, per [recovery.md](recovery.md).

## Testing

The parser ships with a table of recorded completions: clean turns, every
fault variant, line-grammar scalar actions, batch file blocks, pathological
content (tag-like lines inside values, giant single lines, abrupt cutoffs).
Tests assert exact Action values and exact
fault variants. The table grows whenever live operation produces a new
shape; transcripts make every failure reproducible.

## Status

implemented.
