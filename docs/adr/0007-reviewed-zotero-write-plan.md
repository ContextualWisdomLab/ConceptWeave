# ADR 0007: Separate reviewed Zotero write planning from execution

- Status: Proposed
- Date: 2026-09-04
- Supersedes: the future-write deferral in ADR 0006; its read-only Zotero 9 decision remains valid

## Context

Issue #8 requires classification changes to default to dry-run, preserve complete collection and tag state, reject stale review input, and make rollback reconstructable. At initial planning the installed Zotero 9.0.6 Local API could not write; the later Zotero 10.0.1 audit is recorded separately in the current Gap baseline. Zotero 10+ writes additionally require a runtime-granted key, the same server identity, and fresh library/item versions. A planner establishes the review and recovery contract without inventing authority or adding an unsafe Zotero 9 mutation path.

The installed-version and runtime-availability statements in the original alternatives describe 2026-09-04, not current host state. Later local evidence records Zotero 10.0.1; that alone establishes no write approval.

## Decision

### Complete-source and recovery-envelope integration (2026-09-07, Proposed)

PR39 normal merge `4e856c5` retains its typed full-text write scope and PR38's
complete inventory validation. An extracted preparation helper had bypassed the
pending-source completion gate: a completed paper worksheet could reach meaning
approval while standalone evidence remained unresolved. The inherited mixed-source
test fails at that boundary. The guard belongs in shared full-text preparation,
which both evaluation and write admission call, not only in the public evaluator.
Write preparation also retains the parent's proposal binding and inventory checks
before either real authority callback. Starting and locally finalizing review
remain distinct from complete admission; no paper or pending source is dropped.

The delayed rollback wrapper had copied an operation tail and binding but omitted
the preceding receipt's verified outcomes, exact failed request and observation.
Committed test `75b9de7` fails because that preceding envelope is absent. Repair
`9fcc8bb` borrows the opaque prior receipt using the existing original-observation
pattern, while preserving existing serialized binding, observation and tail fields.
An observation cannot outlive its source receipt or deserialize into executable
authority. The executor reuses the core's exact failed request instead of tracking
the last request a second time. Follow-up tests include an earlier verified inverse
before a later failure, as well as matching, changed and unavailable observations.

An operation-only reconstruction was rejected because it loses prior evidence;
duplicating the whole receipt into a new owned authority type was unnecessary for
this in-process read-only view. The borrowed lifetime requires callers to retain
the earlier receipt; durable restart admission remains separate unfinished work.
Matching metadata never proves causal completion or permits a retry. These local
changes supply neither independent governance approval nor live-write permission.

Independent follow-up traced every public reconciliation constructor: each emits
indeterminate status, so the wrapper's success branch was unreachable. Preserve
the public function signature but reject every observation-only retry before I/O;
remove dead execution code instead of fabricating resolved private receipts in
tests. A future successful retry requires a separately designed and independently
verified causal-resolution contract, not another metadata observation.

ConceptWeave builds a local-only `ClassificationWritePlan` from an externally verified complete review set. Dry-run is the default. The review must match the exact Zotero version, server identity, library version, classifier revision, raw-snapshot digest, complete item-key/item-version coordinates, and observed collection/tag state. The plan retains the reviewed Zotero version used for execute eligibility, while private fields and read-only accessors prevent external callers from mutating validated execution state. Write-execution receipts copy the plan's review and snapshot coordinates. Legacy rollback receipts retain conditional restoration evidence but do not carry those review/authority coordinates; they must not be advertised as full-text-bound approval evidence. It rejects unknown or duplicate items, detached item revisions, blank or duplicate metadata, unsupported tag types, no-op changes, and `NeedsStewardReview` as a write decision. Operations are deterministic and retain complete before, after, and rollback states. Manual tag markers `None` and `0` are canonicalized to `None`; automatic tag type `1` is preserved.

