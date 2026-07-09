# Tool View

## Derivation

Derive the view from state, operation, evidence need, owner policy, remaining
budget, recovery strategy, and prior admissions. Persist allowed tools and
hidden reasons before prompt compilation.

## Typical Views

- capture-compose: workspace_search, workspace_read, record_capture or
  journal_compose;
- retrieve: workspace_search, workspace_read;
- project-inspect: workspace_tree, workspace_search, workspace_read, git_status;
- project-edit: workspace_read, workspace_patch, verify_run;
- artifact-section: artifact_write_section;
- verify: check_run;
- recover-parse: corrected action grammar only;
- recover-output-limit: artifact_continue or operation_split;
- maintenance-preview: workspace_validate, rebalance_plan;
- report: no mutation tools.

## Size

Most views contain one to four tools. Broader exploration needs an explicit
state and a reason. Shell is never silently added.

## Prompt

Render only the active tools, their required fields, primitive constraints, and
one concrete decision-bound example when the experiment supports it. Do not
render the global catalog.
