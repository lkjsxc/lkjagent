# No-Op Suppression

## Purpose

No-op suppression owns truthful idle maintenance closure. A maintenance cycle
that changes nothing records a suppression key instead of looping through the
same memory and queue checks.

## Suppression Record

```text
suppression_key
state snapshot
observed tools
new evidence requirement
expires after cycles or new evidence
reason
```

## Trigger

Create or refresh a suppression record when a cycle finds only previously
recorded lessons, lists the same queue state, prunes nothing, changes nothing,
and records no new structural finding.

## Guard

Future cycles with the same key cannot repeat the same action signatures unless
new owner, queue, log, or workspace evidence appears.

## Status

design-only
