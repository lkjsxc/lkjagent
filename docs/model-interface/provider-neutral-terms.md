# Provider Neutral Terms

## Purpose

This file owns provider-neutral wording for durable project material. The docs,
logs, memory summaries, prompt frames, and generated summaries describe the
configured model endpoint without unnecessary provider names.

## Allowed Default Terms

- model endpoint.
- configured model.
- assistant model.
- reasoning model.
- local model family.
- model profile.
- model adapter.
- inference backend.

## Named Model Exceptions

A named model may appear only when it is part of raw owner input, a raw fixture,
a page explicitly about that adapter, or a cited factual statement. Display
logs store a sanitized form and preserve a raw evidence pointer separately.

## Sanitizer Output

The sanitizer returns sanitized text plus a replacement report naming each
matched pattern, replacement, and count. Completion remains blocked while the
model-specific naming guard is active.

## Status

implemented
