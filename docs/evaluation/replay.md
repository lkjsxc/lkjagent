# Replay

## Purpose

Define deterministic replay over recorded exchanges.

## Replay Contract

Replay drives the core engine seam with recorded model outputs, endpoint
anomalies, file effects, and check results. It does not contact a live endpoint.

## Gate

`cargo run -p lkjagent-xtask -- smoke replay` runs replay cases. A passing run
prints `ok smoke-replay` plus a bounded artifact path when configured.

## Historical Fixtures

The stage baseline preserved `tmp/fixtures/live-proof-20260701T100958Z/`,
`tmp/fixtures/live-proof-20260701T113018Z/`, and `tmp/fixtures/data/logs/` as
raw seed material. Task fixtures distilled into the corpus are committed with
the replay test that uses them.

## Failure Rule

A live failure becomes a replay fixture before it is fixed. The fixture is
scrubbed, minimized, named by failure class, and landed with the test.
