# Improve Structure No-Op

## Purpose

This fixture owns the failure where `improve structure` closed after inspection
and topology audit without changing structure or recording a truthful blocker.

## Input Sequence

```text
Please init docs, include about lkjagent, qwen, rust
improve structure
```

## Expected Behavior

- Classify the second input as an amendment or structural-maintenance case.
- Inspect the workspace with filesystem evidence.
- Produce a structural health report.
- Make at least one real structural improvement or record a truthful blocker.
- Treat topology audit as necessary and insufficient.

## Forbidden Behavior

- Completion after `memory.find` alone.
- Completion after `queue.list` alone.
- Completion after inspection only.
- Recording workspace facts from memory search alone.

## Status

implemented
