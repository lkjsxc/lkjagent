# Tag-Based Action Protocol

## Purpose

Fix the format in which the model expresses actions.

## Decision

The model speaks a small tag-based protocol: an optional think preamble, then
exactly one action block built from paired, attribute-free tags, terminated
by a closing act tag that doubles as the stop sequence. The grammar is owned
by [../architecture/protocol/action-format.md](../architecture/protocol/action-format.md).

JSON is banned as a model output format. Tags survive small-model decoding
far better: no brace balancing, no string escaping, no trailing-comma traps,
and the stop sequence guarantees one action per turn.

## Consequences

- The parser is strict, line-oriented, and tiny; its rules live in
  [../architecture/protocol/parsing.md](../architecture/protocol/parsing.md).
- Payloads that contain protocol-shaped lines are routed through shell
  heredocs; the limitation is documented, not hidden.
- Every recovery case has a taxonomy entry in
  [../architecture/protocol/recovery.md](../architecture/protocol/recovery.md);
  invalid output costs one bounded notice, never a crash.
- Skill bodies teach by example because the format reads as plain text.

## Rejected Directions

- JSON tool calls: the dominant convention, but small quantized models
  malform JSON under pressure, and a prior project measured exactly that
  failure mode.
- Markdown sections as protocol: pleasant to read, but headers and fences
  appear naturally inside file content, making parses ambiguous.
- Line-oriented key-value records: robust but cramped for multi-line
  payloads, which dominate real tool use.
