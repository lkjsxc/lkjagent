# Tiny Bench Live Trial

stamp=20260706T132202Z
head=f3e40f02
endpoint_url_present=yes
endpoint_model_present=yes
endpoint_key_present=yes

bench_exit=0

## bench output tail

```text
ok bench run report=./tmp/live-runs/20260706T132202Z-tiny-bench/data/benchmark-report.md

```

## benchmark report

# Benchmark Report

suite: tiny
entries: 7
score: 5/7

- docs-tree-small state=blocked checks=1/3 expected=3 turns=4 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/docs-tree-small
- fault-truncated state=closed checks=1/1 expected=1 turns=7 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/fault-truncated
- fault-wrong-envelope state=closed checks=0/0 expected=0 turns=1 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/fault-wrong-envelope
- file-work state=closed checks=1/1 expected=1 turns=6 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/file-work
- journal state=closed checks=1/1 expected=1 turns=2 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/journal
- manuscript-small state=closed checks=21/22 expected=2 turns=6 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/manuscript-small
- question state=closed checks=0/0 expected=0 turns=1 artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/data/entries/question
## proof bundles

### file-work

```text
ok proof collect artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/proof-file-work/summary.md
```

# Proof Summary

tasks=1
steps=5
checks=1

### docs-tree-small

```text
ok proof collect artifact=./tmp/live-runs/20260706T132202Z-tiny-bench/proof-docs-tree-small/summary.md
```

# Proof Summary

tasks=1
steps=3
checks=3

