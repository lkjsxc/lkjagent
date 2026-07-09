# Maintenance

## Scope

Maintenance is deterministic by default:

- scan changed workspace paths;
- verify fingerprints and links;
- update affected indexes;
- compact oversized navigation pages;
- plan safe rebalancing;
- archive completed activity by retention policy;
- checkpoint and optimize SQLite when due.

## Scheduling

Wake maintenance from file changes, thresholds, or explicit due times. Do not
call the model on every idle poll. Model-assisted organization requires a
specific maintenance matter and verified preview.

## Safety

- Never rewrite owner prose silently.
- Preview moves and link edits.
- Use idempotent operation journal entries.
- Apply atomic renames and verify postconditions.
- Retain aliases until all references are repaired.
- Expose each maintenance result in workspace/system/operations.

## Fairness

Owner work outranks routine maintenance. Long maintenance splits into bounded
operations and yields between them so new owner input remains responsive.
