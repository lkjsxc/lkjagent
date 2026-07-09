# Completion

## Obligations

Each matter stores required obligations and their evidence predicates. Examples:

- workspace file exists with expected current fingerprint;
- semantic record row and history agree with the file;
- artifact sections and manifest are complete;
- requested checks pass on final bytes;
- project diff matches intended paths.

## Completion Candidate

The model may emit a progress report but cannot create a completion candidate.
The reducer creates it only when all required predicates appear satisfied. The
verify phase recomputes each check from fresh state.

## Response Gate

An imperative request cannot close with readiness, initialization, future-tense
promises, or a plan alone. The report operation becomes eligible after effects
and checks. Its message is stored with links to evidence.

Naming produced paths and blocker-free results is a close-transaction
postcondition on the report message, not a prerequisite obligation. If message
construction or persistence fails, the matter stays completing and retries the
report without repeating prior effects.

## Invalidation

Any source change after a check invalidates the check and dependent response.
Any unresolved active, pending, failed, blocked, or unsuperseded operation
prevents completion.

## Close Transaction

Commit passed checks, completion event, lifecycle transition, and final
conversation message in one transaction. The TUI never infers success from a
generic event string.
