# Metrics

## Goal Quality

- obligation pass rate;
- false close count;
- unexpected block count;
- artifact and record correctness;
- retrieval recall and unsupported-claim rate;
- final response path and evidence accuracy.

## Loop Quality

- useful decisions per matter;
- progress-producing decision rate;
- repeated decision-failure tuples;
- recovery strategy changes;
- recovery success and time;
- active, waiting, and quiescent seconds;
- crash-resume duplication.

## Prompt Quality

- selected and rendered tokens by lane;
- semantic duplication ratio;
- unresolved conflicts rendered;
- stable-prefix and cached tokens;
- context-source freshness;
- prompt change after failure.

## Tool Quality

- first-pass parse and admission;
- wrong or hidden tool attempts;
- calls per useful effect;
- repeat guard events;
- shell share versus native tools.

## TUI Quality

- duplicate logical IDs;
- causal inversions;
- bottom-anchor violations;
- out-of-range scroll states;
- p50 and p95 key-to-render latency.

## Cost

Track endpoint calls, input, cached input, output, elapsed endpoint time, and
failed-call waste without optimizing away correctness.
