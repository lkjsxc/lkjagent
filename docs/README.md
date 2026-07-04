# Documentation

## Purpose

Map the lkjagent contract. Read these directories in order when changing the
product.

## Read Order

1. [current-state.md](current-state.md): what is specified, implemented, and
   open now.
2. [vision/](vision/README.md): mission, invariants, and scope.
3. [state/](state/README.md): state cells, reducers, selectors, and arbitrary
   state keys.
4. [runtime/](runtime/README.md): durable runtime decisions, loop, recovery, and
   completion.
5. [product/](product/README.md): daemon lifecycle, queue, CLI, status, and
   console.
6. [engine/](engine/README.md): plan-family helpers during the state-ledger
   transition.
7. [context/](context/README.md): context items, contradictions,
   contamination, budgets, and prompt frames.
8. [protocol/](protocol/README.md): model output envelopes and faults tied to
   decisions.
9. [tools/](tools/README.md): catalog, policy, tool views, admissions,
   observations, and guards.
10. [checks/](checks/README.md): deterministic checks and word counting.
11. [memory/](memory/README.md): memory rows and retrieval.
12. [store/](store/README.md): SQLite schema, exchange logs, crash resume.
13. [llm/](llm/README.md): endpoint client, sampling, and generation budgets.
14. [operations/](operations/README.md): running, verification, and proof
    bundles.
15. [evaluation/](evaluation/README.md): benchmarks, replay, and live proof.
16. [repository/](repository/README.md): layout, file limits, docs rules, style,
    and commits.
17. [agent/](agent/README.md): instructions for coding agents.
18. [decisions/](decisions/README.md): recorded design decisions.
