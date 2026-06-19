# Self-Maintenance

## Purpose

State the self-maintenance boundary. The resident daemon opens bounded
graph maintenance work when the queue is empty and no user task is open, so
it keeps improving state instead of stopping at a quiet idle loop.

## Trigger

At a turn boundary with no open task and no pending queue rows, the harness
opens the stalest due maintenance directive and records
`daemon_state=working`. Recent last-run stamps keep the daemon idle until a
directive is due again or owner work arrives. The next endpoint turn receives
the maintenance notice and must choose one bounded improvement action or
close the cycle honestly with agent.done. A new queue row preempts
maintenance at the next turn boundary before another maintenance endpoint
turn is sent.

## Maintenance Directives

Maintenance directives are chosen in rotation and weighted by staleness:

| Directive | Work |
| --- | --- |
| distill | Read recent transcript spans; write durable lessons via memory.save per [../memory/distillation.md](../memory/distillation.md) |
| refine-skills | Record source skill improvement candidates from observed gaps |
| prune-memory | Merge duplicate memory rows, rewrite vague entries, drop superseded ones |
| audit-self | Compare recent failures against contracts; record mismatches as memory entries tagged for the owner |

The harness tracks per-directive last-run stamps in the state table when a
cycle opens. A directive is due after its cooldown interval passes. The
daemon, not the model, decides when the next cycle starts.

## Bounds

- Maintenance uses the same toolset, protocol, budgets, and YOLO
  authority as task work; it has no special sandbox inside the container
  blast radius.
- Maintenance may mutate /data/workspace and /data: workspace files, git
  state and remotes, queue rows, memory rows, config, and external services
  when credentials and network access exist.
- Maintenance still obeys the integrity invariants: one action per turn,
  bounded observations, real observations only, append-only transcript
  events, and queue preemption only at turn boundaries.
- A maintenance cycle that finds nothing useful ends early with agent.done
  and a one-line summary. Recent stamps delay the next idle restart so
  maintenance cannot spin in a tight endpoint loop.

## Heartbeat

Idle time is used for heartbeat plus bounded maintenance cycles. Scheduled
wakeups and cron-style timers remain out of scope per
[../../vision/scope.md](../../vision/scope.md); the resident loop simply
continues polling.

## Status

implemented as automatic daemon behavior.
