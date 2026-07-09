# lkjagent Evidence-First Rebuild

## Purpose

This packet is an implementation program for lkjsxc/lkjagent. It is based on
the attached checkout at commit ae5ff551 and the public main branch ending at
2affb801. It supersedes earlier planning packets wherever they disagree.

## Central Conclusion

The repository has useful Rust primitives, but the state ledger is still a
projection over task and step rows. Recovery repeats impossible calls, generic
messages can close work without effects, and the live runner can replace a
blocked matter with a synthetic closed idle snapshot. This is why the harness
usually stops doing useful work after only a few decisions.

The required change is a fresh authority model:

    durable event
      -> reduced state vector
      -> selected operation
      -> compiled prompt or native effect
      -> admitted result
      -> evidence and checks
      -> next durable event

No model statement, editable checklist, historical run, or synthetic idle row
may claim completion.

## Read Order

1. Every file in 00-bootstrap, beginning with BOOTSTRAP_PROMPT.md.
2. 01-baseline/critical-findings.md
3. 02-product/product-contract.md
4. 03-runtime/authority-model.md
5. 08-implementation/dependency-graph.md
6. 09-evaluation/live-campaigns.md
7. 10-acceptance/final-gate.md

## Directory Map

- 00-bootstrap: downstream coding-agent control.
- 01-baseline: evidence from the supplied checkout.
- 02-product: target product and authority boundaries.
- 03-runtime: state, prompt, loop, recovery, and completion.
- 04-context: retrieval, budgeting, conflicts, and compaction.
- 05-protocol-tools: attribute-free protocol and scoped tools.
- 06-workspace: one visible workspace and its storage invariants.
- 07-tui: canonical transcript, ordering, and scrolling.
- 08-implementation: dependency-ordered work program.
- 09-evaluation: experiments and real daily-use campaigns.
- 10-acceptance: machine-derived gates.
- 11-subagents: parallel role contracts.
- 12-risks: risks, rejected approaches, and decisions.
- 13-scripts: packet and evidence validators.

## Non-Negotiable Outcome

The final checkout must complete real owner goals across daily records and
software projects, remain active while executable work exists, recover through
materially different strategies, and expose truthful evidence through the
workspace, SQLite, TUI, tests, clean checkout, Docker Compose, and repeated
live endpoint sessions.
