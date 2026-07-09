# Endpoint

## Purpose

Define one bounded OpenAI-compatible chat-completions effect.

## Configuration

Flat configuration and named environment variables provide URL, model, secret
name, finite timeout, context limit, and optional capability settings. Effective
non-secret configuration and capability fingerprints bind every exchange.

## Request

The persisted decision supplies the compiled prompt, sampling values, maximum
output tokens, expected closing stop sequence, grammar constraint when supported,
and deadline. An endpoint call counts against its matter budget once sent.

## Result

The adapter returns response bytes or a typed transport, timeout, rate-limit,
empty, reasoning-only, output-limit, or provider outcome. It also returns
request and response monotonic times and provider usage fields when reported.
Missing usage stays unknown.

Request and response bodies enter restricted content-addressed storage.
`provider_exchanges` records decision, attempt ordinal, request fingerprint,
redacted refs, timing, usage, model identity, and outcome.

## Repair Boundary

A pure append of the expected closing tag may be allowed only when the parsed
body is otherwise complete and the decision enabled that deterministic repair.
All other malformed or truncated output becomes a runtime fault and cannot be
partially executed.

## Retry

Backoff, deadlines, and alternative caps are recovery strategies selected from
failure lineage. The endpoint adapter itself does not loop until success or hide
an attempted exchange.
