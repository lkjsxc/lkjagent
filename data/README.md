# Data Directory

## Purpose

Runtime data lives here during local runs. SQLite databases, logs, exchange
bodies, and workspace files are local runtime evidence and are not committed by
default.

Keep durable product behavior reproducible through tests, docs, and committed
proof bundles under `tmp/agent-runs/` or `tmp/live-runs/` when a packet requires
raw evidence.
