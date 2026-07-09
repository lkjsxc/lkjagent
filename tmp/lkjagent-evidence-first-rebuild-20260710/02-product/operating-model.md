# Operating Model

## One Daemon

One foreground daemon processes one effectful decision at a time. It may keep
many matters open, waiting, scheduled, or dormant. Concurrency may be added
later only around safe reads or external calls with explicit leases.

## Event-Driven Waiting

The daemon waits on owner input, due timers, endpoint retry times, filesystem
change notices, and maintenance schedules. It does not spend thousands of loop
iterations returning synthetic idle tasks.

## Long Work

Large goals become bounded semantic operations. The daemon repeatedly selects
the next dependency-ready operation, writes evidence, evaluates progress, and
replans when facts change. A small per-cycle quantum preserves responsiveness
without limiting total useful work to a few turns.

## Quiescence

Quiescent means no runnable or due work. It is a daemon condition, not a fake
closed matter. Quiescence does not call the model. Due deterministic maintenance
may wake it without owner input.

## Human Interruption

New owner turns enter a durable queue. They do not corrupt an active endpoint
call. The next selector pass routes them as an answer, update, separate matter,
or priority change through explicit causal edges.
