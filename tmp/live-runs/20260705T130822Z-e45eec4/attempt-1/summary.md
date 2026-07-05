# Live Attempt 1

## Purpose

Bounded live endpoint attempt with secrets redacted.

## Timing

- start: 2026-07-05T13:08:22Z
- end: 2026-07-05T13:10:27Z
- daemon_status: 124

## Commands

- cargo run -p lkjagent-app -- --data tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data send --new <objective>
- timeout 120s cargo run -p lkjagent-app -- --data tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data run
- cargo run -p lkjagent-app -- --data tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data status
- cargo run -p lkjagent-xtask -- proof collect --data tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data --out tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/proof-bundle

## Objective

Live attempt 1: create a concise proof note under reports/live-attempt-1.md explaining what lkjagent changed today. Close only after deterministic checks pass.

## Endpoint Evidence

- endpoint_url: present
- model: present
- credential: present-redacted

## Final Status

```text
daemon: idle
task: none
step: none
last: none
question: none
queue: 0 pending
tokens: task in=3776 out=222 cached=unknown
lease: active owner=pid:3200858 heartbeat=unix:1783257026.255026850Z
state: active=1 conflicts=0
decision: case-1-decision-0008 completion.close status=settled ctx=fnv1a64:18ac6042 tools=fnv1a64:b4e036f2
admissions: 6 observations: 6 exchanges: 7 artifacts: 0

```

## Doctor

```text
doctor: warn
schema: tables=25 missing=none
table_counts: queue=1,tasks=1,steps=2,attempts=7,check_results=0,events=2,memory=0,memory_fts=0,token_usage=7,config=2,cases=1,runtime_events=25,state_cells=4,state_history=25,runtime_decisions=8,prompt_frames=7,tool_admissions=6,observations=6,context_items=6,context_edges=0,state_edges=0,workspace_records=0,workspace_record_history=0,artifacts=0,provider_exchanges=7
lease: active owner=pid:3200858 heartbeat=unix:1783257026.255026850Z
endpoint: url=env model=env credential=env
workspace: root=tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data/workspace missing=workspace/records,workspace/artifacts,workspace/indexes
decisions: unfinished=0
prompt_refs: orphan=0
warnings: missing-workspace-dirs

```

## Workspace

```text
workspace: root=tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/data/workspace
records: total=0 archived=0
artifacts: total=0
indexes: files=0
readmes: workspace=false,records=false
missing: workspace/records,workspace/artifacts,workspace/indexes

```

## Proof

- proof_bundle: tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/proof-bundle
- proof_log_tail: ok proof collect artifact=./tmp/live-runs/20260705T130822Z-e45eec4/attempt-1/proof-bundle/summary.md

## Adoption

Raw logs are preserved under this ignored tmp directory. No protocol or context change is adopted from this attempt without separate review.
