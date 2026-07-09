# Records

## Purpose

Define the common owner-readable managed-document contract.

## Metadata

Every managed Markdown file has one H1, then an attribute-free metadata block:

```text
<lkjagent_record>
<document_id>journal_20260608t120000z_k3m7q2</document_id>
<kind>journal</kind>
<effective_date>2026-06-08</effective_date>
<state>active</state>
<source_ref>activity_20260608t115900z_f4n2p8</source_ref>
</lkjagent_record>
```

Document IDs are path-independent, lowercase ASCII type prefix, UTC creation
stamp, and random lowercase base32 suffix. Scalar fields occur once. Source and
related-document fields may repeat. Unknown managed fields, states, duplicate
IDs, and invalid escaping fail import.

## Content

Body Markdown begins after metadata and contains concise semantic sections.
Managed content names kind, current state, effective local date, source refs,
owner facts, generated synthesis when present, and related documents.

Raw owner commands remain conversation or activity evidence. They are not copied
into semantic records unless verbatim capture was requested. Capture writes
explicit facts deterministically. Compose synthesizes from admitted sources.

## Revisions

SQLite stores immutable exact revision bytes, SHA-256, parent revision,
tokenizer and token counts, creating operation, and admission. Document identity
survives moves, state changes, archive, aliases, and tombstones.

## Size

Managed memory and navigation pages admit at most 512 conservative tokens. Long
artifacts split by named semantic section with an outline and manifest. External
owner and project files keep their natural format and enter context only as
bounded fingerprinted excerpts.

## Checks

Validate path family, title, metadata, non-placeholder body, sources,
fingerprint, links, index membership, token count, and command separation.
