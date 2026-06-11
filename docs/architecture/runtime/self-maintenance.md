# Self-Maintenance

## Purpose

Specify what the loop does when the queue is empty and no task is open: it
improves the system it runs on. This is the continuous-operation payoff and
the reason idle time is never wasted.

## Trigger

At a turn boundary with no open task and no pending queue rows, the harness
injects a maintenance notice opening a maintenance cycle. A cycle has a turn
budget (initial contract: 8 turns), after which the harness re-checks the
queue before opening another cycle. Owner messages always preempt the next
cycle, never the current turn.

## Maintenance Directives

The maintenance notice carries one directive, chosen by the harness in
rotation, weighted by staleness:

| Directive | Work |
| --- | --- |
| distill | Read recent transcript spans; write durable lessons via memory.save per [../memory/distillation.md](../memory/distillation.md) |
| refine-skills | Pick the skill with the oldest refinement stamp or the worst recent outcome; sharpen it via skill.save per [../skills/lifecycle.md](../skills/lifecycle.md) |
| prune-memory | Merge duplicate memory rows, rewrite vague entries, drop superseded ones |
| audit-self | Compare recent failures against contracts; record mismatches as memory entries tagged for the owner |

The harness tracks per-directive last-run stamps in the state table; the
model never schedules itself.

## Bounds

- Maintenance uses the same toolset, protocol, and budgets as task work;
  there is no privileged mode.
- Maintenance may write memory rows and skill files; it must not touch the
  workspace or the queue. The restriction is enforced by the harness
  refusing fs and shell actions targeting the workspace during maintenance,
  with a tool_error observation.
- A maintenance cycle that finds nothing useful ends early with agent.done
  and a one-line summary; an empty cycle is a valid, honest outcome.

## Why Not Heartbeats or Cron

Scheduled wakeups are out of scope per [../../vision/scope.md](../../vision/scope.md).
Maintenance is purely queue-shadowed: it consumes exactly the time the owner
is not using, and stops the moment work arrives.

## Status

design-only.
