# Waiting And Quiescence

## Purpose

Define visible waiting and daemon-level quiescence without synthetic work.

## Waiting

`waiting-owner` contains one bounded question and the matter relation that the
answer will settle. `waiting-external` contains a due time or observable wake
condition. Both remain durable, visible states and may coexist with other
runnable matters. Future cooldown cells remain active but ineligible; no runtime
decision or model call is created until their persisted UTC instant is due.

## Candidate Priority

The selector settles interrupted effects first, then owner answers and
cancellations, safety repairs, active owner work, verification, completion
reports, queued matters, and due deterministic maintenance. Ties use persisted
priority, causal sequence, and stable operation ID.

## Quiescence

The daemon is quiescent only when there is no eligible operation, due wake,
pending owner turn, interrupted effect, or due maintenance. Quiescence is a
derived daemon condition. It is never a matter, task, decision state, or elapsed
poll counter.

An ineligible operation produces a visible wait or blocked-report candidate
with a wake condition. It never falls through to success or idle.

## Wake Sources

Owner turns, elapsed timers, external observations, recovered effects, source
fingerprint changes, and explicit maintenance schedules can wake the daemon.
Waiting and quiescent periods consume no model calls.
