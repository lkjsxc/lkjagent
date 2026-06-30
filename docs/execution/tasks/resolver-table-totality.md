# Resolver Table Totality

## Purpose

Convert fallback-shaped runtime resolver logic into named total resolver rules.

## Status

done: resolver rules are named, rule ids and progress keys persist, focused
runtime and store tests, quiet verify, and Docker verify passed

## Depends On

- [observability-render-redesign.md](observability-render-redesign.md)

## Files To Read

1. [Transition network](../../architecture/state-graph/transition-network.md)
2. [Transition kernel](../../architecture/runtime/authority/transition-kernel.md)
3. [Obligation network](../../architecture/runtime/obligation-network/README.md)
4. [Runtime authority](../../architecture/runtime/authority/README.md)
5. `crates/lkjagent-runtime/src/kernel/resolver.rs`
6. `crates/lkjagent-runtime/src/kernel/resolver_rules.rs`
7. `crates/lkjagent-store/src/schema_authority.rs`

## Files To Touch

- `docs/architecture/runtime/authority/resolver-table.md` or obligation-network equivalent
- `docs/architecture/runtime/authority/README.md`
- `docs/_meta/catalog/architecture.toml`
- `crates/lkjagent-runtime/src/kernel/resolver*.rs`
- `crates/lkjagent-store/src/schema_authority.rs`
- focused runtime and store tests

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-runtime resolver
cargo test -p lkjagent-store runtime_authority
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- No runtime module is named or treated as fallback resolver logic.
- Every non-terminal owner case yields one decision variant or a typed blocked
  handoff with exact missing facts.
- The selected resolver rule id and progress key are persisted and visible.
- Table tests cover each obligation and root status.
- Historical recovery fixtures do not repeat invalid action classes.

## Must Not

- Do not add another mission ladder outside the resolver table.
- Do not let graph guidance admit tools after runtime refusal.
- Do not close no-match cases as success.
