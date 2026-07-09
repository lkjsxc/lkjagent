# Commit Protocol

## Slice Shape

Each behavior slice normally has:

1. docs contract commit;
2. failing regression commit when useful;
3. implementation and focused pass commit;
4. integration or evidence commit.

Do not create a commit that only adds empty types, unused modules, placeholder
methods, or a broad scaffold.

## Message

State the concrete behavior. Include:

- Why;
- Changed docs, source, and tests;
- exact Tested commands and results;
- exact Not-tested commands and reasons;
- evidence path when behavior is claimed.

## Evidence Freshness

Evidence records the commit it tested. A later source or behavior-doc commit
marks integration, Docker, live, PTY, and final verifier evidence stale.

## Frequency

Commit after a complete small vertical behavior, not after every file and not
after an entire multi-day redesign. Preserve bisectable green states where
possible.

## Handoff

Name anchor, final commit, public branch state, dirty paths, completed workgraph
nodes, failed nodes, commands, raw evidence, adopted experiments, rejected
experiments, and next executable action.
