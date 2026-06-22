# Prompting

## Purpose

This directory owns the prompt-frame contract. lkjagent compiles prompts from
case state instead of passing vague owner text directly to the model endpoint.

## Table of Contents

- [prompt-frame.md](prompt-frame.md): fields that every model turn receives.
- [runtime-source.md](runtime-source.md): runtime decision and context-frame source rules.
- [state-selected-prompts.md](state-selected-prompts.md): prompt mode selection by hard state.
- [documentation-prompts.md](documentation-prompts.md): documentation task prompt modes.
- [generic-model-language.md](generic-model-language.md): provider-neutral wording rules.

## Local Map

- Prompt frames consume [../state/state-vector.md](../state/state-vector.md).
- Documentation prompt modes feed
  [../documentation-system/semantic-seed.md](../documentation-system/semantic-seed.md).
- Provider-neutral wording feeds
  [../model-interface/provider-neutral-terms.md](../model-interface/provider-neutral-terms.md).

## Status

design-only
