# Semantic Seed

## Purpose

The semantic seed owns the first connected documentation tree for a
multi-topic documentation task. It proves the topics relate before expansion.

## Required Seed

For an initial `lkjagent`, named model topic, and Rust request, the seed is:

```text
docs/README.md
docs/project/README.md
docs/project/purpose.md
docs/model-interface/README.md
docs/model-interface/contract.md
docs/implementation/README.md
docs/implementation/rust.md
docs/relations/README.md
docs/relations/project-model-implementation.md
```

## Content Requirements

- Root README links the immediate child directories and states read order.
- Each topic directory states its local contract.
- The relation page connects project runtime, model interface, and Rust.
- Named model facts are absent unless sourced or marked as raw owner text.
- No directory name ends with `.md`.

## Status

implemented