Execute planning fails closed for Zotero versions below 10. The plan contains no API key and performs no network call. Dry-run enumerates every operation as not attempted. The execution core accepts caller-owned preflight and write functions, preflights the complete plan before the first mutation, and verifies server, library, item revision, collection, and typed-tag responses. After a failed or invalid response, a follow-up read is observation only: matching before-state cannot prove a delayed request terminated, and matching after-state or a newer revision cannot prove which writer caused it. The receipt keeps the exact submitted request and optional observation, always names that item as indeterminate, and creates no inverse for that unconfirmed write. Earlier directly verified applied items and their inverse coordinates remain intact. The API key remains adapter-owned. Cross-item transactionality is not claimed, and source records and attachments are never deleted.

The authenticated Zotero 10+ adapter is a narrow loopback transport for those injected functions. A caller may supply credentials directly or perform one official `/api/local/authorize` request with a bounded nonblank application name and expected server identity. Every authorization, read, and write response must repeat that identity before status classification. Success returns an exact 32-character header-safe key plus the remembered decision; denial requires same-server bounded JSON with `denied: true`. The private authorization wrapper can only disclose the remembered decision or be consumed into the existing adapter; neither value is debuggable or serializable. Denial and rate limiting never trigger an automatic retry or repeated prompt, and only a bounded integer retry delay is retained. Writes distinguish an expired authorization from a matching-server stale precondition, while a different-server `412` invalidates the read/write partition as a database switch. Thin public adapter boundaries delegate to the generic write and rollback cores rather than creating parallel mutation logic. Rollback evidence binds the server, post-write item revision, complete expected current metadata, and complete restoration metadata. Before its first read, rollback rejects evidence spanning server identities; before its first write, it verifies every item at one current library version. It then follows the already reversed receipt order and advances that version only from a verified write. A failed or unverifiable inverse response is re-read only as observation and always remains indeterminate. Its exact submitted request, full operation and optional observation are retained; matching metadata cannot prove completion or termination. Only untouched operations remain listed, without automatic retry authority. Public operation DTOs and empty operation slices are not complete original-write scope or independent approval; authoritative consumer wrappers must preserve those boundaries before live use. Reusing consumed evidence fails preflight before writing. Static errors and serializable receipts cannot echo a credential, response body, or URL. Cross-item transactionality is not claimed, and source records and attachments are never deleted.

PR #21 retains validated delayed reads without writes and complete observed metadata. Metadata-only restored/unchanged and retry inference is removed in `f9c2c03`: the observer always retains indeterminate causal status and emits no retry operation. Eight metadata scenarios and the three-GET adapter fixture remain covered; the latter compares the entire observed state. Legacy enum variants and the optional retry field remain for contract compatibility, not as outputs or approval from this observer. Authoritative successor wrappers still must retain the complete prior rollback receipt, its exact submitted request, binding and untouched tail; an operation-only observation cannot replace that envelope or establish causal completion, termination, or independent retry authority.

## Local validation before external approval (2026-09-05)

