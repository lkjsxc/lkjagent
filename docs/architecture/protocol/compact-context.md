# Compact Context

## Purpose

Define the model-facing prompt frame for a weak local model. The runtime owns
the next step; the model sees one compact authority card and one decision-
selected action surface, not a broad tool manual.

## Runtime Card

A model-call prompt frame contains one card:

```text
<runtime-card>
<decision>42</decision>
<mission>artifact-repair</mission>
<mode>recovery</mode>
<case>1</case>
<root>stories/novel</root>
<missing>artifact-readiness</missing>
<must-use>artifact.next</must-use>
<blocked>agent.ask memory.find memory.save shell.run</blocked>
<budget>512 output tokens</budget>
<reason>weak artifact paths remain after audit</reason>
</runtime-card>
```

Fields are deterministic data from the persisted `RuntimeDecision`. The card
must not include hidden reasoning, object-literal tool calls, raw provider
reasoning, invalid action examples, or raw parse-fault assistant text.

## Action Surface

The card is followed by exactly one action surface. For non-content actions it
is a full exact tag action rendered from the registry and current authority:

```text
<next-action>
<action>
<tool>artifact.next</tool>
<root>stories/novel</root>
</action>
</next-action>
```

For content writes it is a write contract, not prefilled body text. The surface
names `fs.batch_write`, root, exact paths, size limits, required sections, and
forbidden weak phrase classes. The model must author the singular
`fs.batch_write` action using line protocol inside `<files>`.

## Escape Surface

The prompt may name a tiny set of escape tools only when the decision admits
them for the mission. The default prompt does not render the full registry. If
an escape tool is blocked by the decision, it must not appear as a usable
example.

## History Hygiene

Prompt history stores compact observations and runtime facts. It never replays
object-literal batch examples, nested `<file>` children, provider reasoning,
invalid assistant output, or broad recovery transcripts as assistant examples.

## Verification

Tests must prove rendered prompt frames:

- start from a stored decision id;
- contain one `runtime-card` and one `next-action`;
- include the output budget;
- contain no object-literal `fs.batch_write` payloads;
- show a non-content action that parses, validates, and is admitted by the
  decision;
- show content writes as contracts with no generated body prose.

## Status

implemented for the current runtime cards and decision-selected action
surfaces. Open work is to reuse the compact surface for content atom and
manuscript assembly contracts without broadening the prompt manual.
