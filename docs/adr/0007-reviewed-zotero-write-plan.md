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

The authenticated Zotero 10+ adapter is a narrow loopback transport for those injected functions. A caller may supply credentials directly or perform one official `/api/local/authorize` request with a bounded nonblank application name and expected server identity. Every authorization, read, and write response must repeat that identity before status classification. Success returns an exact 32-character header-safe key plus the remembered decision; denial requires same-server bounded JSON with `denied: true`. The private authorization wrapper can only disclose the remembered decision or be consumed into the existing adapter; neither value is debuggable or serializable. Denial and rate limiting never trigger an automatic retry or repeated prompt, and only a bounded integer retry delay is retained. Writes distinguish an expired authorization from a matching-server stale precondition, while a different-server `412` invalidates the read/write partition as a database switch. Thin public adapter boundaries delegate to the generic write and rollback cores rather than creating parallel mutation logic. Rollback evidence binds the server, post-write item revision, complete expected current metadata, and complete restoration metadata. Before its first read, rollback rejects evidence spanning server identities; before its first write, it verifies every item at one current library version. It then follows the already reversed receipt order and advances that version only from a verified write. A failed or unverifiable inverse response is re-read only as observation and always remains indeterminate. Its exact submitted request, full operation and optional observation are retained; matching metadata cannot prove completion or termination. Only untouched operations remain listed, without automatic retry authority. Public operation DTOs and empty operation slices are not complete original-write scope or independent approval; authoritative consumer wrappers must preserve those boundaries before live use. Reusing consumed evidence fails preflight before writing. Static errors and serializable receipts cannot echo a credential, response body, or URL. Cross-item transactionality is not claimed, and source records and attachments are never deleted.

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
- AC5 is implemented. AC6 now has deterministic write and rollback preflight, partial-failure reconciliation, secret-free receipts, and synthetic authenticated transport evidence. AC6 remains incomplete until approved live Zotero 10 write, partial-failure, and rollback behavior is verified.

## Alternatives considered

### Execution evidence correction (2026-09-06, Proposed)

PR #19 keeps its thin public adapter execution boundary and adopts the same
private proposal-bound plan and uncertainty semantics. Integration `bf4b2e5`
exposed the test's missing required digest. `785cdf5` uses the existing verified
plan builder and exercises both core and public wrapper on failed HTTP writes;
`5693f12` aligns the source fixture with the reviewed nonempty before-state after
the builder correctly rejected it as stale. No validator was relaxed. The test
verifier is synthetic and provides no real-world approval. A stale PRD paragraph
allowing an inverse for an observed unexpected mutation is corrected to match
the original-owner runtime and this decision. Existing delegation is retained
instead of adding a second execution path, accepting that genuine authorization,
independent approval and live recovery evidence remain separate outstanding gates.

PR #17 integration preserves the transport implementation while inheriting the
original-owner fix. Test `97cce5a`, strengthened by `29a3771`, composes authenticated
HTTP with the executor and verifies failed POST plus a fully matching observed
state remains unknown. Complete request/observation, proposal binding, single POST
and no inferred inverse are asserted. No new runtime, retry policy or authority
issuer is introduced; synthetic transport evidence does not prove live recovery.

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
- Treating mock transport coverage as live proof was rejected because no approved Zotero 10 authorization, runtime write, and rollback exercise has been performed. No live prompt ran and no key is committed.
