# Fault Injection

## Store And Filesystem

Inject failure:

- after prepared effect row;
- after temporary file write;
- after file flush;
- after atomic rename;
- before database settlement;
- during index update;
- during multi-file group.

Restart and prove exactly one semantic result, correct fingerprint, and visible
recovery evidence.

## Endpoint

Inject timeout, connection reset, rate limit, truncated output, invalid envelope,
stale response, and wrong tool. Advance a fake clock for backoff tests.
Inject a native tool error that the dispatcher reports as an observation. Prove
the attempt becomes a failure lineage and no recovery card says the fault was
successful.

## Context

Inject changed source fingerprint, stale summary, conflicting owner correction,
untrusted instruction text, secret marker, and oversized file.

## TUI

Inject duplicate refresh, delayed final row, restart with draft, tied sequence
request, rapid resize, slow store read, and event burst.

## Assertions

Every injected fault selects a typed recovery or waiting operation, changes the
strategy on recurrence, preserves evidence, and never becomes happy response or
synthetic idle.

Every observation references exactly one existing accepted admission, and every
accepted admission settles to exactly one effect outcome and observation.
