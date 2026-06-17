# Skill: Web Research

## Purpose

Use curl-backed external research when the answer depends on information
outside the container.

## Trigger

The task depends on current external facts, official references, or source-backed claims.

## Context

- Prefer official or upstream sources before third-party summaries.
- Local repository docs still outrank external pages for local contracts.
- The transcript needs URLs and dates for claims that may change.

## Procedure

1. State the exact question the external source must answer.
2. Run `curl -I --max-time 10 <url>` to confirm the source is reachable.
3. Fetch only the needed page or document and extract the relevant facts.
4. Cross-check with a second official source when the claim is high impact.
5. Cite the URL and retrieval date in the final answer or handoff.

## Checks

- `curl -I --max-time 10 <url>` returns an HTTP status line such as `HTTP/2 200`.
- The answer cites the source URL used for the claim.
- Any unsupported inference is labeled as an inference.

## Must Not

- Do not browse when local contracts already answer the question.
- Do not cite a source you did not read.
- Do not paste large copyrighted passages into the transcript.
