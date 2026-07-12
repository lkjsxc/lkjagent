# Daemon

## Purpose

Define owner intake, roots, scheduling, quiescence, and restart behavior.

## Roots

Runtime data and visible workspace are separate capabilities. Direct execution
resolves the default workspace as `../workspace` from the data root. Compose
mounts host data at `/data` and host workspace at `/workspace`.

The resolved workspace path and fingerprint are visible in status and decisions.
All file opens are relative to a retained no-follow root capability.

## Intake

Public send writes directly to the native store. Ordinary prose currently opens
a new matter with one owner turn, owner message, intake event, and active
`matter/opened` cell. Intake never creates workspace files, a retired task, or a
canned record body.

## Scheduling

One public cycle selects and persists at most one direct decision, then performs
at most one provider call and the single action admitted from that decision.
Selection is from native cells and causal sequence. The first cutover selects one
open matter at a time; fairness across simultaneous open matters is not yet
proved. New owner input is delivered only at command boundaries.

## Quiescence

The daemon sleeps only when no operation or wake is eligible. It wakes on owner
input, due time, file change, or effective config change. The process stays alive
when a matter waits or blocks and continues servicing unrelated work.

## Restart

Public restart projects native cells and unfinished work before selection. A
sent provider exchange is marked ambiguous and is never replayed. An unfinished
file effect blocks rather than overwriting unknown bytes; settled effects and
checks-ready state are recognized by the native projection. Full phase recovery
for every interrupted file boundary remains narrower than the effect primitive's
isolated recovery suite.

A store with retired task/step schema is rejected without mutation. The owner may
retain it as historical evidence and choose a fresh data root.

## Workspace Creation

Startup creates the configured root only when needed. It does not generate the
full life/project/artifact tree. A content workflow declares each directory or
README target required by real output.
