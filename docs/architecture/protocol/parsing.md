# Parsing

## Purpose

The strict rules turning completion text into one action or one structured
parse fault. The parser is pure in lkjagent-protocol:

```text
parse_live_completion : CompletionText + ParseSettings -> ParseOutcome
```

`ParseOutcome` carries the parsed action when present, one fault when present,
the envelope mode, and a normalized text hash. Parse faults map directly to the
routes in [recovery.md](recovery.md). Provider stop-closure normalization is a
wire step documented in [stop-token-policy.md](stop-token-policy.md).

## Envelope Rules

1. `<action>` must appear exactly once for the natural live path.
2. `</action>` must appear exactly once unless provider-stop closure restored
   it before parsing.
3. `<actions>` is invalid.
4. `<act>` is invalid for live dispatch.
5. Plain prose without a complete action body is `MissingActionEnvelope`.
6. A duplicate action envelope is `MultipleActionEnvelopes`.
7. A natural open envelope without a close is `UnclosedActionEnvelope` unless
   the provider-stop closure rule applies.

## Body Rules

1. The first recognized field is the tool.
2. Canonical model output uses `<tool>known.tool</tool>`.
3. The bounded line body `tool: known.tool` exists only for strict implicit
   envelope normalization and parser fixtures.
4. The tool must name one registry entry.
5. Each parameter name must be unique.
6. Parameter sets are validated against [../tools/registry.md](../tools/registry.md).
7. Missing required and unknown names produce `BadParams` listing every offender
   against the parameters parsed so far.
8. Conditional requirements such as `checks|paths` produce the same registry
   example renderer as ordinary missing required parameters.
9. Values are bytes. The parser does no unescaping, entity decoding, or
   execution from partially parsed actions.
10. Only assistant content is parsed. Provider reasoning fields are evidence
    and never become action text.

## Tag Line Classification

The parser classifies every structural-looking line before parameter
validation:

- exact opening action envelope.
- exact closing action envelope.
- exact paired opening tag.
- exact paired inline tag.
- exact closing tag.
- attribute-like tag.
- malformed angle-bracket text.
- ordinary payload line.

A line such as `<path=stories/chronos-fracture</path>` is an
`AttributeLikeTag` fault. It is not an unknown parameter. The fault records the
invalid tag name, a value hint when one is recoverable, and the parameters that
were recognized before the malformed line. It must not report already parsed
required fields as missing.

## Implicit Envelope Normalization

A missing opening envelope can normalize only when all conditions hold:

- the response has no natural `<action>` envelope;
- the response contains exactly one complete action body starting with
  `<tool>known.tool</tool>` or `tool: known.tool`;
- no prose exists outside recognized fields;
- registry validation passes;
- runtime admission for the persisted decision passes before dispatch;
- parse and provider exchange logs record `ImplicitActionEnvelope`.

These bodies can normalize:

```text
<tool>graph.state</tool>
```

```text
tool: graph.state
```

These bodies remain faults:

```text
I will inspect the graph state next.
```

```text
<tool>graph.state</tool>
<tool>fs.list</tool>
```

```text
<tool>graph.plan</tool>
<path=stories/chronos-fracture</path>
```

## JSON

Top-level JSON action output is `JsonActionRejected` in the live parser. JSON
inside a parameter remains tool-specific payload text and is validated by that
tool contract.

## Non-Goals

- Not an XML parser: no attributes, namespaces, comments, CDATA, or entities.
- No semantic repair heuristics in the parser. Recovery chooses the next route
  from structured faults and persisted runtime authority.

## Testing

The parser table covers clean turns, every fault variant, provider-stop closure,
implicit envelope normalization, attribute-like tags, duplicate parameters,
conditional requirements, giant lines, and abrupt cutoffs. Tests assert exact
parse outcomes and grow when live operation produces a new shape.

## Status

partially implemented. Existing parser behavior still accepts non-action
envelope constants and top-level JSON in live parsing; the implementation task
is to align parser outcomes with this contract.
