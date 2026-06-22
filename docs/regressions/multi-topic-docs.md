# Multi-Topic Docs

## Purpose

This fixture owns the owner-reported failure where a request for lkjagent,
model endpoint, Asia foods, Minecraft, and Factorio collapsed into a generic
lkjagent-only scaffold.

## Input

```text
Create docs. about lkjagent, model endpoint, asia foods, minecraft, factorio
```

## Forbidden Behavior

- Dropping Asia foods, Minecraft, Factorio, or the model endpoint.
- Creating only architecture, guides, operations, overview, and reference.
- Recording disconnected topic blurbs only in the root README.
- Completing after topology passes while relation coverage is missing.
- Emitting repeated scaffold text or mock content.

## Expected Runtime Guard

The documentation contract preserves all requested topics. lkjagent is the
central project, the model endpoint is the provider-neutral model-interface
topic, and the external topics are domain examples that test objective and
artifact contracts.

## Required Seed

The first accepted seed contains project, model-interface, domain-examples,
and relations directories. The relation page connects lkjagent, the model
endpoint, Asia foods, Minecraft, and Factorio before expansion.

## Completion Evidence

Completion requires topology, semantic seed, relation graph, mock-content, and
objective-match audit evidence. The benchmark fixture is
`owner-docs-multi-topic-001`.

## Status

implemented
