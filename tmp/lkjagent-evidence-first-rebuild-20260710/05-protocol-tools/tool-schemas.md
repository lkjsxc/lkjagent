# Tool Schemas

## Types

`path` is normalized relative UTF-8 under the admitted root. `fingerprint` is a
current `sha256:` value. `id` is a ledger identity. `count` and `tokens` are
bounded nonnegative decimal integers. `text` is bounded escaped UTF-8. A slash
separates required input fields; brackets mark optional fields. No tool accepts
an undeclared field or raw shell command.

## Workspace

| Tool | Input fields | Observation |
|---|---|---|
| workspace_list | path / max_entries | children, kinds, truncation |
| workspace_tree | path / max_depth / max_entries | bounded nodes |
| workspace_search | query / scope / max_results / max_tokens | ranked source refs |
| workspace_read | path / max_tokens / [start_line] | excerpt and fingerprint |
| workspace_create | path / kind / title / body / effective_date | document and revision IDs |
| workspace_replace | path / expected_fingerprint / body | new revision |
| workspace_append | path / expected_fingerprint / body | new revision |
| workspace_patch | path / expected_fingerprint / patch | new revision and hunks |
| workspace_move | source_path / target_path / expected_fingerprint | moved document ID |
| workspace_validate | path / check_set | measured checks |

## Semantic Records

| Tool | Input fields | Observation |
|---|---|---|
| record_capture | kind / title / effective_date / facts / source_ref | record path and checks |
| journal_compose | effective_date / source_ref / focus | journal revision and provenance |
| todo_update | target_mode / title or document_id / state / [due] / [priority] | TODO revision |
| calendar_update | target_mode / title or document_id / start / end / timezone | event revision |
| finance_update | target_mode / document_id or payee / amount / currency / date / account | finance revision |
| project_note | project_id / title / body / source_ref | note revision |
| decision_record | project_id / title / decision / rationale / source_ref | decision revision |
| session_record | project_id / started / ended / outcome / source_ref | session revision |

## Development And Artifacts

| Tool | Input fields | Observation |
|---|---|---|
| git_status | repository_path | bounded status |
| git_diff | repository_path / scope / max_tokens | diff excerpt and full ref |
| git_log | repository_path / max_entries | commit facts |
| repository_search | repository_path / query / max_results | matches |
| repository_read | repository_path / path / max_tokens | excerpt and fingerprint |
| verify_run | repository_path / allowed_command_id | exit, log ref, checks |
| artifact_write_section | artifact_id / unit_id / body / source_ref | unit fingerprint |
| artifact_continue | artifact_id / after_unit_id / body / source_ref | unit fingerprint |

## Runtime And Maintenance

| Tool | Input fields | Observation |
|---|---|---|
| context_inspect | source_ref / max_tokens | source-linked details |
| conflict_resolve | conflict_id / chosen_source / rationale | resolution event |
| operation_split | operation_id / unit_titles | child operation IDs |
| operation_replan | operation_id / strategy / evidence_ref | superseding operation |
| rebalance_plan | scope / max_moves | non-mutating plan and checks |
| check_run | check_id | fresh measured result |

Shell is a separate state-scoped fallback. Its input is an allowlisted command
ID plus typed arguments; it never accepts an arbitrary command string through
this model protocol.
