# Skill: Agent Maintenance

## Purpose

Maintain the source surfaces that steer agents: AGENTS.md, the manual, and
this skill library. This mirrors explicit maintenance directives through
commits.

## Trigger

The manual, AGENTS.md, or this library needs maintenance.

## Context

- [../../../AGENTS.md](../../../AGENTS.md): the entry point being maintained.
- [../README.md](../README.md): the manual's authority table.
- [../../architecture/runtime/self-maintenance.md](../../architecture/runtime/self-maintenance.md):
  the directives this skill mirrors.
- [../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md): refinement and retirement bars.
- Recent session evidence: handoffs, commit history, and any friction the
  requesting user names.

## Procedure

1. Pick one explicit directive for the session:
   distill (turn recurring session friction into a skill or a manual fix),
   refine-skills (sharpen the stalest skill here against real session
   evidence), prune (collapse duplicated guidance into its owner), or
   audit-self (walk AGENTS.md and the manual against current contracts and
   fix drift).
2. For refinement, apply the lifecycle moves: tighten Trigger, repair
   Procedure steps evidence contradicted, extend Must Not with observed
   failures, shrink everything.
3. For retirement, delete the skill file, remove its index row in
   [README.md](README.md), and leave the why in the commit message.
4. Keep AGENTS.md a router: rules live in their owner files; AGENTS.md
   holds only the non-negotiables and pointers. Move anything deeper down
   into the tree.
5. Verify every path AGENTS.md and the manual reference still exists; fix
   or remove dead routes.
6. Run the docs gate and commit with the directive named in the intent
   line.

## Checks

- Docs gate passes: check-docs and check-lines ok lines, or the interim
  checks in [../../operations/verification.md](../../operations/verification.md)
  print nothing.
- Every skill in this directory still matches the format (the gate's
  skill-shape check covers this).
- AGENTS.md is at or under its line cap and every link in it resolves.

## Must Not

- Do not add rules to AGENTS.md that an owner file should hold.
- Do not refine a skill without evidence from real sessions; speculative
  edits are noise.
- Do not let this library grow without bound; one capability per skill,
  retire the unused.
- Do not touch product contracts under this directive; that is a different
  session.
