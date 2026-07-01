# Completion Gates

## Purpose

Define the artifact-specific checks required before owner completion.

## Required Gates

Content artifact completion requires all of these gates:

- requested root exists and matches the requested artifact kind.
- root README exists and links local children.
- manifest exists, including `catalog.toml` for recursive docs artifacts.
- every directory has a README.
- every README links every local child.
- sequence-only part files are absent unless explicitly requested.
- expected semantic groups exist for the request profile.
- leaf files are content-bearing and not scaffold-only.
- profile-specific content readiness passes.
- line limits pass before and after mutation.
- requested scale is met or honestly bounded with evidence.
- artifact audit passes.
- unsupported verification claims are absent.
- graph plan, observation, and verification evidence exist.

## Evidence Ownership

`artifact-readiness` and `document-structure` are audit-owned. Direct
`graph.evidence` cannot satisfy them. A graph observation may point to audit
output, but the readiness state comes from `artifact.audit` or `doc.audit`.
`artifact.audit` observations include the current `artifact_ledger_id`.

## Refused Completion

Failed completion keeps the task open or blocked. The refusal names the active
mode, failed gate, missing evidence, existing evidence, failed checks, and one
next executable action admitted by the same effective policy.

Planning evidence, file existence, README-only output, scaffold-only text, and
unsupported verification claims never close an owner task.

## Status

implemented for the central runtime completion gate. The gate refuses missing
objectives, missing evidence, latest recovery faults, weak artifact paths,
missing artifact readiness, missing content atoms, missing manuscript paths, and
manuscript word counts below the floor. It allows completion only from ready
artifact projections with blockers cleared. Live endpoint completion proof
remains a separate operator-run blocker.
