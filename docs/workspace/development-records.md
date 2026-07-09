# Development Records

## Purpose

Define multiple-project source, decision, session, and verification records.

## Project Identity

Each project has a stable ID and root under `projects/<project-id>/`. Create only
used branches for its README, notes, decisions, sessions, artifacts, evidence,
and named repositories or validated linked paths.

## Records

Project notes live under `notes/YYYY/MM/`. Decisions record status, decided
time, superseded decision, rationale, and consequences. Sessions record start,
end, goal, work, checks, outcome, and next action. All carry the project ID.

## Repository Boundary

Git and filesystem effects remain bounded to the selected repository. Before an
edit, record inspected paths and fingerprints. After it, record the diff,
focused checks, broader Docker verification, workspace project note, and
remaining risks.

## Context Separation

A project matter selects only its metadata, named repositories, relevant source
excerpts, current operations, recent decisions, and verification evidence.
Similar names do not permit cross-project retrieval. A readiness message cannot
satisfy a source-change obligation.

## Proof

Proof records reference command output, checks, document revisions, effect and
observation rows, provider exchanges, and source commits. They never promote an
unrun command or skipped required capability.
