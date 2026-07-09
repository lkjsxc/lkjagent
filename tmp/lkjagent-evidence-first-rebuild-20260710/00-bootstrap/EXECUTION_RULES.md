# Execution Rules

## Documentation

- Treat docs as the current implementation contract.
- Every docs directory has one README table of contents and at least two useful
  children.
- Keep authored docs and Rust files at or below 200 lines.
- Use semantic filenames; do not create serial fragments or release-labelled
  copies.
- Delete superseded text instead of accumulating historical alternatives.
- Update current-state claims from machine evidence, not intent.

## Implementation

- Backward compatibility is not required.
- Prefer a pure functional core and explicit effects at boundaries.
- Avoid product mocks, placeholder bodies, bulk scaffolds, and fake success.
- Never create an artifact unless its requested content is actually present.
- Preserve owner workspace files across a store reset; rebuild projections.
- Use dedicated typed operations before shell.
- Reject unsafe or ambiguous effects before filesystem mutation.

## Verification

- A command that did not run did not pass.
- A local Docker pass is insufficient until a clean Git archive also passes.
- Commit Cargo.lock because the clean Docker build copies it.
- Every source commit invalidates older integration, live, and PTY evidence.
- A blocked live profile is a failure, never a closed or ran success.
- Historical evidence may become a regression fixture, never current proof.

## Git

- Commit the extracted packet unchanged as the anchor.
- Do not amend, rebase away, or squash the anchor or evidence commits.
- Commit docs before corresponding behavior.
- Use small coherent commits with exact Tested and Not-tested trailers.
- Keep unrelated owner changes intact.

## Autonomy

- Exhaust safe in-scope implementation and diagnostic work before asking.
- Ask only when a missing owner decision changes product semantics materially.
- Lack of a live endpoint is an unresolved required gate, not permission to stop.
