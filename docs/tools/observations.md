# Observations

## Purpose

Define the one immutable outcome that settles an attempted effect.

## Shape

Every observation records ID, effect, decision, operation, admission, status,
attempt outcome, bounded content reference, source fingerprints, creation time,
and contamination class. One effect has exactly one settling observation.

```text
<observation>
<status>ok</status>
<content_ref>sha256:...</content_ref>
<fingerprint>sha256:...</fingerprint>
</observation>
```

Large or secret-bearing output remains in protected content-addressed storage.
The observation stores a bounded redacted view and exact byte fingerprint.

## Prompt Rule

Only current relevant observations enter normal prompts. Old observations remain
source evidence and may be summarized as clean candidates or excluded as stale,
superseded, external raw, or recovery-only. Failed output enters recovery only
through a bounded diagnosis and fingerprint.

## Effect Rule

An observation cannot exist without an accepted admission and attempted effect.
A rejected admission has no effect or observation. Recovery reads the durable
observation before selecting replay, compensation, inspection, or blocking.
