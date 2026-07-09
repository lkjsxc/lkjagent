# Context Safety

## Data Versus Instruction

Workspace and external text are untrusted data unless the owner explicitly
designated a policy file through configuration. Render excerpts inside source
cards with a data-only boundary. Never concatenate file text into the kernel.

## Contamination

Classify:

- clean;
- untrusted-instruction;
- failed-output;
- recovery-only;
- secret-bearing;
- malformed;
- stale.

Normal states admit only clean candidates. Recovery states receive bounded
recovery-only summaries, not raw failed output. Harness structures are projected
to attribute-free path/value cards. An explicitly requested project JSON excerpt
may appear only as inert, escaped, source-linked evidence when its syntax is
necessary to inspect or edit the file.

## Secrets

Redact API keys, authorization headers, environment values, and configured
secret patterns before durable observations. Raw provider requests use
restricted local logs and are not copied into committed evidence.

## Validation

Fail prompt compilation if raw harness serialization, unrequested JSON blobs,
raw secret markers, unresolved material conflicts, missing source fingerprints,
or unexpected instruction language appears in context lanes.