The original planner introduced at `53b1d4dd046727d345fa2032d9426b2ba697b9df` in [PR #13](https://github.com/ContextualWisdomLab/ConceptWeave/pull/13) called the external verifier after matching top-level review coordinates but before checking execute eligibility, report membership and each operation's revisions and metadata. A caller may redeem a one-use approval inside that verifier. For example, a valid first change followed by a stale second change could consume approval and still return no plan. Repeating the request would then lose an otherwise usable approval without any Zotero write. Checking only the newer full-text caller would leave every existing planner caller exposed.

Regression commit `505e111c993d8269e5b7b9e17a25a5ce20f8606e` preserves the failure at the original owner: four new negative test groups failed because the verifier ran once instead of zero times; the valid-input control passed. Repair `8a684882005085d8b3cb47812e185975084e0475` moves the existing verifier block after operation validation and sorting. It adds no alternate planner or dependency. Twenty-two invalid-input scenarios, each with accepting and denying verifiers, require zero calls; four valid dry-run/execute and accept/deny controls require exactly one call with the unchanged complete review. Local validation errors now intentionally take precedence over `UnverifiedApproval`. A valid rejected approval still returns that error, and successful plans retain deterministic operations and complete rollback metadata.

The exact repaired #13 head passed 72 tests across 18 unfiltered suites, including two doctests, strict Clippy, formatting, warnings-denied rustdoc and the CI contract. Its unchanged coverage gate passed 143/143 functions, 1,357/1,357 source-normalized regions and 244/244 normalized branch outcomes. Raw LLVM remained 1,508/1,530 lines, 2,126/2,161 regions and 206/244 branches. These are local source measurements, not protected acceptance. The [gap baseline](../product-technical-gap-baseline.md) records normal parent-to-child propagation without discarding predecessor commits.

Rejected alternatives were per-caller guards, which duplicate validation and miss sibling callers, and accepting invalid input before approval as harmless, which ignores caller-owned receipt consumption. The remaining limit is explicit: this planner validates its existing metadata-write contract, not the complete full-text review envelope, authority revocation, or live execution.

## Full-text write admission contract

A full-text-reviewed golden set and its aggregate evaluation are not authority to replace Zotero collections or tags. Admission must combine the complete capture-bound golden set, a separately approved explicit write set, and the requested mode in a required, non-flattened input. Every changed item's disposition must match its approved golden label. All local capture/report/proposal/full-denominator and write-state checks must finish before either external authority verifier runs. Reuse the existing full-text evaluator and repaired planner; do not insert a permissive verifier bridge, derive destinations from disposition names, backfill receipts or convert aggregate evaluation into approval.

The returned opaque, serialize-only plan must retain a versioned binding for the complete labels, capture/proposal coordinates, approvals, destinations and mode. Write execution, partial failure, rollback, retry and delayed reconciliation must preserve that same binding. No executable legacy-plan downcast or freely mixed rollback operations may detach it. Existing legacy write DTOs accept unknown nested JSON fields, so strict outer deserialization alone is insufficient. Begin with typed-only admission, or separately document and test an intentional owned-DTO compatibility change before claiming strict persisted JSON admission.

The failure analysis must cover relabeling, destination/mode substitution under old authority, denial by either verifier, missing full-denominator labels, stale preflight, mixed receipts and indeterminate outcomes. Dry-run must make no reads or writes. Paper text and authority secrets must stay out of errors and receipts. This is a bounded extension of ConceptWeave's existing intake context, not a new Utility Repository, transport, approval issuer or live-write CLI. Published owner contracts, authentic decisions, independently verified authority and approved live write/rollback evidence remain separate prerequisites.

## Local typed implementation (2026-09-06; still Proposed)

In the context of applying complete full-text-reviewed research classifications, facing loss of capture provenance and destination authority at the legacy planner boundary, we decided for private validation preparation followed by two real whole-scope verifiers and opaque bound recovery, and against allow-all verifier bridges, metadata downcasts and a second executor, to achieve exact approved-input continuity across local write attempts, accepting typed-only admission and unresolved durable-recovery and original-write-reconciliation gaps.

The owning Research Intake library now requires `FullTextWriteScope`. It includes every full-text golden label, capture-bound approval input, complete reviewed metadata changes and mode. The existing golden and write validators were moved into private preparation functions; their public legacy entry points still invoke their original verifiers after local validation. New admission runs both preparation paths and checks each changed disposition against its golden label before invoking either callback. An invalid later write cannot redeem a valid earlier approval. A meaning denial stops before write verification; an accepted meaning review does not itself authorize a destination. The two external verifiers are not a distributed atomic redemption protocol: a locally valid request may consume meaning verification before the independent write verifier denies it. Revocation, expiration and issuer policy belong to external governance.

The admitted plan retains the complete typed scope. The versioned binding hashes the compact serde JSON tuple `("conceptweave-full-text-write-v1", scope)` with SHA-256 and separately retains capture/proposal/snapshot coordinates and mode. Exact array order and receipt inputs are part of this identity; this is not a cross-language canonical-JSON claim. Every bound outcome carries the same commitment. Write receipt serialization omits the legacy review ID and authority input rather than exposing them; original owner-only scope must be retained to verify the commitment. The plan has no public legacy-plan projection; recovery accepts an opaque receipt, never caller-assembled operation slices.

Execution delegates to the existing complete-preflight and conditional replacement core. Known applied work can be rolled back from its one bound receipt. Unknown original-write state, dry-run or empty inverse work is rejected before reads or writes, avoiding a false restored outcome. A rollback failure retains known pending work; its retry refuses to ignore an indeterminate operation. Delayed rollback reconciliation observes that operation once and keeps the untouched tail. An unchanged observation retries that operation plus the tail; a restored observation skips its already restored operation; an indeterminate observation cannot write. Subsequent attempts still run complete preflight. Each receipt is per-attempt evidence and earlier receipts remain necessary for the full history.

The first committed RED `47d4e89` names the absent admission APIs; local GREEN `d36dad8` passed seven invalid-input scenarios and all eight mode/authority combinations across three test functions. Recovery RED `79e1c22` names the missing bound recovery APIs; initial local GREEN `425fb8c` passed seven combined admission/recovery test functions. Final full-suite and coverage measurements belong in the current Gap checkpoint, not this intermediate evidence. All inputs are synthetic unit fixtures; no actual review, authority issuance, capture read, authorization prompt or Zotero mutation occurred.

Positive consequences are one validation source per contract, explicit destination authorization and retained recovery scope. Costs are retained full-scope audit storage, per-attempt receipt history and exact-representation digest ordering. Rejected alternatives include copying the validation or transport loops, which would drift, and deserializing the new scope around permissive legacy nested DTOs, which would silently ignore input. Executable persistence after restart, delayed reconciliation of an unknown original write, independent deployed issuers and approved live write/rollback remain unfinished. Local success does not make this ADR Accepted.

Implementation references: Serde Project. (n.d.). *Field attributes*. Retrieved September 6, 2026, from https://serde.rs/field-attrs.html; Serde Project. (n.d.). *Implementing Serialize*. Retrieved September 6, 2026, from https://serde.rs/impl-serialize.html. Existing dependency APIs only; Context7 returned its monthly quota limit, so these official references were checked directly. DeepWiki has no indexed ConceptWeave repository. The owner source and tests are the implementation evidence.

## Original-write observation follow-up (2026-09-06; still Proposed)

In the context of inspecting an original write after its immediate verification failed, facing loss of the actual submitted precondition and the risk of treating later matching values as completion, we decided for one read-only observation attached to the unchanged opaque attempt and against automatic replay or equality-based recovery admission, to achieve auditable inspection without expanding authority, accepting that durable resolution and approved recovery remain unfinished.

For example, the first item can advance the library revision before the second write loses its response. The plan's initial library revision cannot identify that second request's precondition. Runtime `dcc36310394c68fca74251ae85fe72d942be32ba` retains the exact last submitted request only when the existing executor reports it indeterminate. It neither reconstructs that version from inverse work nor creates a second executor. The later observation borrows the complete original receipt, preserving earlier inverse operations, the failed item, untouched items and the same scope commitment. Failed reads omit adapter errors; foreign or malformed returned state remains explicitly unverified evidence. Dry-run, preflight failure, successful and known-failure receipts cannot invoke this read path. The unknown outcome is never cleared and the existing rollback guard still refuses it.

Rejected alternatives include a before/after classifier that would suggest causal completion without proving the earlier request has stopped, and a write-token retry engine. Official Zotero documentation makes tokens redundant for versioned requests; local token caches are memory-only and forgotten on restart. Neither a cached token nor matching metadata supplies governance, durable history or peer authentication. This additive inspection improves operator evidence while leaving the resolution gate closed; it does not complete original-write recovery. Retain the owner-only original scope to verify its commitment and earlier observation files to reconstruct history.

RED `e300eb8` failed with the absent observation API. The candidate passed eleven focused functions and 255 workspace tests in 41 unfiltered suites, including seven compile-fail doctests, plus the unchanged static and coverage gates recorded in the Gap baseline. Tests exercise unknown first/second writes, before/after/foreign/malformed/failed observations, exact advanced preconditions, preserved original receipt serialization, redacted authority inputs and continued zero-I/O recovery refusal. These are synthetic unit cases, not live request or approval evidence. Research references and the scope limits are recorded in the full-text audit. No ADR number, Accepted status or protected branch rule changes.

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
