# Tests

## Purpose

Integration tests for lkjagent-app.

## Table of Contents

- [app.rs](app.rs): CLI help, scripted endpoint, and template tests.
- [cli-rows.rs](cli_rows.rs): row-backed CLI inspection tests.
- [context-cli.rs](context_cli.rs): owner context resolution command tests.
- [context-items.rs](context_items.rs): durable context hygiene tests.
- [contamination.rs](contamination.rs): contamination classification tests.
- [docs-tree.rs](docs_tree.rs): documentation tree app flows.
- [effect-error.rs](effect_error.rs): effect failure settlement tests.
- [endpoint.rs](endpoint.rs): endpoint adapter integration seam.
- [exchange.rs](exchange.rs): exchange refs and token usage persistence.
- [explore.rs](explore.rs): bounded explore registry behavior.
- [manuscript.rs](manuscript.rs): manuscript app flows.
- [prompt-frame.rs](prompt_frame.rs): prompt-frame body replay tests.
- [recovery.rs](recovery.rs): unfinished decision recovery tests.
- [resume.rs](resume.rs): row-first resume, waiting answer, and decision reuse tests.
- [state-snapshot.rs](state_snapshot.rs): state-cell snapshot hydration tests.
- [workspace-rebalance.rs](workspace_rebalance.rs): workspace manifest, rebalance, audit, and alias tests.
