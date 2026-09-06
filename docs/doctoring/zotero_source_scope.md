# Zotero complete metadata inventory and pending source scope

Status: verified producer repair in the existing PR #9 owner lane. Dependent report admission, governance binding and whole-library completion remain incomplete. No approval, protected merge, model call or Zotero mutation is claimed.

## Observed failure and source contract

The [actual September 6 native/API inspection](https://github.com/ContextualWisdomLab/ConceptWeave/blob/55b1d91dcd299145239a62062ed504fcb6e7bfd1/docs/doctoring/zotero_metadata_visual_audit.md) found 3,719 selected top-level records but only 3,715 bibliographic proposals. Complete attachment/note response audits identified three standalone PDFs and one note outside that worksheet. They are not proven to be three distinct additional papers. Their identities, contents and relationships must not be guessed, discarded or merged.

Zotero documents all-item and top-level endpoints separately and supports both standalone and child attachments (Zotero, n.d.-a, n.d.-b). Actual Zotero 10.0.1 responses to the filtered `/items/top` diagnostic included children; endpoint naming and header totals therefore did not prove standalone scope. DeepWiki's repository answer described intended `noChildren` behavior but did not explain the observed filtered response. That contradiction remains a provider investigation, not a reason to override actual records or assume a fixed upstream bug. This implementation uses the existing complete `/items` reader, not a new filtered adapter. Context7's previously exhausted allowance was not bypassed; official primary documentation and actual responses support this increment.

## Root cause and minimal repair

`read_local_snapshot` → `read_snapshot_with` → `read_snapshot_with_clock` admits the complete bounded metadata read, then calls `classify_snapshot`; the CLI serializes that report. Previously `is_bibliographic` excluded notes, attachments, annotations and records with parents. A scalar total and direct child keys survived, but the other records' metadata and unresolved ancestry disappeared. Matching all 3,715 worksheet slots could therefore be mistaken for accounting for the whole library.

The producer now moves every excluded record into `unclassified_items`, reusing `ZoteroItem`. Sorted bibliographic proposals plus sorted inventory account for every reader-admitted identity. The existing parent-to-children index is consumed iteratively starting at each bibliographic root. Each adjacency list is removed once; records not reached remain in sorted `pending_source_item_keys`. This retains standalone roots and their descendants, missing-parent trees, disconnected cycles and self-cycles without recursion, another transport or a dependency. Time is O(n log n), auxiliary memory O(n); the private report grows because records previously dropped are now retained.

No pending record receives a paper disposition or enters bibliographic DOI/title duplicate candidates. Empty pending keys mean only that the observed parent graph reaches bibliographic roots. They do not establish correct semantics, genuine review, independent approval, complete full-text capture, atomicity or write permission. `ItemData` is a metadata projection: note HTML, PDF bytes, attachment-specific fields and unknown provider fields are not preserved by this serialization. Keep original evidence/captures separately and do not advertise a lossless source backup.

Independent review also found blank source keys admitted by the reader. The existing shared page predicate now rejects an empty/whitespace-only key before accumulation, alongside future item revisions. It does not trim or rewrite valid keys, impose a new key-format regex or change the infallible offline classifier's existing contract. Duplicate keys remain rejected after the complete read; arbitrary offline input is not certified merely because classification terminates.

## Committed experiments and verification

- Baseline `f8566408e6a3017cf775fadf2a2f7e50b2d20dc6`: 43 tests / 10 unfiltered suites, including two doctests.
- Inventory RED `48dcd0d`: all three new tests fail on absent pending inventory, not compilation. They cover standalone metadata, nested children, deterministic ordering, orphan/cycle/self-cycle paths, empty input and fully linked evidence.
- Inventory GREEN `220f697`, then adjacency-consumption simplification `48c3525b0061dfba7552f1648f5ad5028b653ab8`: 46 / 10 tests pass. An independent agent reran the three focused tests on the latter exact source and reported no blocking producer finding on reader-admitted input.
- Identity RED `f7a67bf` detects a third request before rejection; its fixture accidentally also repeated a valid key. Refined committed RED `3a57f3b` uses unique remaining keys and fails the actual expected rejection assertion, isolating the missing-identity defect.
- Final source `1e95d6eb979e66ecb7dae4f81f18a6b0a91b7624`: **47 tests / 10 unfiltered suites**, including two doctests; strict Clippy, warnings-denied rustdoc and release build pass. The identity test covers empty/space/control-whitespace on first and subsequent pages with no next request after rejection.
- The unchanged coverage gate passes **123/123 functions, 751/751 normalized source regions and 114/114 normalized branches**. Raw LLVM is not 100%: 1,231/1,232 lines, 1,985/1,993 regions and 113/114 branches. No threshold, exclusion or dependency was changed. Coverage and final verification logs are `/tmp/conceptweave-source-scope-final-{tests,clippy,rustdoc,release,coverage}-20260906.log`.

## Genuine library replay, distinct from unit evidence

The release executable built from `48c3525` completed a read-only actual Local API run in **17.61 seconds**. Its SHA-256 was `04e12299babb63c58dd32736373cca8c5a72a9f02aa20ed5366f01e55b935ee7`. This run precedes the later blank-key guard; it is not a run of the final `1e95d6e` executable.

The private new report `/private/tmp/conceptweave-source-scope-live-C98C52A4-1D9A-4528-BFE8-6A2EBD1AAE98.json` is a single-link regular `0600` file, 3,731,468 bytes, SHA-256 `7a6cd9f7f90a052964f60b5152cd12dc5928d7187ad41e808a0c215bab3b3b97`. It retains **8,326 = 3,715 proposals + 4,611 nonbibliographic records** and exactly **four pending keys**. The prior private attachment/note audit's four identities match the pending list exactly. All 8,326 combined identities are unique and nonblank; all retained nonbibliographic revisions are within library revision 2. The previous report's entire bibliographic proposal list, server identifier and library revision compare equal. Zotero 10.0.1/API 3/schema 44 are unchanged. Only aggregate checks are published; no original titles, keys, server identity, note body or screenshot is committed.

This is one elapsed-time observation, not a controlled performance improvement claim. No full text was recaptured, no old capture was rewritten, and no bibliographic proposal was promoted to an authentic decision. Existing worksheet decisions and independently approved labels remain 0/3,715, with four unresolved sources additionally visible.

## Visual Inspection and adoption gates

This turn attempted native Zotero inspection; the computer-use tool reported that the Mac was locked and automatic unlock failed. No lock bypass, fresh screenshot or new visual pass is claimed. The previous actual screenshot of 3,719 selected items and a retraction warning remains historical evidence; the previous one-item retracted view was accessibility-only because later frames were stale. Unlocking is required for another current visual inspection. API and unit success do not substitute for it.

The root consumer audit at `22a29c1cfc0918fa34287f3bffe7f400e97f4a0f` identifies the next required integration:

1. Require both inventory fields on report restoration; missing/null legacy inventory must not become empty. Validate uniqueness, disjointness, exact snapshot complement, original key/version/parent/type evidence and recompute pending ancestry. `SnapshotItemRevision` currently lacks item type.
2. Use shared admission in `build_steward_review_worksheet`, plus direct `prepare_reviewed_golden_set`, `build_duplicate_merge_review_manifest` and `prepare_classification_write_plan`, before any external verifier. These latter routes currently bypass worksheet admission.
3. Keep `StewardReviewProgress.complete`'s bibliographic meaning separate from full-library scope status. Pending-source reconciliation needs bound evidence and its own explicit status; do not mark children as additional unreviewed papers or turn an empty pending list into approval.
4. `classification_proposal_digest` v1 binds only classified proposals, not this inventory. Full-text captures bind the whole report and must reject incompatible restored reports. Never rewrite a prior capture, default scope away or backfill approval identity to make old receipts pass.
5. Preserve parent/child history while normally forwarding the new item-revision, inventory and identity delta through existing owners. This producer is not already present in the root #39 runtime. Run each changed consumer's full tests and current-head hosted checks/reviews; prerequisites, protected rules and independent approval still apply.

## References

Zotero. (n.d.-a). *Zotero Web API documentation*. Retrieved September 6, 2026, from https://www.zotero.org/support/dev/web_api/v3/basics

Zotero. (n.d.-b). *Adding files to your Zotero library*. Retrieved September 6, 2026, from https://www.zotero.org/support/attaching_files
