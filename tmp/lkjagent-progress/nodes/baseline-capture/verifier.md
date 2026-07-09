# Findings

Source commit `9f851ffd7961e39c66a6ade3e3672423e3e9e9b8` descends from
the supplied base. Product crates are unchanged; the post-anchor changes are
limited to verification docs and the xtask node gate with focused tests.

The gate validates command exits, SQLite counts, the repeated 768-token
decision sequence, request bounds, repository anchors, bounded historical
claims, diary output, the clean-checkout failure, and contained raw evidence
references. Independent WAL-aware SQLite reads found three tasks, thirteen
steps, ten decisions, six exchanges, and zero admissions, observations,
artifacts, or workspace records. Case 2 is blocked after two identically capped
write decisions.

The diary manifest paths, sizes, and SHA-256 values match the files, SQLite
integrity is `ok`, and exactly one canned journal record exists. The six
critical failures are honestly split into three reproduced and three bounded.

Inspected SHA-256 values:

- `raw/52-node-gate.log`: `1c49964ee8274e21b62d41de5d773c8a6ddab42289ea1f119e3f95b9a5db96a8`
- `raw/52-node-gate.tsv`: `5b14e16049120ee00b7869b19161bd60c8de1488ea974deef5f1421d3ea280fb`
- `raw/41-source-binding.tsv`: `b0269b68f765aef2a9a06e55f54128f4d3a4c65a2c09c753f27cf545b6cde604`
- `raw/40-critical-failures.tsv`: `fae10f623821d8b20b4e27afea36df08076a8ca5af3661121204e8e6acda850c`
- `raw/12-sqlite-facts.tsv`: `89abbf38cb9def89c0c6688772a32bc03b3d06b2feb61356766286bc55c0c5d9`
- `raw/18-diary-after-run-manifest.tsv`: `65589f42e57e85ce4d310552f0acb83bc91ed222cfcac1918a34acff200455e1`
- `raw/31-clean-checkout.log`: `e16e1300b032333df3cb29cc166c6f7c7fdfa01455d3e949fb40906d5c5608f8`

# Commands

- Packet lint: exit 0, `PASS packet_files=160`.
- Exact Docker `gate baseline-capture`: exit 0, `ok gate baseline-capture`.
- Focused test list: exit 0, two tests.
- Focused tests: exit 0, two passed and zero filtered.
- Product-crate diff from base: exit 0, no differences.
- Packet tree diff from anchor: exit 0, no differences.
- Packet SQLite snapshot comparison: exit 0, identical.
- Diary manifest comparison: exit 0, no mismatch.
- Host sqlite3 lookup: exit 127; Python read-only SQLite fallback: exit 0.

# Verdict

PASS. The baseline-capture node is ready for a source-bound receipt. No repair
is required.
