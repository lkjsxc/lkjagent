# Completion Gates

## Purpose

Define the artifact-specific checks required before owner completion.

## Gates

Completion requires:

- requested root exists.
- root README exists.
- manifest exists.
- every directory has a README.
- README files link every local child.
- sequence-only part files are absent unless explicitly requested.
- expected semantic groups exist.
- leaf files are content-bearing and not scaffold-only.
- profile-specific content readiness passes.
- line limits pass.
- requested artifact kind matches the manifest or readiness summary.
- requested scale is met or honestly bounded with evidence.
- artifact audit passes.
- unsupported verification claims are absent.
- graph plan, observation, and verification evidence exist.

Failed completion keeps the task open or blocked and returns a structured
handoff with missing gates and the next executable action.
