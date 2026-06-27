# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Repair current-state and authority status truth | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 2 | Reopen executable blocker queue and task contract | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 3 | Specify one effectful kernel driver contract | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 4 | Complete runtime authority store-chain reads | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 5 | Build durable snapshot adapter from store facts | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 6 | Close the runtime event catalog | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 7 | Cover reducer decisions for every mission | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 8 | Route daemon turns through the kernel driver | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 9 | Enforce decision-derived admission and staleness | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 10 | Render prompt frames from persisted decisions | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 11 | Persist provider anomalies as kernel events | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 12 | Canonicalize `fs.batch_write` schema repair | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 13 | Centralize semantic artifact lifecycle | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 14 | Strengthen story and long-novel readiness | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 15 | Centralize every completion attempt | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 16 | Restrict maintenance to closed idle | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 17 | Make compaction a resumable runtime effect | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 18 | Derive status, logs, and touched paths from authority | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 19 | Promote the active run into benchmark fixtures | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 20 | Delete or quarantine split authority policy paths | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |
| 21 | Run final local and Docker gates | [current-work/kernel-cutover-plan.md](current-work/kernel-cutover-plan.md) | implemented |

## Active Data Fixture

`data/logs/current-model-run.md` and `data/logs/index.ndjson` are failure
evidence. The checked-in run proves these facts:

- active case `1` is at node `document` in phase `execution`;
- owner task is `Create a long novel. with detailed settings`;
- pre-owner maintenance repeats empty memory searches, no-op pruning, and
  `agent.done` instead of staying closed idle;
- artifact root is `stories/long-novel-with-detailed-settings`;
- active tracks are `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- evidence ledger has `plan` and `observation` only;
- touched-path summary says `none`, despite scaffold creation and root listing;
- `doc.audit` failed readiness with 28 weak structure-only pages;
- two `fs.batch_write` attempts used child `<file>` tags inside `<files>` and
  both failed with `invalid parameter: each block must start with path:`;
- recovery repeated the invalid child-tag shape before changing route;
- turns 59 and 62 record `provider_anomaly.reasoning_only_response`;
- document audit and artifact readiness audit remain pending.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes a row.
- The transition kernel is the target runtime authority. Graph policy is
  snapshot guidance, not fallback dispatch authority after runtime refusal.
- Stale-action refusal uses queue head, case id, graph node and phase, active
  mode, artifact root and cursor, latest fault, missing evidence, compaction
  pressure, maintenance state, and prompt-frame head.
- Schema repair for `fs.batch_write` is active because the fixture repeats
  child `<file>` blocks after a schema refusal.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. Owner work preempts maintenance before endpoint and
  before dispatch.

## Remaining Proof Gaps

No current blocker proof gaps remain after focused tests, `quiet verify`, and
Docker Compose verify.
