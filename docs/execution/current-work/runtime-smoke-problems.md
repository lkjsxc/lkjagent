# Runtime Smoke Problem History

## Purpose

Preserve the historical live-smoke defects that motivated the runtime smoke
problem sweep. This page is evidence history, not active work. Current active
redesign work is tracked in
[dense-runtime-state-network.md](dense-runtime-state-network.md).

## Historical Evidence Sources

Pre-sweep local transcripts showed the defects before final repair:

- `tmp/user-story-smoke-data-fix/logs/current-model-run.md`: neutral long-novel
  title smoke that reached `open_task=none` only after noisy repair.
- `tmp/problem-probe-compact-data/logs/current-model-run.md`: long-novel title
  containing `Compact` that closed incorrectly.
- `tmp/user-smoke-data/logs/current-model-run.md`: earlier `Compact Smoke Novel`
  run with the same classification failure pattern.
- `tmp/runtime-smoke-ground-truth-20260629T051817Z/`: checked-in pre-change
  ground truth for Compact Compass and iwanna.

The checked-in `data/logs/current-model-run.md` remains a historical failure
fixture for the missing-root loop.

## Historical Defects

The old runs showed these defects:

- Compact-title story requests could route as compaction work and close without
  artifact audit.
- Named novel roots could degrade to generic roots such as
  `stories/novel-named`.
- Missing-root repair could repeat same-root audit, refused mkdir, and
  placeholder-root examples before write progress.
- Recovery examples could mention `stories/example-story` while a current owner
  root existed.
- A compact story-bible seed could pass close for large story work.
- Success evidence lived only in local transcripts instead of focused tests,
  benchmark cases, or tracked smoke evidence.

## Final Sweep Evidence

The completed sweep added focused tests, benchmark fixture updates, and final
Docker smoke evidence under:

- `tmp/runtime-smoke-final-iwanna-20260629T131603Z/`
- `tmp/runtime-smoke-final-compact-20260629T134111Z/`

Those runs preserved owner roots, avoided generic roots, reached
story-semantic readiness, emitted `agent.done`, and reached `open_task=none`
without the observed false close or noisy recovery loop.

## Active Successor Work

The dense runtime state network now hardens the architecture behind those
repairs. It adds typed intent profiles, denser fact and obligation rows, a total
resolver, runtime-owned deterministic effects, scale-aware artifact readiness,
central typed completion inputs, and prompt/admission staleness fingerprints.

## Related Files

- [runtime-smoke-ground-truth.md](runtime-smoke-ground-truth.md)
- [dense-runtime-state-network.md](dense-runtime-state-network.md)
- [../tasks/runtime-smoke-problem-sweep.md](../tasks/runtime-smoke-problem-sweep.md)
- [../tasks/dense-runtime-state-network.md](../tasks/dense-runtime-state-network.md)
