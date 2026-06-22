# Functional Core

## Purpose

The functional core owns deterministic decisions. It accepts typed inputs and
returns state, authorization, audit, prompt, recovery, or maintenance decisions
without performing effects.

## Contract

- Filesystem, shell, queue, memory, Docker, git, and model calls stay at the
  edge.
- Reducers transform events into hard state and weighted tracks.
- Authorization returns a decision with blockers and preferred tools.
- Audits return structured results that own their evidence kind.

## Links

- Runtime authority: [../architecture/runtime/authority/README.md](../architecture/runtime/authority/README.md).
- Prompt frame: [../prompting/prompt-frame.md](../prompting/prompt-frame.md).
- Semantic audits: [../verification/semantic-audits.md](../verification/semantic-audits.md).

## Status

implemented
