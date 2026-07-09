# Native Tools

## Workspace

- workspace_list: bounded direct children;
- workspace_tree: bounded depth and entries;
- workspace_search: lexical or trigram body search;
- workspace_read: current fingerprint and bounded excerpt;
- workspace_create: create a new managed file;
- workspace_replace: replace expected fingerprint;
- workspace_append: append with expected fingerprint;
- workspace_patch: apply a bounded patch;
- workspace_move: previewed atomic move;
- workspace_validate: links, sizes, and fingerprints.

## Records

- record_capture: deterministic owner facts;
- journal_compose: model-assisted dated reflection;
- todo_update, calendar_update, finance_update;
- project_note, decision_record, session_record.

## Artifacts And Maintenance

- artifact_write_section, artifact_continue;
- rebalance_plan.

## Development

- git_status, git_diff, git_log;
- verify_run from an allowed command catalog;
- repository_search and repository_read.

## Runtime

- context_inspect, conflict_resolve;
- operation_split, operation_replan;
- check_run.

Questions and progress or final messages are typed envelopes, not tools. They
cannot create effects or completion evidence.

All tools return attribute-free observation cards to the model and structured
internal results to the ledger.
