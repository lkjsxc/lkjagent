# Improvement Loop

## Purpose

Define how benchmark failures turn into durable harness improvements.

## Loop

1. Run `cargo run -p lkjagent-xtask -- benchmark check-corpus`.
2. Run `cargo run -p lkjagent-xtask -- benchmark run --suite tiny --data data/benchmark`
   with real endpoint config.
3. Inspect failures by family and operational metric, not only by total score.
4. Pick the smallest durable change that addresses a failure cluster.
5. Update the owning docs contract before changing code, prompts, tools,
   context policy, skills, memory, or runtime behavior.
6. Rerun the focused task or family, then the suite when the focused result
   changes.
7. Record what improved, regressed, or stayed inconclusive from the actual
   report paths.

Never claim benchmark improvement without a real run report. `check-corpus`
proves the benchmark is valid; it does not score lkjagent. A low score is
evidence to inspect, not a reason to weaken a judge or teach the agent a
task-specific answer.

## Status

implemented.
