# Self-Maintenance

## Purpose

State the self-maintenance boundary. The resident daemon does not open
maintenance work when the queue is empty and no task is open; it waits in
the idle state.

## Trigger

There is no automatic idle trigger. At a turn boundary with no open task and
no pending queue rows, the harness records `daemon_state=idle`, refreshes
the daemon lock heartbeat, and waits for the next queue row.

## Maintenance Directives

The pure runtime still defines maintenance directives for explicit future
use, chosen in rotation and weighted by staleness:

| Directive | Work |
| --- | --- |
| distill | Read recent transcript spans; write durable lessons via memory.save per [../memory/distillation.md](../memory/distillation.md) |
| refine-skills | Pick the skill with the oldest refinement stamp or the worst recent outcome; sharpen it via skill.save per [../skills/lifecycle.md](../skills/lifecycle.md) |
| prune-memory | Merge duplicate memory rows, rewrite vague entries, drop superseded ones |
| audit-self | Compare recent failures against contracts; record mismatches as memory entries tagged for the owner |

The harness tracks per-directive last-run stamps in the state table when a
cycle opens; the model never schedules itself.

## Bounds

- Explicit maintenance uses the same toolset, protocol, budgets, and YOLO
  authority as task work; it has no special sandbox inside the container
  blast radius.
- Explicit maintenance may mutate /data/workspace and /data: workspace
  files, git state and remotes, queue rows, memory rows, skills, config,
  and external services when credentials and network access exist.
- Maintenance still obeys the integrity invariants: one action per turn,
  bounded observations, real observations only, append-only transcript
  events, and queue preemption only at turn boundaries.
- A maintenance cycle that finds nothing useful ends early with agent.done
  and a one-line summary; an empty cycle is a valid, honest outcome.

## Heartbeat

Idle time is used only for the daemon lock heartbeat. Scheduled wakeups and
cron-style work remain out of scope per [../../vision/scope.md](../../vision/scope.md).

## Status

not implemented as automatic daemon behavior. The runtime crate retains pure
directive and budget helpers for explicit maintenance paths.
