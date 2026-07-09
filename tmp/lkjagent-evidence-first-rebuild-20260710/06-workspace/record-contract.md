# Record Contract

## Managed Markdown

Each record contains:

- meaningful title;
- local date and time when relevant;
- kind and current status;
- source or provenance refs;
- owner-authored facts;
- optional generated synthesis clearly identified;
- related paths;
- concise content sections.

Every managed file carries a mandatory path-independent document ID. Use
lowercase ASCII type prefix, underscore, UTC creation stamp, and random
lowercase base32 suffix. The Markdown header has fixed fields for document ID,
kind, local effective date, state, sources, and structured related-document
IDs. Reject duplicate IDs, invalid escaping, unknown managed states, and
malformed headers.

After one H1 title, use this attribute-free metadata block:

    <lkjagent_record>
    <document_id>journal_20260608t120000z_k3m7q2</document_id>
    <kind>journal</kind>
    <effective_date>2026-06-08</effective_date>
    <state>active</state>
    <source_ref>activity_20260608t115900z_f4n2p8</source_ref>
    <related_document>project_20260501t000000z_r8c2w5</related_document>
    </lkjagent_record>

Scalar fields occur once. Source and related-document fields may repeat. XML
entities escape text. Body Markdown begins after the closed metadata block.

Immutable revision bytes, fingerprints, and operation metadata live in SQLite
or its content-addressed blob area. This permits exact recovery and rebuild.

## Command Separation

The raw owner command belongs in activity history and conversation evidence.
It is not copied into the semantic record unless the owner explicitly requests
verbatim capture.

## Capture And Compose

- capture writes explicit owner facts deterministically;
- compose uses the model to synthesize a useful record from admitted sources;
- update merges through expected fingerprint and preserves prior content;
- append adds a dated semantic section, not duplicate headers.

## Dates

Use configured owner timezone. Calendar uses event date. Finance uses transaction
date. Relative dates are normalized at intake and stored with their source text.

## Checks

Validate path family, size, title, non-placeholder content, source claims,
fingerprint, links, index membership, and command separation.

A successful record remains incomplete while required README or generated-index
debt is active.
