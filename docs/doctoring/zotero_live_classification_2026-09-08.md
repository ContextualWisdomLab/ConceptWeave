# Live Zotero classification snapshot (2026-09-08)

## Fresh replay checkpoint (21:09 KST)

The release-mode read-only replay was rerun without mutation. It again observed
8,326 records, 3,715 bibliographic proposals, 4,611 nonbibliographic records,
four pending source keys, 3,658 abstentions, 49 duplicate candidates and zero
failures. The fresh private `0600` report is
`/tmp/conceptweave-zotero-live-20260908-210911.json` with SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
Equal aggregates do not grant review, approval or Zotero write authority.

The existing Rust CLI completed a release-mode, read-only Local API run against
the running Zotero instance and wrote the report to a private `/tmp` file with
mode `0600`. The report is not committed or published.

- Report file SHA-256: `8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`
- File size: 8,132,756 bytes
- Zotero: `10.0.1`; API: `3`; schema: `44`
- Snapshot items: `8,326`
- Classified proposals: `3,715`
- Unclassified metadata records: `4,611`
- Pending source-resolution keys: `4`
- Library version: `2`
- Rule revision: `ontology-research-v2`
- Server identity: present (opaque value withheld)

These counts establish a fresh local proposal snapshot only. They do not issue
steward decisions, approvals or Zotero writes. Any full-text review must use
the existing capture-bound private evidence path and retain the four pending
sources rather than clearing them.

## Fresh replay checkpoint (22:13 KST)

The same release-mode read-only command was rerun against the live Local API.
The new private report `/tmp/conceptweave-zotero-live-20260908-221326.json`
is `0600`, 8,132,756 bytes, and has the same SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
Observed totals remain 8,326 records, 3,715 proposals, 4,611 unclassified
records, four pending source keys, 49 duplicate candidates and zero failures.
This repeatability is evidence of a stable observation, not steward review,
approval, semantic publication or write authority.

## Fresh replay checkpoint (2026-09-09 00:24 KST)

The release-mode read-only Local API replay completed without mutation at
`/tmp/conceptweave-zotero-live-20260909-002410.json`. The owner-only file is
`0600`, 8,132,756 bytes, and retains SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
Aggregate totals remain 8,326 observed records, 3,715 classified proposals,
4,611 unclassified records, four pending sources, 49 duplicate candidates and
zero failures. Title-only discovery counts were 27 `ontology`, 23 `OWL` and
20 `RDF` matches; they overlap and remain discovery evidence only. No review,
approval or Zotero write occurred.

## Fresh replay checkpoint (2026-09-09 00:47 KST)

The release-mode read-only Local API replay completed again at
`/tmp/conceptweave-zotero-live-20260909-004718.json`. The owner-only file is
`0600`, 8,132,756 bytes, and has the same SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
Aggregate totals remain 8,326 observed records, 3,715 classified proposals,
4,611 unclassified records, four pending sources, 49 duplicate candidates and
zero failures. Title-only discovery counts remain 27 `ontology`, 23 `OWL` and
20 `RDF` matches under the established matching rules; they overlap and remain
discovery evidence only. No review, approval or Zotero write occurred.

## Fresh replay checkpoint (2026-09-09 01:17 KST)

The release-mode read-only Local API replay completed again at
`/tmp/conceptweave-zotero-live-20260909-011756.json`. The owner-only file is
`0600`, 8,132,756 bytes, and retains SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
Aggregate totals remain 8,326 observed records, 3,715 classified proposals,
4,611 unclassified records, four pending sources, 49 duplicate candidates and
zero failures. Case-insensitive title signals are ontology 27, semantic 36,
taxonomy 2, knowledge graph 1 and vocabulary stem 81; exact uppercase `OWL`
and `RDF` counts are 23 and 20. These are discovery/repeatability metrics only;
no review, approval or Zotero write occurred.

## Fresh replay checkpoint (2026-09-09 latest local verification)

The pinned release executable was rerun against the live loopback Local API in
read-only mode. It wrote `/tmp/conceptweave-zotero-live-20260909-latest.json`
with mode `0600` and size 8,132,756 bytes; SHA-256 remains
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`.
The report observed 8,326 records, classified 3,715 proposals, retained 4,611
unclassified records, four pending source keys and 49 duplicate candidates.
The bound contract is Zotero 10.0.1, Local API 3, schema 44 and library version
2. This is repeatable local proposal evidence only; no review, approval or
Zotero write occurred.

The discovery KPI was independently recomputed from the latest report using only
the 3,715 classified proposal titles, not the 4,611 retained nonbibliographic
records. It yields 27 case-insensitive `ontology`, 36 `semantic`, 2 `taxonomy`,
1 `knowledge graph`, 81 vocabulary-stem, and exact-uppercase `OWL`/`RDF` counts
of 23/20. Applying the same predicates to all 8,326 titles yields 30/36/2/2/81/26/23,
which is intentionally not the research KPI. The denominator boundary is part of
the evidence contract; these signals remain discovery queues, not decisions.

The same report's four pending source keys resolve to one standalone note and
three standalone attachment records, each without a parent link. The attachment
records retain only the metadata projection in this report; their file bytes,
content type and link mode remain outside the classification artifact and must
be inspected through the separate capture-bound evidence path. No pending source
was cleared or converted into a bibliographic proposal.
