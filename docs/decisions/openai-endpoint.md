# OpenAI-Compatible Endpoint

## Purpose

Fix how the harness reaches its model.

## Decision

The harness talks to exactly one OpenAI-compatible chat-completions HTTP
endpoint, configured by URL and model name. The reference deployment is a
llama.cpp-class server running a roughly 14B open model at 4-bit on a 16 GB
budget with a 32k context window. The endpoint contract is
[../architecture/llm/endpoint.md](../architecture/llm/endpoint.md).

The harness never loads model weights, never links inference libraries, and
never manages GPU or CPU memory for the model.

## Consequences

- The model is swappable by editing one config value; the harness only
  depends on the wire format and the prefix-cache behavior.
- Prompt-cache friendliness must be earned on the harness side by keeping the
  serialized message list append-only between compactions.
- The endpoint is a separate container or host process; compose wiring lives
  in [../operations/compose.md](../operations/compose.md).
- Endpoint failures are a first-class recovery case in
  [../architecture/protocol/recovery.md](../architecture/protocol/recovery.md).

## Rejected Directions

- Embedding llama.cpp over FFI: tighter KV-cache control, but it welds the
  harness to one inference stack and bloats the binary and the build.
- Multi-provider abstraction: this project serves one local model; provider
  matrices are accidental complexity here.
- Raw completion endpoint with a hand-built chat template: discards the
  template correctness the server already owns.
