# Docs Init lkjagent Model Rust

## Purpose

This fixture owns the failure where a docs-init request produced disconnected
blurbs and a misleading directory name instead of a connected seed.

## Input

```text
Please init docs, include about lkjagent, qwen, rust
```

## Expected Behavior

- Create a DocumentationContract before writing.
- Make lkjagent the central project and runtime.
- Treat the named model term as a model-interface or adapter topic without
  unsupported claims.
- Treat Rust as the implementation substrate.
- Create project, model-interface, implementation, and relations pages.
- Run topology, semantic seed, relation, mock-content, and model-name audits.

## Forbidden Behavior

- root README with independent unlinked blurbs.
- directory named `docs/lkjagent.md/`.
- generic first-pass scaffold.
- unsupported named-model capability claims.
- completion after topology audit alone.

## Status

implemented
