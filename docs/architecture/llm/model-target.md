# Model Target

## Purpose

Name the reference deployment the harness is sized against, and separate that
reference from the short list of properties the harness actually requires
from any endpoint.

## Reference Deployment

A roughly 14B-parameter open-weights model at 4-bit quantization on a 16 GB
memory budget, with a 32,768-token context window, served by a
llama.cpp-class server with prefix caching enabled. The budgets in
[budgets.md](../context/budgets.md) and the latency expectations are
calibrated against this deployment.

It is a reference point, not a dependency: nothing in the harness names a
model family, and swapping the model is one config edit per
[../../decisions/openai-endpoint.md](../../decisions/openai-endpoint.md).

## The Five Requirements

| Requirement | Why the harness needs it |
| --- | --- |
| the chat-completions route | the only wire format the client speaks ([endpoint.md](endpoint.md)) |
| returning the close tag | the parser must see the complete act block |
| a context window of at least 32k tokens | the window budgets assume it ([budgets.md](../context/budgets.md)) |
| prefix caching | acceptable turn latency; the design works without it, just slower |
| applying its own chat template | the harness sends a message list and never builds template bytes |

An endpoint with these five properties is a valid deployment, whatever model
it serves.

## What Degrades

- Smaller models trade capability for headroom. The protocol and the budgets
  are sized so a disciplined 7B-class model remains usable: the grammar is
  small, the prefix is stable, and observations are bounded.
- Absent prefix caching, every turn re-evaluates the whole window. Nothing
  breaks: correctness is unchanged, but turn latency grows with the length
  of the log.

## Boundary

Model serving is out of scope for the harness
([../../vision/scope.md](../../vision/scope.md)): it never loads weights,
never links inference libraries, and never manages model memory, per
[../../decisions/openai-endpoint.md](../../decisions/openai-endpoint.md).
The endpoint runs as a separate container or host process; the wiring lives
in [../../operations/compose.md](../../operations/compose.md).

## Status

design-only.
