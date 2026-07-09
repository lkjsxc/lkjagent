# Context Experiments

## Candidate Ideas

- state-routed exact-path retrieval;
- lexical plus trigram body search;
- project and date filters;
- recency-decayed relevance;
- novelty-aware cost selection;
- hierarchical README descent;
- matter capsules;
- recent-conversation causal windows;
- contradiction blocking edges;
- source-change invalidation;
- stable-prefix ordering;
- provider-usage calibration.

## Combination Cells

Test at least:

- exact paths plus ranked lanes;
- ranked lanes plus causal conversation;
- ranked lanes plus matter capsule;
- hierarchical descent plus body search;
- all preceding features with contradiction blocking;
- the strongest set with stable-prefix optimization.

## Metrics

Use task success, evidence recall, false fact rate, prompt tokens, duplication,
cache reuse, latency, tool success, recovery success, and owner-visible quality.

## Adoption

Apply the precommitted floors and comparative rule in
`../09-evaluation/adoption-thresholds.md`. Record rejected and conditional ideas
with their exact combination and evidence. A single weak standalone result does
not rule out a useful measured combination.
