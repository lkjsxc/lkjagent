# Runtime Forensics

## Current User Data

The supplied SQLite store contains three owner turns:

- hello: closed after explore, respond, and close decisions;
- create something to read about world history: blocked;
- are you ok: closed after one response and close.

There are zero tool admissions, observations, checks, artifacts, and workspace
records in this data set.

## World-History Failure

The model planned three long writes to the same path. The first requested at
least 1,500 words, but the runtime set max output to 768 tokens. The endpoint
hit the cap and the runtime discarded the incomplete content.

Recovery added a note saying the attempt must change shape or scope, but it
kept the same 768-token cap, the same 1,500-word instruction, the same output
path, and nearly the same prompt. The second call failed identically. A recovery
decision then led to completion blocked while later operations remained pending.

## Root Cause

This is not an endpoint-quality problem. Planning does not validate feasibility,
content work is not decomposed into bounded semantic units, and recovery state
does not own an executable repair strategy.

## Required Fixture

Preserve this sequence as a regression. The replacement must preflight output
size, generate semantic sections, write each section atomically, verify the
assembled artifact, and never retry an unchanged impossible call.
