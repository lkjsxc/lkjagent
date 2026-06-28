# Compact Context

## Purpose

Define the model-facing prompt frame for a weak local model. The runtime owns
the next step; the model sees one compact authority card and one exact valid
action, not a broad tool manual.

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

## Exact Next Action

The card is followed by exactly one valid action example:

```text
<next-action>
<action>
<tool>artifact.next</tool>
<root>stories/novel</root>
</action>
</next-action>
```

The example is rendered from the registry and current authority, then
round-tripped through parse, schema validation, and admission in tests.

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
- show an action that parses, validates, and is admitted by the decision.

## Status

open for this redesign.
