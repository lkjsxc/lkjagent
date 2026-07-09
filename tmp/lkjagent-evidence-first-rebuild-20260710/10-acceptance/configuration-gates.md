# Configuration Gates

## Shape

- Root must be one flat JSON object.
- Nested objects, any arrays, unknown keys, empty names, wrong types, and
  invalid ranges fail startup.
- Arrays are rejected; the anchored registry contains scalar keys only.

## Registry

- Every documented key exists in the typed registry.
- Every registry key has default, range, secrecy, reload policy, and consumer.
- Generated example and reference match the registry.
- No declared key is diagnostics-only unless its purpose is diagnostic.
- Tracked data/lkjagent.json contains exactly the anchored registry keys and
  scalar values; a named nonempty Docker integration suite probes behavior.

## Behavior

- CLI, environment, file, and default precedence is deterministic.
- Changing prompt_context_tokens changes the compiled prompt cap.
- Changing workspace_file_max_tokens changes managed-file admission.
- Timezone changes local semantic dates after restart.
- Restart-only keys do not partially hot-reload.

## Secrets

Secret values remain in named environment variables. Raw configuration and
secret values never appear in model prompts, workspace records, committed logs,
proof bundles, or error messages.

## Deployment

Compose, health checks, CLI, live runner, clean checkout, and public CI use the
same key names and root semantics.
