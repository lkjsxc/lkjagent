# Semantic Seed

## Purpose

The semantic seed owns the first connected documentation tree for a
multi-topic documentation task. It proves the topics relate before expansion.

## lkjagent Model Rust Seed

For an initial lkjagent, model-boundary, and Rust request, the seed preserves
the project, provider-neutral model interface, implementation, and relation
pages before any broad expansion.

Required paths include:

```text
docs/project/lkjagent.md
docs/model-interface/model-endpoint.md
docs/implementation/rust.md
docs/relations/project-model-implementation.md
```

## Domain Example Seed

For `lkjagent`, model endpoint, Asia foods, Minecraft, and Factorio, the seed
preserves every requested topic and keeps the external topics scoped as domain
examples until the owner asks for full source-backed domain documents.

Required paths include:

```text
docs/project/lkjagent.md
docs/model-interface/model-endpoint.md
docs/domain-examples/asia-foods.md
docs/domain-examples/minecraft.md
docs/domain-examples/factorio.md
docs/relations/project-model-domain-examples.md
```

## Content Requirements

- Root README links the immediate child directories and states read order.
- Each topic directory states its local contract.
- Relation pages connect the central project, model interface, and topics.
- Named model facts are absent unless sourced or marked as raw owner text.
- No directory name ends with `.md`.
- Generic architecture, guides, operations, overview, and reference scaffolds
  are forbidden until relation coverage justifies them.

## Status

implemented
