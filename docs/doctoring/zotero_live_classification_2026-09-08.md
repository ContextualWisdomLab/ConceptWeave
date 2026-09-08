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

## Fresh replay checkpoint (2026-09-09 02:31 KST)

Another pinned release replay completed read-only against the Local API. The
0600 report was 8,132,756 bytes with the same SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc` and the
same aggregate: 8,326 observed, 3,715 classified proposals, 4,611 retained
nonbibliographic records, four pending sources and 49 duplicate candidates.
This is repeatability evidence only; it does not establish approval, full-text
review, Zotero mutation or immutable release.

## Fresh replay checkpoint (2026-09-09 03:04 KST)

The optimized release binary completed another read-only Local API replay. The
owner-only report was `0600`, 8,132,756 bytes, and retained the same SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`: 8,326
observed records, 3,715 classified proposals, 4,611 retained records, four
pending sources and 49 duplicate candidates. This confirms repeatability only;
it does not clear pending sources or grant review, write or release authority.

## Capture-bound availability checkpoint (2026-09-09 03:08 KST)

Using the same report, the optimized release binary completed the read-only
`--capture-full-text` path and then `--full-text-availability`. The capture is
owner-only (`0600`, 235,602,798 bytes); the separate availability summary is
also `0600` (359 bytes) with capture digest
`sha256:106ca65ab62151f303c5807ecceb271c0bec953f091426fb1e74ec808913f9b8`.
Across 3,715 bibliographic papers, 3,203 have nonempty text, 440 have no
attachment, 34 have an unmanifested attachment, 38 have captured-but-empty
text, and 471 need review without text. Two nonempty records are unbound.
These are acquisition and review-workload measures only; no paper decision,
approval, pending-source resolution or Zotero write follows.

## Blank worksheet and bound review view checkpoint (2026-09-09 03:12 KST)

The same private report/capture produced a `0600` full-text worksheet of
1,776,452 bytes (SHA-256
`87a35820097e6b858e47f251c23f8b64dd7eb6941a55fbcdcada4296c4124af6`). It has
3,715 blank decision slots, preserves 8,326 snapshot items and carries proposal
digest `sha256:32cb83fd8a66b3fea50831f952c254e96e20b1b8da78d18acad465ac2c4dff0d`.
The bound full-text review view then emitted a separate `0600`, 1,590,859-byte
artifact (SHA-256
`5ec82fa82017c83bc281134a2d96f6a45040f432829a9c61124a9f639ea3460e`) with a
25-item blank review batch and the same capture/report/proposal bindings. These
artifacts are prepared for steward review only; no decision or approval was
issued.
