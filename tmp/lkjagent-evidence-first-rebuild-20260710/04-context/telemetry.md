# Context Telemetry

## Per Decision

Record:

- candidates discovered, excluded, and selected;
- token estimate and actual usage by lane;
- semantic duplicate count and rendered duplication ratio;
- stale, contaminated, and conflict exclusions;
- source types and fingerprints;
- retrieval latency and index generation;
- stable-prefix tokens and changed-token ratio;
- selected item utility and exclusion reason.

## Outcome Linkage

Link context frames to parse success, admission success, effect success, checks,
latency, token usage, and final obligation progress. This enables comparison of
context strategies rather than aesthetic judgment.

## Alerts

Flag:

- selected content over budget;
- any semantic duplicate;
- source fingerprint mismatch;
- unresolved conflict in a normal prompt;
- more than the configured share of low-utility context;
- repeated prompt fingerprint after a failure;
- low cache reuse caused by unstable ordering.
