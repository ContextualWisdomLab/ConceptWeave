# ADR 0007: Preserve the Research Intake aggregate while adding golden-set evaluation

- Status: Proposed
- Date: 2026-09-14
- Supersedes: the implementation details of ADR 0006's 2026-09-05 integrity amendment and September 6 source-scope admission amendment; ADR 0006's read-only Zotero and Research Intake decisions remain in force.

## Problem

Golden-set PR #10 was developed from historical Research Intake head `51c7df6d03f072449422fd58ca24b2f9d6026f07`. Canonical Research Intake #9 later advanced to `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, making `ClassificationReport` provenance/inventory/proposal state private and constructor-bound and retaining the stable public `ZoteroItem { key, version, data }` caller contract.

The historical #10 implementation conflicted with both boundaries. Its evaluator directly read or mutated trusted report fields, and its raw-source design added mandatory public `ZoteroItem.source_record: Option<Value>`, requiring unrelated callers to manufacture `None`. A mechanical merge would either reopen trusted state or break existing source callers.

A later exact-head review (`5192398634`) found a second boundary error in the first reconciliation: a public `CapturedZoteroItem::try_from(Value)` can bind arbitrary caller-supplied JSON, but cannot authenticate that the bytes came from Zotero. The initial `conceptweave-zotero-provider-snapshot-v3` label therefore overstated the authority of that receipt class.

## Constraints

- Preserve every valid current-#9 lifecycle, abstention, source-resolution, provenance and publication delta.
- Preserve #10's source/proposal digest, steward-label, fail-closed approval and aggregate-only evaluation behavior.
- Do not make caller-supplied absence or presence of raw JSON an authenticity claim.
- Keep captured raw evidence private after projection so raw and typed inputs cannot drift independently.
- Distinguish content binding from provider/source authentication.
- Existing v1/v2 and superseded provider-labelled receipts are not migration authority.
- No Zotero mutation or semantic publication authority is added.

## Alternatives

1. **Reopen `ClassificationReport` fields.** Rejected because it defeats the current Research Intake aggregate invariant and permits ordinary external reminting/mutation of trusted state.
2. **Keep `source_record` as a new public `ZoteroItem` field.** Rejected because it is a source-breaking caller contract and turns fabricated `None` into ambiguous provenance.
3. **Hash only the typed projection.** Rejected for complete raw-capture evidence because unknown nested metadata and omitted-versus-explicit fields would be lost.
4. **Hash only raw JSON.** Rejected because classifier semantics depend on the typed projection; the receipt must bind what the classifier actually consumed.
5. **Treat a public `Value` constructor as provider authentication.** Rejected because callers can fabricate the value; digest domain selection cannot establish source origin.
6. **Duplicate Research Intake into a second evaluator-owned report type.** Rejected because it creates a competing trusted aggregate and repeats source/pending/lifecycle logic.
7. **Layer evaluation on read-only Research Intake accessors, keep raw capture content-bound only, and reserve source authentication for a transport-owned attestation boundary.** Selected.

## Decision

PR #10 ordinary/non-force integrates current #9 as a second parent. `ClassificationReport` and public `ZoteroItem` remain unchanged from current #9. The crate root composes the existing Research Intake module with a separate golden-set module.

`CapturedZoteroItem` owns one complete caller-supplied raw JSON value and the `ZoteroItem` decoded from that value. Both fields are private; only an immutable typed-item accessor is exposed. Raw-capture receipts hash canonical raw JSON plus typed input under `conceptweave-zotero-captured-json-snapshot-v3`. This proves exact content binding, not provider origin. Typed offline fixtures use the distinct `conceptweave-zotero-typed-snapshot-v3` domain and likewise make no provider-authentication claim.

RED `67b5214ce8cd39fddbc1ba6043971b9832b2ef17` requires the public capture path to use the non-authenticating domain. Production repair `2e52972745953fbb5b606226fce2dda2cf61abd3` changes the domain and rustdoc; doctoring `c0a139d82649d44b7fc8a115357ddc7b961315ed` records invalidation of the superseded provider-labelled semantics.

`GoldenSnapshot` owns the constructor-produced report, exact item revision inventory and snapshot digest. It exposes read-only accessors. Golden evaluation consumes only those accessors and therefore does not require a test-only backdoor into trusted report fields. Invalid external states are exercised through malformed caller inputs and mismatched approval evidence rather than by making trusted fields public.

Proposal identity remains `conceptweave-classification-proposals-v3` and binds all current report evidence reachable through Research Intake accessors, including lifecycle/evidence fields, retained sources, pending coordinates, duplicate provenance and reader metadata. Canonical ordering removes irrelevant record-order differences.

## Effects and risks

The stale-parent conflict is resolved without force-push, destructive rebase or whole-tree side selection. Existing `ZoteroItem` struct literals remain valid. Raw and typed forms cannot be mutated independently after capture, but that integrity property must not be confused with source authenticity.

The public raw-capture API is explicit rather than silently inserted into `read_local_snapshot`. If the product later requires authenticated Zotero-origin evidence, a canonical transport adapter that actually observes the provider response must mint a separate attestation binding provider/repository/library/version/transport identity or equivalent evidence. It may reuse the same canonical content-binding primitive, but a caller-selected `Value` or digest domain cannot grant that authority. It must not duplicate Research Intake pagination/budget logic.

The superseded `conceptweave-zotero-provider-snapshot-v3` label is unreleased Draft evidence, not an immutable semantic release. Existing local receipts under it require fresh capture and independent approval rather than local digest migration.

The wrapper crate root is an integration seam, not a second domain owner. If it introduces module-path, rustdoc, coverage or binary-link regressions, those are acceptance failures and must be repaired before Ready/merge. No predecessor Rust or hosted GREEN transfers to the reconciled head.

## Verification

Required acceptance is unchanged-head Rust 1.98 workspace tests, fmt, strict all-target/all-feature Clippy, warnings-denied rustdoc, release build, owned production doc/rustdoc/test/edge coverage, applicable hosted Product/security/dependency/review checks, and qualifying independent review. The current branch remains Draft until those gates are proven.
