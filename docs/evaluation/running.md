# Running Benchmarks

## Purpose

State the benchmark commands for deterministic local checks and real agent
runs.

## Corpus Checks

Corpus checks require no endpoint and no network:

```sh
cargo run -p lkjagent-xtask -- benchmark list
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- benchmark judge --task crt-exact-001 --workspace /tmp/workspace
```

`benchmark check-corpus` validates task definitions, starter paths, fixtures,
judges, and pass/fail separation. On success it prints exactly:

```sh
ok benchmark-corpus
```

## Real Runs

Real benchmark runs require an OpenAI-compatible endpoint visible from the
agent container:

```sh
LKJAGENT_ENDPOINT_URL=http://host.docker.internal:8080 \
LKJAGENT_MODEL=local-model \
cargo run -p lkjagent-xtask -- benchmark run --suite tiny --data data/benchmark
```

The runner creates one compose project and one data directory per task. It
starts the real `agent` service, sends prompts through `lkjagent send`, polls
`lkjagent status` and `lkjagent log --full`, stops compose cleanly, judges the
workspace from the host, and writes reports under the benchmark data
directory. If endpoint config is absent, it exits with an endpoint
configuration missing message and writes no score.

Report comparison uses TSV reports from two runs:

```sh
cargo run -p lkjagent-xtask -- benchmark compare old.tsv new.tsv
```

## Status

implemented.
