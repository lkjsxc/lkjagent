# Skill: Protocol Change

## Purpose

Change the action grammar, parser, or system prompt without breaking the
model's ability to act: grammar, parser, prompt, and fixtures move as one.

## Trigger

The action grammar, parser, or system prompt is changing.

## Context

- [../../architecture/protocol/action-format.md](../../architecture/protocol/action-format.md): the grammar contract.
- [../../architecture/protocol/parsing.md](../../architecture/protocol/parsing.md): rules and fault variants.
- [../../architecture/protocol/recovery.md](../../architecture/protocol/recovery.md): every new failure shape needs a taxonomy row.
- [../../architecture/protocol/system-prompt.md](../../architecture/protocol/system-prompt.md): the budgeted sections teaching the grammar.
- [../../decisions/xml-action-protocol.md](../../decisions/xml-action-protocol.md): the settled ground; moving it means updating it.

## Procedure

1. Write the grammar change into action-format.md with a real example, and
   the matching parser rule into parsing.md, including the new ParseFault
   variant if any.
2. Add a recovery taxonomy row for every new way the model can now fail,
   with its bounded harness response.
3. Update the system prompt grammar section and recount its token budget
   against [../../architecture/context/budgets.md](../../architecture/context/budgets.md);
   growth there must shrink elsewhere in the same change.
4. Extend the recorded-completion test table in lkjagent-protocol: clean
   cases, each fault variant, and pathological content exercising the new
   rule.
5. Implement parse and render changes as pure functions; render and parse
   must round-trip on every fixture.
6. If tool parameters changed shape, update
   [../../architecture/tools/registry.md](../../architecture/tools/registry.md)
   in the same change; the registry table and the prompt section derive
   from one source.

## Checks

- `cargo test -p lkjagent-protocol` passes with the new fixture rows; the
  round-trip property holds on all fixtures.
- The system prompt section rebuilt from the registry stays within its
  1,024-token budget (the build-time check fails otherwise).
- `cargo run -p lkjagent-xtask -- quiet verify` prints ok verify.

## Must Not

- Do not add escaping or entity rules; payloads that cannot be expressed
  route through shell heredoc by contract.
- Do not let the parser repair malformed output; recovery belongs to the
  loop with the model in it.
- Do not change grammar without updating the prompt that teaches it in the
  same commit.
- Do not invent fixtures for behavior the contract does not state.
