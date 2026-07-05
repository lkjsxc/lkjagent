# Development Records

## Purpose

Define records for software development, projects, and proof work.

## Families

Known development `kind` values include:

- `project`: objective, state, milestones, decisions, and next actions;
- `development`: repository tasks, branches, tests, and code evidence;
- `reference`: API notes, local design facts, and owner-approved citations;
- `routine`: recurring verification, dependency checks, or maintenance;
- `proof`: evidence bundle manifest and command results.

## Repository Rule

External repositories may live under `workspace/repos/` or be referenced from
project records. Prompt admission must still use bounded context items with
fingerprints instead of unbounded transcript or repository dumps.

## Proof Links

Proof records link command output refs, check rows, artifact fingerprints,
provider exchange refs, and state-edge evidence. They explain what ran and what
was skipped; they do not make an unrun gate pass.
