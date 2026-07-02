# Replay

## Purpose

Define deterministic replay over recorded exchanges.

## Replay Contract

Replay drives the plan-engine store, effects, workspace, and checks with
committed deterministic fixtures. It does not contact a live endpoint.

## Gate

`cargo run -p lkjagent-xtask -- smoke replay` runs the replay cases. A passing
run prints `ok smoke replay data=<path>`. The gate is part of `quiet verify`.
Docker runs the same command through `docker compose run --rm replay`.

## Cases

The replay currently covers:

- `manuscript-small`: three Aurora Ledger chapter files totaling at least 1,500
  words with engine-computed checks;
- `question`: a closed direct-answer task in the plan store.

The replay output data directory is suitable input for
[../operations/proof-bundles.md](../operations/proof-bundles.md).

## Live Smoke

`cargo run -p lkjagent-xtask -- smoke live` reports endpoint configuration
presence and prints an explicit skip line when a live run is not requested.

## Historical Fixtures

The stage baseline preserved `tmp/fixtures/live-proof-20260701T100958Z/`,
`tmp/fixtures/live-proof-20260701T113018Z/`, and `tmp/fixtures/data/logs/` as
raw seed material. Task fixtures distilled into the corpus are committed with
the replay test that uses them.

## Failure Rule

A live failure becomes a replay fixture before it is fixed. The fixture is
scrubbed, minimized, named by failure class, and landed with the test.
