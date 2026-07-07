# Token Accounting

## Purpose

Define provider token usage fields and display semantics.

## Fields

- `input_total_tokens`: provider prompt or input tokens as reported.
- `input_cached_tokens`: subset of input served from cache when reported.
- `input_uncached_tokens`: total minus cached when both are known.
- `output_tokens`: provider completion or output tokens.
- `cache_status`: `known`, `unknown`, `not_supported`, or `provider_specific`.
- `raw_usage`: provider metadata stored internally, not model-visible context.

## Unknowns

Missing cached input is unknown, not zero. Missing total input is unknown even
when output tokens are known. Derived uncached input is stored only when total
and cached values are both known.

## Display

Operator displays render all known fields explicitly:

```text
tokens: input_uncached=804 input_cached=1200 input_total=2004 output=196 cache=known
```

Unknown fields render as `unknown` so operators can distinguish provider silence
from real zero values.

## Proof

Proof bundles include token usage row ids, provider exchange refs, cache status,
and raw usage refs. They do not expose secret-bearing raw provider bodies.
