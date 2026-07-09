# Data Directory

## Purpose

Runtime data lives here during local runs. SQLite databases, exchange logs,
workspace files, and flat local configuration are local runtime evidence and are
not committed by default.

## Configuration

`data/lkjagent.json` is a tracked flat JSON configuration containing every key
from [the configuration registry](../docs/product/configuration-registry.md).
Values are strings, integers, or booleans. Unknown keys, arrays, nested values,
wrong types, invalid bounds, and cross-key conflicts are startup errors. Secret
values remain in the environment variable named by `endpoint_api_key_env`.
Model-visible prompts do not include the raw JSON config blob.

## Workspace

`data/workspace` is the default owner-visible memory tree. The owner should be
able to inspect records, transcripts, artifacts, indexes, and proof files there
without asking the agent.
