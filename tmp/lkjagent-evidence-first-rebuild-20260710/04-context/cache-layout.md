# Cache Layout

## Stable Prefix

Keep deterministic, slowly changing content first:

- kernel;
- owner policy;
- state grammar;
- tool schemas sorted by stable name;
- output envelope grammar.

## Volatile Suffix

Place objective details, selected excerpts, current observations, conflict
details, and retry diagnosis after the stable prefix.

## Fingerprints

Record fingerprints for kernel, state, each lane, tool view, output grammar,
full prompt, and stable prefix. Compute changed-token ratio between consecutive
calls.

## Reuse

Do not rewrite stable cards with timestamps, counters, or random ordering.
Reference durable IDs in volatile cards only when the decision requires them.

## Evaluation

Measure cache hit tokens, uncached input, total input, output, latency, and task
success. Cache improvement never justifies stale or irrelevant context.
