# Live metadata admission and visual scope audit

Observation: September 6, 2026, 10:39–10:58 UTC. This is read-only runtime and source-scope evidence, not completed classification, approved meaning or a protected release. Aggregate coordinates are in [the audit record](zotero_metadata_visual_audit.json).

## Verified live admission

The existing PR #9 owner executable was freshly built from local documentation head `f8566408e6a3017cf775fadf2a2f7e50b2d20dc6`, with runtime source `8effa6a9b15ac1a09b7e80dab4cf2885fad02211`. Binary SHA-256 was unchanged before and after execution: `a2840a829530c271cbfa750d32afeed1dc7d27b58f8da3bc84f51083cb533b65`. The complete read took 26.29 seconds and accepted Zotero 10.0.1, API 3, schema 44 and library revision 2. Every retained bibliographic revision was zero; the new item-revision guard accepted the actual input without assigning invented positive revisions.

The 2,488,762-byte proposal report is a new, single-link, owner-only regular file created under process-local `umask 077`. Its SHA-256 is `f9a041d4c1a90b1e2268a253d7cf82b61a47fd185d69a4b0d85df88a96066481`. Independent aggregate checks verified API/schema presence, unique bibliographic identities, revisions within the library revision and absent model receipts. No bibliography, item key, raw note, server identity, credential or screenshot is committed. No source record, collection, tag, decision or approval was changed. These local permissions do not establish loopback peer authentication or provider atomicity.

## Scope reconciliation

| Observed category | Records | Treatment |
| --- | ---: | --- |
| Bibliographic records | 3,715 | Existing proposal denominator; 3,658 abstentions and 57 other deterministic proposals |
| Child attachments | 3,922 | Linked evidence, not additional paper decisions |
| Child notes | 88 | Linked evidence, not additional paper decisions |
| Annotations | 597 | Separate source-record count |
| Standalone PDF attachments | 3 | Not present in the bibliographic proposals; identification/reconciliation remains open |
| Standalone note | 1 | Not present in the bibliographic proposals; evidence disposition remains open |
| Total metadata records | 8,326 | No category is silently dropped from the scope audit |

The complete 3,925-attachment sweep used 40 sequential bounded GETs and validated each page's count, item type, revision, API/schema/provider/server continuity and final unique-identity total. The complete note response contains 89 records, one without a parent. Child attachments plus child notes equal the report's 4,010 direct child references. The native application's 3,719 selected top-level records reconcile as 3,715 bibliographic records plus these four standalone records. This does not prove that each standalone PDF is a distinct paper: determine identity and provenance before linking, admitting or excluding it, and retain its original evidence. The note must not be assigned a paper label by type alone.

The attempted `/items/top` queries with `itemType=attachment` or `note` returned child records too. Their `Total-Results` values were 3,925 and 89, not standalone counts. This observation invalidated the initial header-only counting approach. Counts above come from returned parent coordinates and a full attachment traversal, not the route's name or the first 100 records. Provider cause and a portable top-level filtering contract remain unverified.

## Visual inspection

CUA screenshots of the real Zotero window showed the duplicate-items view, then the whole-library selection with **3,719 items selected** and a visible retraction warning. No merge, deletion, metadata edit or synchronization action was taken. The retracted-items accessibility view subsequently reported one record and an explicit retraction description. That one-record count is accessibility evidence, not a newly verified retraction diagnosis. Later screenshots retained the previous selection frame despite the accessibility view changing; those stale images do not prove that the retracted-items view rendered. The original duplicate-items view was restored in the accessibility state. This capture/render mismatch remains a visual-verification limitation, not a proven Zotero rendering defect.

## Required next delta

The classifier deliberately excludes attachments and notes, but its current completion denominator does not reconcile these four standalone sources. Add a source-bound pending-scope inventory and completion gate in the earliest research owner, with zero-scope/standalone/child/dangling-parent tests and complete identity coverage. Keep unresolved evidence visible; do not relabel attachments as papers, auto-merge them, shrink the denominator or invent steward labels. Preserve retraction/correction evidence separately from topical disposition and publication authority. This is a newly evidenced gap, not an implemented capability.

The existing full-text capture and approved-review requirements remain intact; this metadata report cannot replace or renew an older capture or approval. A released contextual-orchestrator contract remains required before model-assisted proposals. Last independently verified decisions/approvals remain zero for the existing 3,715-item worksheet, and the additional scope reconciliation is unfinished. GitHub push, forward propagation, exact-head hosted checks and independent review remain pending behind the recorded API cooldown.
