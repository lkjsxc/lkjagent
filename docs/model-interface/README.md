# Model Interface

## Purpose

This directory owns the provider-neutral model boundary. lkjagent treats the
configured endpoint as an unreliable proposer and keeps state, tools, audits,
and completion authority in the runtime.

## Table of Contents

- [contract.md](contract.md): runtime-owned contract around model proposals.
- [provider-anomalies.md](provider-anomalies.md): provider output shapes that are not model actions.
- [provider-neutral-terms.md](provider-neutral-terms.md): allowed durable wording.

## Local Map

- The prompt frame is [../prompting/prompt-frame.md](../prompting/prompt-frame.md).
- The action grammar is [../architecture/protocol/action-format.md](../architecture/protocol/action-format.md).
- The relation to Rust is
  [../relations/project-model-implementation.md](../relations/project-model-implementation.md).

## Status

implemented
