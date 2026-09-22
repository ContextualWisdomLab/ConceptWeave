# Zotero metadata item revision admission

Status: locally verified prerequisite repair for PR #9; not pushed, protected, released or propagated to descendants at this checkpoint. No actual library, paper, model or Zotero mutation was exercised. Tests use synthetic unit inputs only.

## Finding and provider contract

[PR #9's review](https://github.com/ContextualWisdomLab/ConceptWeave/pull/9#discussion_r3934542708) identifies an item revision higher than its response's `Last-Modified-Version` being accepted into classification provenance. Baseline `bb2faccfda9efed55b6759f1bbf7907bf6ec0c3b` checks page-header agreement, counts, bytes, elapsed time and unique keys, but never compares a returned object's revision with its containing library revision. A later review or mutation precondition could therefore inherit impossible metadata coordinates even though the pages agree with each other.

The Web API documents library revisions and object revisions separately: a multi-object items response carries the library revision; changed objects receive the library's updated revision. Revision numbers are opaque, monotonic and need not be consecutive (Zotero, 2022). The Local API documentation distinguishes Zotero 10's per-library local transaction revisions from Zotero 9's synced versions. Never-synced earlier objects can be version zero; Zotero 10 local revisions are comparable only within one instance and are unrelated to Web API revisions (Zotero, 2026). Together these rules support the within-response metadata invariant `item.version <= page.library_version`; they do not justify cross-instance ordering, rejecting zero, or applying this condition to full-text endpoints.

## Decision and failure boundary

The call path is `read_local_snapshot` → `read_snapshot_with` → `read_snapshot_with_clock` → page transport → `classify_snapshot`, followed by CLI report-file creation only after success. The shared reader now checks every page member after the existing resource validation and before `items.extend`. One `.any(...)` predicate returns the existing `ReadError::SnapshotChanged` if a member is too new. Bibliographic records and child attachments, notes and annotations take the same path. No later page is requested after rejection and no partial report is returned.

Filtering offending records would change the denominator; clamping revisions would fabricate provenance. Changing the pure offline classifier would broaden its contract and still be later than the provider admission boundary. No new error type, helper, public signature, dependency or cross-product responsibility is needed. The added pass is linear in each bounded page and does not allocate another collection. The downside is that an inconsistent page invalidates the whole read, even if earlier pages were usable. Equal headers and valid member revisions still do not prove atomicity, server authentication or semantic correctness. ADR 0006 remains Proposed and no approval or write authority changes.

## Executed checks

- Baseline `bb2facc`: 41 tests / 10 unfiltered suites, including two doctests.
- Committed RED `1cf3472499ad49716a70603be2e15dc857819231`: the new rejection test fails at the expected `SnapshotChanged` assertion, not compilation or an unrelated error. Its valid-version control passes before the guard exists.
- Source GREEN `8effa6a9b15ac1a09b7e80dab4cf2885fad02211`: 43 tests / 10 unfiltered suites, including two doctests, under explicit Rust 1.98.0. Rejection covers first and subsequent pages and four item types, with a valid member before the offending member. Acceptance covers zero, lower, equal and `u64::MAX` revisions under Zotero 9 and 10 labels and checks retained revisions.
- All-target warnings-denied Clippy, warnings-denied rustdoc, release build, formatting, existing CI contract and diff validation pass. CodeGraph is healthy at 11 files, 206 nodes and 433 edges.
- Unchanged coverage gate passes: 113/113 functions, 709/709 normalized source regions and 106/106 normalized branches. Raw LLVM is not 100%: 1,097/1,098 lines, 1,690/1,697 regions and 105/106 branches. No gate, exclusion or fixture denominator was weakened.

Reproduce with `cargo +1.98.0 test --workspace --locked` and `bash scripts/check_coverage.sh`. Local logs are `/tmp/conceptweave-pr9-item-revision-{baseline,red,green,clippy,rustdoc,release,coverage}-20260906.log`.

## Integration gate

The previous GitHub PR audit was rejected by the account GraphQL quota at 2026-09-06 10:05:51 UTC. This repair uses already-read review evidence and local source, not a new remote-state claim. Do not retry that audit before 11:06:12 UTC or substitute another endpoint/token/provider to bypass the limit. Then perform one normal fresh head/base/state/writer check, preserve concurrent delta through normal history, push the owner repair and merge it forward through each dependent research PR with fresh exact-head verification. This new propagation is outstanding; the earlier elapsed-deadline propagation does not include this revision guard. Required hosted checks, current-head independent review and protected integration remain separate gates. Do not close predecessors or claim actual paper decisions from these tests.

## References

Zotero. (2022, August 14). *Zotero Web API v3: Syncing*. https://www.zotero.org/support/dev/web_api/v3/syncing

Zotero. (2026, July 29). *Zotero Local API*. https://www.zotero.org/support/dev/web_api/v3/local_api
