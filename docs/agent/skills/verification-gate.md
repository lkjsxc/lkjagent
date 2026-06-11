# Skill: Verification Gate

## Purpose

Build or change a gate so that it stays quiet, honest, and identical
between local, compose, and CI runs.

## Trigger

A gate is being built or its checks are changing.

## Context

- [../../operations/verification.md](../../operations/verification.md): the gate table and the quiet contract.
- [../../repository/documentation-standards.md](../../repository/documentation-standards.md): what check-docs must enforce.
- [../../repository/line-limits.md](../../repository/line-limits.md): what check-lines must enforce.
- [../../repository/functional-style.md](../../repository/functional-style.md): what check-style must scan for.
- [../../operations/compose.md](../../operations/compose.md): the no-source-mount rule the final gate proves.

## Procedure

1. Update the gate's row in verification.md first: what it checks, in one
   clause per check. The doc row is the gate's contract.
2. Implement in lkjagent-xtask: each check is a pure function from
   repository facts (file list, line counts, parsed docs) to a list of
   violations; the runner does IO once, then judges purely.
3. Honor the quiet contract exactly: silence during the run, one ok line
   on pass, the failing step plus a bounded output tail on fail, correct
   exit codes.
4. Make every violation message name the file, the rule, and the fix
   direction in one line; agents repair from these messages without
   opening this skill.
5. Test the gate on fixtures: a conforming tree fragment and one fixture
   per violation class, asserting exact messages.
6. Run the new gate against the real repository; fix what it finds or fix
   the gate, but never special-case real files in gate code.
7. Keep compose and CI thin: both call the same xtask gate, nothing else,
   per verification.md.

## Checks

- The gate's fixture tests pass: `cargo test -p lkjagent-xtask`.
- The gate passes on the repository: its ok line appears.
- A deliberately broken fixture file makes it fail with the documented
  message and nonzero exit.
- `docker compose run --rm verify` agrees with the local verdict.

## Must Not

- Do not print progress, banners, or timing on success; one line is the
  contract.
- Do not encode a rule in the gate that no doc owns; write the doc rule
  first.
- Do not let CI run different commands than the local gate.
- Do not weaken a check to make the current tree pass without a decision
  record explaining why the rule changed.
