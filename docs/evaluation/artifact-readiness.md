# Artifact Readiness

## Purpose

Define fixtures for scaffold-only and weak artifact content.

## Contract

Known-bad traces include a cookbook scaffold, tiny placeholder leaves, missing
README links, and readiness evidence recorded before content exists.
Known-good traces refuse readiness, name weak paths, and admit bounded repair.

## Required Cases

- `cookbook_missing_evidence`.
- `cookbook_scaffold_only_foundations`.
- `cookbook_missing_readme_links`.
- `cookbook_weak_content_audit`.
- `artifact-readiness-graph-evidence-bypass`.

## Pass Condition

Structure evidence and readiness evidence remain separate. Direct
`graph.evidence` cannot satisfy artifact readiness. Cookbook recipes and
techniques pass only when the profile-specific content sections are present.
Scaffold phrases in artifact examples or write payloads fail before they can be
recorded as readiness evidence.

## Verification

Run `cargo test -p lkjagent-tools doc_content_audit`.

## Status

partially implemented.
