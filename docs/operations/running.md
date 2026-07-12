# Running

## Purpose

Define safe direct and Compose operation with separate roots.

## Direct

```sh
cargo run --locked -p lkjagent-app -- --data data send --new "OWNER TEXT"
cargo run --locked -p lkjagent-app -- --data data run
cargo run --locked -p lkjagent-app -- --data data status
cargo run --locked -p lkjagent-app -- --data data workbench
```

The current CLI accepts these commands. Its bridge limitations are recorded in
`../current-state.md` until direct cutover.

## Configuration

Place flat `lkjagent.json` below the selected data root. Keep the key in the
environment named by `endpoint_api_key_env`. Never print effective secret bytes.
Use a fresh isolated data root for trials; the supplied store has ambiguous and
retired authority rows.

## Compose

The target Compose contract mounts host data through `LKJAGENT_DATA_DIR` and host
workspace through `LKJAGENT_WORKSPACE_DIR`. Data appears at `/data`; owner files
appear at `/workspace`. The current Compose file still mounts only `/data` and is
an open cutover item.

## Lifecycle

Start the endpoint before the agent when Compose owns both. Health requires
process, store, workspace, and endpoint readiness; PID alone is insufficient.
Stop the daemon at an effect/provider boundary, capture Online Backup and logs,
then remove isolated containers without deleting host evidence.

## Existing Data

Do not run the fresh direct schema against old task/step stores. Preserve old
bytes for offline evidence and choose another data root. No product converter is
part of the active runtime.
