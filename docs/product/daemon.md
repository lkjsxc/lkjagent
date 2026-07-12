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

At cycle start, pending owner turns become durable events. Ordinary prose always
opens or updates a matter. Questions for missing facts create a visible waiting
state. Intake never fabricates a closed task or a canned record body.

## Scheduling

One cycle performs at most one provider call or one external effect. Stable
priority and causal sequence select among runnable matters. A blocked matter does
not starve another matter. New owner input is delivered between boundaries, not
inside a provider or effect transaction.

## Quiescence

The daemon sleeps only when no operation or wake is eligible. It wakes on owner
input, due time, file change, or effective config change. The process stays alive
when a matter waits or blocks and continues servicing unrelated work.

## Restart

Startup claims the lease, validates effective config, reconciles applying file
effects, detects ambiguous provider sends, reduces due wakes, and then selects
work. It never blindly repeats an external action.

An active store with retired task/step schema is rejected without mutation. The
owner may retain it as historical evidence and choose a fresh data root.

## Workspace Creation

Startup creates the configured root only when needed. It does not generate the
full life/project/artifact tree. A content workflow declares each directory or
README target required by real output.
