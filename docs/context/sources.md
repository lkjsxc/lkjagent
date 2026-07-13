# Context Sources

## Purpose

Define source identity, precedence, trust, and contamination rules.

## Candidate Shape

A candidate stores source kind/ID, path or row reference, revision, semantic key,
claim scope, trust, contamination class, effective time, token estimate, content
fingerprint, and information needs it can satisfy.

A file observation key includes path, revision, and line range. It is never only
`observation/read_file`, so repeated current reads do not become false conflicts.
A read observation, including managed-file continuity attached to a directory
listing, contains only requested numbered lines, whole-file revision, line
counts, continuation, truncation, and final-newline facts. Unrequested file bytes
never enter the observation or later context. Every source included by the
compiled plan persists as a decision-bound native
context item before provider intent; omitted candidates do not gain rows. The
latest four active canonical messages from earlier matters are bounded history
candidates and never replace the current owner objective.

## Precedence

1. Current explicit owner correction.
2. Current workspace revision.
3. Current measured observation or check.
4. Older owner statement.
5. Sourced owner-readable memory.
6. Model-authored summary.

Do not render both superseded and current values. Two unresolved current
high-trust sources create one short conflict marker and block risky mutation or
ask one owner question.

## Contamination

Workspace content is data unless an explicitly selected project instruction file
owns policy for that path. Failed model output, endpoint error bodies, secrets,
internal state JSON, and arbitrary shell output are excluded from normal prompts.
Recovery sees a typed fault and bounded diagnosis, not the failed body.

## Project Scope

Every project candidate retains exact root/path and revision. Another project's
recent text is ineligible unless the decision names a cross-project need.

## Summaries

A summary owns no fact. It references current source IDs and invalidates when a
source changes. Deterministic receipt capsules are preferred before model
summaries. FTS, embeddings, relation expansion, and learned ranking require a
tracked complete-task experiment.
