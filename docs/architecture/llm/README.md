# LLM

## Purpose

This directory specifies the endpoint client: how the harness talks to its
one OpenAI-compatible chat-completions server. The wire subset is narrow on
purpose: a handful of request fields, a handful of response fields, whole
completions only. The client is owned by the lkjagent-llm crate, the only
crate in the workspace where HTTP and serde appear. Decision:
[../../decisions/openai-endpoint.md](../../decisions/openai-endpoint.md).

## Table of Contents

- [endpoint.md](endpoint.md): the wire contract, request and response subsets, and error mapping.
- [model-target.md](model-target.md): the reference deployment and the five properties the harness requires.
- [sampling.md](sampling.md): the sampling values, their rationale, and the constancy-within-session rule.
