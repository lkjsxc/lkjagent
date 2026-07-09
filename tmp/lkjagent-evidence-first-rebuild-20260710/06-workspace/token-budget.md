# Token Budget

## Measurement

Use the configured model tokenizer plus the deterministic
`lkjagent-conservative-unicode` measure. Split Unicode scalar values into ASCII
and non-ASCII runs. Each non-ASCII scalar counts one; each contiguous
ASCII alphanumeric run counts ceiling(bytes/3); each non-whitespace ASCII
punctuation scalar counts one. Whitespace counts zero. The admission count is
the larger of provider tokens and this measure. Byte-count divided by four is
forbidden because it undercounts Japanese. Record both counts and tokenizer IDs.

## Managed Documents

Enforce at most 512 admission tokens for every managed memory document and
generated navigation page. If the provider tokenizer is unavailable, the
conservative count remains authoritative. Reserve space for title, metadata,
and links before writing the body.

## Semantic Splitting

For long artifacts:

1. create an outline;
2. assign semantic section names;
3. generate each bounded section;
4. verify section checks and links;
5. assemble a concise manifest or index.

Do not create anonymous numbered fragments. Project source files follow the
repository's source-line contract rather than the memory-document token target.

## Oversize Input

Externally edited large files remain source data. Retrieval uses bounded
excerpts with offsets and fingerprints. Maintenance may propose semantic
reorganization but cannot rewrite owner text silently.

A raw owner turn over the ceiling is preserved as content-addressed activity
bytes plus several semantic, source-linked activity pages. No byte is lost and
no single managed Markdown page exceeds the ceiling.

## Tests

Cover Japanese, mixed-width text, emoji, long links, headers, README pagination,
and exact boundary behavior.
