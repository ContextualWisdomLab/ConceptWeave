# ADR 0007: Preserve the Research Intake aggregate while adding golden-set evaluation

- Status: Proposed
- Date: 2026-09-14
- Supersedes: the implementation details of ADR 0006's 2026-09-05 integrity amendment and September 6 source-scope admission amendment; ADR 0006's read-only Zotero and Research Intake decisions remain in force.

## Problem

Golden-set PR #10 was developed from historical Research Intake head `51c7df6d03f072449422fd58ca24b2f9d6026f07`. Canonical Research Intake #9 later advanced to `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, making `ClassificationReport` provenance/inventory/proposal state private and constructor-bound and retaining the stable public `ZoteroItem { key, version, data }` caller contract.

The historical #10 implementation conflicted with both boundaries. Its evaluator directly read or mutated trusted report fields, and its raw-source design added mandatory public `ZoteroItem.source_record: Option<Value>`, requiring unrelated callers to manufacture `None`. A mechanical merge would either reopen trusted state or break existing source callers.

## Constraints

- Preserve every valid current-#9 lifecycle, abstention, source-resolution, provenance and publication delta.
- Preserve #10's source/proposal digest, steward-label, fail-closed approval and aggregate-only evaluation behavior.
- Do not make caller-supplied absence of raw provider data an authenticity claim.
- Keep provider-captured raw evidence private after projection so raw and typed inputs cannot drift independently.
- Existing v1/v2 receipts are not migration authority.
- No Zotero mutation or semantic publication authority is added.

## Alternatives

1. **Reopen `ClassificationReport` fields.** Rejected because it defeats the current Research Intake aggregate invariant and permits ordinary external reminting/mutation of trusted state.
2. **Keep `source_record` as a new public `ZoteroItem` field.** Rejected because it is a source-breaking caller contract and turns fabricated `None` into ambiguous provenance.
3. **Hash only the typed projection.** Rejected for provider-captured evidence because unknown nested metadata and omitted-versus-explicit fields would be lost.
4. **Hash only raw JSON.** Rejected because classifier semantics depend on the typed projection; the receipt must bind what the classifier actually consumed.
5. **Duplicate Research Intake into a second evaluator-owned report type.** Rejected because it creates a competing trusted aggregate and repeats source/pending/lifecycle logic.
6. **Layer evaluation on read-only Research Intake accessors and add a separate provider capture object.** Selected.

## Decision

PR #10 ordinary/non-force integrates current #9 as a second parent. `ClassificationReport` and public `ZoteroItem` remain unchanged from current #9. The crate root composes the existing Research Intake module with a separate golden-set module.

`CapturedZoteroItem` owns one complete provider JSON value and the `ZoteroItem` decoded from that value. Both fields are private; only an immutable typed-item accessor is exposed. Provider receipts hash canonical raw JSON plus typed input under `conceptweave-zotero-provider-snapshot-v3`. Typed offline fixtures use a distinct `conceptweave-zotero-typed-snapshot-v3` domain and explicitly do not claim provider authenticity.

`GoldenSnapshot` owns the constructor-produced report, exact item revision inventory and snapshot digest. It exposes read-only accessors. Golden evaluation consumes only those accessors and therefore does not require a test-only backdoor into trusted report fields. Invalid external states are exercised through malformed provider/caller inputs and mismatched approval evidence rather than by making trusted fields public.

Proposal identity advances to `conceptweave-classification-proposals-v3` and binds all current report evidence reachable through Research Intake accessors, including lifecycle/evidence fields, retained sources, pending coordinates, duplicate provenance and reader metadata. Canonical ordering removes irrelevant record-order differences.

## Effects and risks

The stale-parent conflict is resolved without force-push, destructive rebase or whole-tree side selection. Existing `ZoteroItem` struct literals remain valid. Raw provider binding is stronger than the old caller-owned optional field because the raw and typed forms cannot be mutated independently after capture.

The new provider capture API is explicit rather than silently inserted into `read_local_snapshot`; callers that need complete provider-level golden receipts must enter through the captured wire boundary. Treating a plain typed `ClassificationReport` as raw-provider-authenticated evidence remains prohibited. A later Local API convenience path may delegate to the same capture primitive, but it must not duplicate Research Intake pagination/budget logic.

The wrapper crate root is an integration seam, not a second domain owner. If it introduces module-path, rustdoc, coverage or binary-link regressions, those are acceptance failures and must be repaired before Ready/merge. No predecessor Rust or hosted GREEN transfers to the reconciled head.

## Verification

Required acceptance is unchanged-head Rust 1.98 workspace tests, fmt, strict all-target/all-feature Clippy, warnings-denied rustdoc, release build, owned production doc/rustdoc/test/edge coverage, applicable hosted Product/security/dependency/review checks, and qualifying independent review. The current branch remains Draft until those gates are proven.
