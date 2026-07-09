# Constrained Output

## Capability Detection

Probe the configured endpoint once for supported stop sequences and grammar
constraints. Store the capability result outside model context. Never assume
support from endpoint name.

## Grammar

When supported, compile the current decision envelope and tool fields to a
grammar constraint. The grammar contains only current tools and field shapes.
It remains attribute-free and does not force JSON.

## Fallback

When grammar constraints are unavailable:

- keep the view small;
- place exact grammar at the end of the stable prefix;
- use one concrete decision-bound example;
- set the matching closing stop sequence;
- parse strictly and enter typed repair on failure.

## Measurement

Compare first-pass parse, admission, recovery calls, latency, output tokens, and
semantic correctness. A grammar that parses but harms tool choice is not
adopted.
