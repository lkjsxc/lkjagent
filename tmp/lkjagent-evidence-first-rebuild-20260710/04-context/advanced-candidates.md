# Advanced Context Candidates

## Routing And Coverage

- derive one information-need record per unsatisfied obligation, expected check,
  operation input, failure, and owner correction;
- require source coverage for each need before admitting optional history;
- use negative routing constraints such as wrong project, wrong date, stale
  revision, or already-rendered claim;
- select a causal event slice from the current matter rather than a fixed recent
  message window;
- expand exact document relations one hop only when they satisfy a named need;
- run query decomposition for multi-intent owner turns, then merge candidates by
  obligation rather than concatenating independent result lists.

## Retrieval Fusion

- exact path and stable document ID lookup;
- field-filtered kind, project, state, and effective-date lookup;
- lexical, trigram, heading, relation, and hierarchical README retrieval;
- reciprocal-rank fusion across independent indexes;
- novelty-aware maximal marginal relevance after trust filtering;
- optional local embedding retrieval only as a measured candidate, never sole
  authority and never before fingerprint validation;
- counterfactual retrieval tests that remove each source and measure which
  obligation or answer changes.

## Claim Graph

- parse selected prose into subject, predicate, value, effective interval, and
  provenance without replacing original bytes;
- collapse entailment-equivalent claims across objective, transcript, summary,
  observation, and workspace regions;
- preserve distinct evidence for one claim as compact source refs, not repeated
  prose;
- distinguish correction, supersession, history, scope difference, and material
  contradiction;
- propagate source invalidation through summary, check, decision, and response
  edges before the next prompt.

## Budget Optimization

- mandatory-first constrained knapsack with per-need minimum coverage;
- lane caps derived from state, uncertainty, and expected tool output;
- source-density scoring using supported claims per token;
- diversity constraints over source type, time range, and project;
- conservative Japanese-aware token measurement calibrated with provider usage;
- reserve output and parser margins before selecting any optional item;
- place the current operation and highest-value evidence near prompt boundaries
  and test middle-position degradation rather than assuming an order.

## Compression And Delta Context

- immutable matter capsules with source-level entailment checks;
- deterministic table or path/value projection before model summarization;
- hierarchical summaries that own no facts and invalidate on child fingerprint;
- observation deltas instead of repeating complete tool output;
- failure-lineage cards that name tried strategy tuples once;
- semantic artifact manifests while full sections remain outside normal context;
- bounded continuation handles that retrieve only the next unit and dependencies.

## Prompt Stability And Feedback

- content-addressed prompt fragments and a stable provider-cache prefix;
- canonical sorting independent of insertion order or database row order;
- prompt-region semantic hashes that reject cross-region duplication;
- changed-token and source-set diffs after every decision;
- require a material context, tool, budget, or strategy change after failure;
- link every context frame to parse, admission, effect, check, progress, cost,
  latency, and later invalidation outcomes;
- learn ranking calibration only from recorded outcomes while keeping hard trust,
  safety, conflict, and source-currentness filters non-learned.

## Evaluation Rule

Do not implement all candidates permanently. Build them as real experiment
factors, test isolated and integrated combinations across the anchored scenarios,
and apply `../09-evaluation/adoption-thresholds.md`. Retain designs, interactions,
raw results, and rejection reasons in docs even after rejected code is removed.
