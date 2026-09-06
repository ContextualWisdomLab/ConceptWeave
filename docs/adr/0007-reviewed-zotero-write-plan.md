# ADR 0007: Separate reviewed Zotero write planning from execution

- Status: Proposed
- Date: 2026-09-04
- Supersedes: the future-write deferral in ADR 0006; its read-only Zotero 9 decision remains valid

## Context

The installed-version and runtime-availability statements in this original context
and its original alternatives describe 2026-09-04, not current host state. Later
local evidence records Zotero 10.0.1; that alone establishes no write approval.

Issue #8 requires classification changes to default to dry-run, preserve complete collection and tag state, reject stale review input, and make rollback reconstructable. The installed Zotero 9.0.6 Local API cannot write. Zotero 10+ writes additionally require a runtime-granted key, the same server identity, and fresh library/item versions. A planner can establish the review and recovery contract now without inventing authority or adding an unsafe Zotero 9 mutation path.

## Decision

ConceptWeave builds a local-only `ClassificationWritePlan` from an externally verified complete review set. Dry-run is the default. The review must match the exact Zotero version, server identity, library version, classifier revision, raw-snapshot digest, complete item-key/item-version coordinates, and observed collection/tag state. The plan retains the reviewed Zotero version used for execute eligibility, while private fields and read-only accessors prevent external callers from mutating validated execution state. Every receipt copies the plan's review and snapshot coordinates so an outcome cannot be detached from its authority or evidence. It rejects unknown or duplicate items, detached item revisions, blank or duplicate metadata, unsupported tag types, no-op changes, and `NeedsStewardReview` as a write decision. Operations are deterministic and retain complete before, after, and rollback states. Manual tag markers `None` and `0` are canonicalized to `None`; automatic tag type `1` is preserved.

Execute planning fails closed for Zotero versions below 10. The plan contains no API key and performs no network call. Dry-run enumerates every operation as not attempted. The execution core accepts caller-owned preflight and write functions, preflights the complete plan before the first mutation, and verifies server, library, item revision, collection, and typed-tag responses. After a failed or invalid response, a follow-up read is observation only: matching before-state cannot prove a delayed request terminated, and matching after-state or a newer revision cannot prove which writer caused it. The receipt keeps the exact submitted request and optional observation, always names that item as indeterminate, and creates no inverse for that unconfirmed write. Earlier directly verified applied items and their inverse coordinates remain intact. The API key remains adapter-owned. Cross-item transactionality is not claimed, and source records and attachments are never deleted.

## Consequences

### Source-scope amendment (2026-09-06, Proposed)

In the context of reviewed collection/tag replacement, facing a report whose
supporting title or inventory can change without changing its copied raw digest,
we decided for shared report admission and a required v2 proposal binding, and
against trusting only copied snapshot coordinates or duplicating inventory
validation, to reject changed evidence before consuming approval, accepting that
old receipts need fresh review and independent reissuance.

Regression `dcde49e` demonstrated two failures: changed titles and inconsistent
observed counts still returned plans. Repair `c348278` checks the shared inventory,
audit and pending-source invariants, then compares the proposal digest, before
calling authority. Existing item/metadata error precedence remains unchanged.
The digest covers proposals, projected unclassified metadata and pending keys;
it does not claim full-text capture or duplicate-candidate authority. Duplicate
membership remains bound by the separate duplicate review contract.

Governance must authenticate the complete reviewed set, including the binding
and requested changes. Test extension `1fe3d7d` exercises omitted/blank bindings,
retained plan identity, and a recomputed binding rejected by the original receipt.
No issuer, API write, automatic legacy backfill, or approval bypass is added.
Mode is a caller-supplied planning argument, not a field authenticated by this
reviewed set; Execute planning therefore supplies no execution authority.
This amendment remains Proposed until protected integration; later consumers must
adopt the required field without deriving fresh authority from serialized plans.

- Review and rollback semantics can be tested on Zotero 9 without changing the library.
- Exact before-state checks prevent silent loss of unrelated collections or automatic-tag metadata.
- AC5 is implemented and AC6 now has deterministic preflight, partial-failure, and rollback-receipt semantics. AC6 remains incomplete until an authenticated Zotero 10+ adapter and approved live write/rollback are verified.

## Alternatives considered

### Execution evidence correction (2026-09-06, Proposed)

In the context of uncertain write responses, facing concurrent edits and delayed
requests that can produce indistinguishable observations, we decided for preserving
uncertainty and the exact submitted request, and against inferring completion or
conditional rollback from post-read metadata, to prevent overwriting an unrelated
edit or repeating a still-running request, accepting that recovery must wait for
independent causal evidence instead of automatic retry.

Independent source review found the same unsafe inference in the later executor;
fixing only full-text wrappers cannot restore uncertainty already discarded by
this owner. RED `646a10c` retained existing lost-response/unchanged-state scenarios
but corrected their unsafe assertions. Runtime `c09d101` removes those inference
branches; `e169630` checks the complete submitted request and absent observations.
The observed metadata remains available for investigation, but carries no retry
or rollback authority. A serialized mutable receipt is audit data and cannot
construct the private executable plan. All outcome receipts retain the verified
proposal/source binding (`b91ad9f`, RED `e8b4c06`). Later recovery consumers must
preserve these fields and refuse an unknown original write, including when the
list of previously verified inverse operations is empty.

- Writing through Zotero 9 was rejected because the provider does not support it.
- Storing only collection/tag deltas was rejected because Zotero array updates are complete replacements and cannot prove lossless rollback.
- Adding the HTTP writer now was rejected because no Zotero 10+ runtime or approved local key is available for end-to-end verification.
