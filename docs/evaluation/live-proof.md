# Live Proof

## Purpose

Define the daily-use live endpoint proof and capture rules.

## Acceptance Criteria

A live proof is acceptable only when the proof bundle shows:

- the requested profile and success checks;
- prompt frames, context lane fingerprints, and tool-view fingerprints;
- provider exchanges with request and response refs;
- admitted or rejected tool calls tied to decisions;
- observations, artifacts, checks, and final state rows;
- workspace records or aliases touched by the profile; and
- adoption notes that say what changed because of the evidence.

The preferred profiles are personal daily capture, finance receipt flow,
calendar meeting flow, software maintenance, workspace rebalance, protocol
stress, TUI operator flow, and recovery flow.

## Duration

Standard live profile duration is 900 seconds when endpoint credentials are
available. Short dry runs are allowed only to prove setup before spending live
budget. Missing endpoint configuration writes explicit skip evidence instead of
a pass.

## Capture

Each run writes a stamped directory under `tmp/live-runs/` with data, prompt
frames, request and response refs, observations, final state, metrics, and an
adoption note. Secret-bearing raw bodies stay local or are redacted before any
committed summary.

## Procedure

The operator starts the daemon, sends the profile objective, waits until the time
box or harness-computed completion, collects proof, and records the exact gates
or skip reason. The model never decides that the live proof passed.

## Failure This Prevents

Live evidence measures real daily-use behavior instead of optimizing for a
single prose-generation benchmark.
