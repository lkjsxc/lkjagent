# Effect Journal

## Purpose

Define admission, execution, settlement, and crash recovery for every effect.

## Admission

Every external or workspace effect has exactly one accepted model or harness
admission. This includes harness-run shell command checks, which prepare before
`/bin/sh` starts. Admissions are unique by decision and action ordinal. A
rejected admission records a typed reason and creates no effect row. Generated
bundle scopes are checked for overlap first, and every admission, journal, and
target set for one command turn prepares in one SQLite transaction.

## Journal States

An effect has a unique non-null admission and idempotency key. The binding
`decision_id`, command ordinal, admission ID, and journal ID is immutable. Its
durable state is `prepared`, `applying`, `committed`, `recovered`,
`compensated`, or `failed`. Intended, prior, and outcome fingerprints make
reconciliation deterministic. Generated artifact writes also own ordered
`effect_target_revisions` containing exact optional prior and intended bytes.
An absent intended BLOB means an artifact-row-owned stale part must be absent.
A membership target closes the managed `part-NNN.md` set. Unowned names and
managed-unit fingerprint drift reject preparation. Unit concatenation preserves
the authored bytes exactly, and append reconstructs only from verified owned
units. Artifact row intents are immutable data on the target revisions.

Exactly one immutable observation settles every attempted effect. The
observation records attempt outcome, bounded content reference, fingerprint,
and contamination class.

## Two Transactions

The decision is durable before execution. One command-turn transaction then
commits every accepted admission, operation lease, prepared journal, and target
revision before dispatch. A rejected admission commits with its failure state,
snapshot, and decision status instead. Check artifact references and file facts
are gathered before shell admission or execution. The effect edge then stages or performs
the external change. All prior targets are checked before mutation;
descriptor-relative no-follow opens and conditional same-directory renames
apply each file revision. Replacement conflicts preserve captured and current
bytes under internal quarantine names rather than deleting either. An in-process
partial failure restores already changed
targets from exact prior bytes when they still match intended bytes. For a
generated artifact, one post-dispatch SQLite transaction commits artifact rows,
durable refs, observations, context items, command checks, state and plan rows,
recovery facts, snapshot, and decision status. Shell outcomes remain in memory
after execution and join this transaction; a crash leaves their journals
`applying` for no-replay recovery. Provider exchanges, external bytes, and a
future outbox necessarily remain outside this SQLite settlement boundary.

## Recovery

Startup reads prepared or applying effects and compares actual external state
with intended and prior fingerprints. Prepared effects fail without replay.
Applying artifact bundles recover only when every main, part, membership, and
intended-absence target matches. A complete prior bundle fails without replay.
Partial, unavailable, unexpected, or conflicting target sets remain `applying`
and block startup for explicit recovery; they create no artifact rows. A complete
intended bundle reconstructs all artifact rows and refs atomically with its
recovery observation. Applying non-file effects, including shell checks, fail
rather than execute again. A pending decision with any provider exchange,
admission, or observation remains pending and blocks startup; the daemon does
not select a replacement decision or call the provider again.
