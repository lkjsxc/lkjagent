# Recovery Shape Enforcement

## Purpose

This task prevents repeated invalid model actions by forcing recovery action
shape changes for every fault class.

## Contract

Each fault records class, case, node, tool, action fingerprint, parameter shape,
retry count, ladder position, selected route, and next admitted action class.
The second same fault cannot admit the same action text as the only route.
Recovery preserves at least one productive escape tool for the failed mission.

## Inputs

- recovery ladder contract.
- runtime recovery policy and retry store.
- source graph recovery nodes and tool arrays.
- dispatcher refusal and registry example rendering.
- uploaded parse, schema, repeat, payload, and completion fault fixtures.

## Outputs

- route table for parse, parameter, tool, repeat, endpoint, budget, context,
  verification, compaction, maintenance, payload, and completion faults.
- dedicated route for attribute-like tags with contextual repair tags.
- rendered examples that parse, validate, and reach an admitted route.
- structured refusals naming the changed tool class.
- blocked handoff only when no internal route remains.

## Invariants

- Repeated `graph.next` cannot suggest `graph.next` again for the same fault.
- Repeated attribute-like path faults cannot keep rendering the same plan action
  as the only route.
- Payload overflow routes to `artifact.next` or bounded batch writes.
- Completion faults route to audit or missing evidence repair.
- `agent.ask` is refused unless external owner input is required.
- `shell.run` appears only from shell-admitted recovery.

## Failure Cases

- Recovery loops on the same parse-invalid action.
- Source graph recovery blocks `artifact.next` when content is missing.
- Batch syntax repair emits an example dispatch later rejects.
- Attribute-like `<path=...` output becomes a false missing-parameter report.
- Owner waiting is used for an internal inspection choice.

## Verification

- `cargo test -p lkjagent-runtime --test authority_recovery_plan`
- `cargo test -p lkjagent-runtime --test recovery_controller`
- `cargo test -p lkjagent-runtime --test fault_wait`
- `cargo test -p lkjagent-runtime --test fault_retry_store`
- `cargo test -p lkjagent-tools --test registry_examples`
- `cargo test -p lkjagent-tools --test dispatch_normalize`

## Status

partially implemented. Recovery route metadata, retry count storage, changed
action-class repeat routing, repeated batch-schema routing from `fs.batch_write`
to `artifact.next`, payload fault suppression of raw `fs.write` retries,
registry examples, and recovery-plan examples that parse, validate, are
admitted, and dispatch to local routes exist. Live shape-change enforcement for
every fault class and failed-route escalation remain open.
