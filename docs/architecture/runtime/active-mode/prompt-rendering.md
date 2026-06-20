# Prompt Rendering

## Purpose

Define the single model-visible active-mode card.

## Card

The prompt contains exactly one authority card:

```text
Active Mode:
mode=OwnerTask
policy_layers=graph
allowed_tools=...
blocked_tools=...
preferred_next_action=...
completion_condition=...
valid_example:
<act>
...
</act>
```

## Rules

Maintenance and graph policy never render together. Compaction never renders
as a model tool requirement. `memory.save` is never required for hard
compaction. `agent.ask` renders only when a concrete owner-required question
exists. The objective text omits raw counters and internal prefixes.

## Valid Examples

Every rendered valid example must parse, validate against the registry, and
be admitted by the same effective policy. If a graph transition is needed
before the productive tool is admitted, the example must be that transition.
