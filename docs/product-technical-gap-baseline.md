# Product / Technical Gap Baseline

**Snapshot:** 2026-09-14

This file is the code-current ConceptWeave gap baseline. Exact SHAs are evidence coordinates, never mutable dependencies. Live protected branches, PRs, reviews and workflow results supersede this snapshot when they advance.

## Canonical ownership

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client contract for those releases. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production model/provider routing. Product-domain truth remains with its canonical owner. Consumers use released/versioned semantic-release/contract/ACL coordinates only; source copies, cross-service SQL and mutable sibling heads are invalid integration paths.

## Protected and central gate state

ConceptWeave protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; no immutable ConceptWeave semantic release exists.

Product bootstrap #35 is OPEN / Ready / mechanically mergeable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its repository-owned Product workflow is still absent from protected `main`, so descendant PRs do not gain Product acceptance merely from leaf source status.

Protected `.github/main` is `828eaaefb0cc97bba4da63eb9270447476d26710`. Central CodeQL handler #2106 is OPEN / Ready / mechanically mergeable at `1ba96e4ddf6a800435651ec1c49acff533242fd9`; central review-repair #2170 is OPEN / Ready / mechanically mergeable at `ae0f2f57f1d2abda7bb2e7ac9ce3bf8f1cac6f39`. On #2170's exact head, Agent Review Runtime Quality CI `34775874685` is terminal success after the full-suite dependency repair; CodeQL PR, Security Scan, Python Security and SAST Semgrep remain queued, so central acceptance is still pending. Noema owner #2079 is OPEN / Ready / mechanically mergeable at `6d7e833224e06b4316df3d6bbdfcb4658e151956` with finding/probe and docstring source repairs present but fresh acceptance pending. None of that evidence transfers to ConceptWeave leaf heads.

## Research Intake #9 and golden-set evaluation #10

Canonical Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`. Its `ClassificationReport` provenance/inventory/proposal state is private and constructor-bound with read-only accessors. Public `ZoteroItem` remains `{ key, version, data }`. Source resolution, lifecycle, abstention, duplicate provenance, bounded Local API reading and proposal-only authority stay with #9.

Golden-set #10 was stale on historical #9 `51c7df6...`. Ordinary two-parent integration `c9954e286041c80d08655e092f10007a684ceab1` adopts both historical #10 and current #9 without force or destructive rebase. The repaired architecture no longer modifies `ZoteroItem` or `ClassificationReport`: `GoldenSnapshot` layers immutable review evidence over the trusted report and evaluation uses read-only accessors.

A fresh provenance review found that public `CapturedZoteroItem::try_from(Value)` cannot authenticate where caller-supplied JSON originated. RED `67b5214ce8cd39fddbc1ba6043971b9832b2ef17` requires that such caller-constructible capture use a non-authenticating receipt domain. Production `2e52972745953fbb5b606226fce2dda2cf61abd3` changes the raw-capture digest domain to `conceptweave-zotero-captured-json-snapshot-v3` and rustdoc now states that complete raw JSON plus typed classifier input are content-bound, not provider-authenticated. Doctoring `c0a139d82649d44b7fc8a115357ddc7b961315ed` invalidates the superseded provider-labelled receipt semantics, and `d1275f9092ead1155414048be680bd6e95e5dc76` aligns regression terminology. Genuine provider-origin evidence, if required, must be minted by a transport-owned capture/attestation boundary that actually observes the provider response; a public JSON constructor cannot grant that authority.

This is source/stack/provenance repair, not acceptance. The current #10 successor requires fresh exact-head Rust 1.98 workspace/fmt/strict Clippy/rustdoc/release/owned coverage, hosted checks and qualifying independent review. #11 and later research descendants must stay open and adopt the accepted #10 successor by ordinary non-force integration rather than copying selected files.

## Source Observation #46

Source Observation #46 is OPEN / Draft / mechanically mergeable at `445acfff1516a69c9180a06f930ead952f0d9f57` on #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`. PostgreSQL 18 NOT NULL partition linkage rejects impossible `conparentid`/`conislocal`/`coninhcount` tuples while preserving generic inheritance semantics. The state remains `NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / ACCEPTANCE_PENDING`; no Rust 1.98 or hosted Product GREEN is claimed for that exact head.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical ownership and released-contract consumer boundary are defined; foreign product truth remains outside ConceptWeave. |
| Research Intake | OWNER_REPAIRED_PENDING_ACCEPTANCE | #9 owns trusted read-only intake; fresh exact-head acceptance remains required before propagation. |
| Golden-set evaluation | PROVENANCE_REPAIRED_PENDING_ACCEPTANCE | #10 ordinary-adopted current #9, repaired aggregate compatibility, and no longer conflates caller-captured raw JSON with provider authentication; Rust/hosted/review evidence is not yet established. |
| Product CI | BLOCKED_CENTRAL_ACCEPTANCE | #35 waits for protected central CodeQL/review settlement and then fresh unchanged-head Product acceptance. |
| Source Observation | ACTIVE_CHILD_PENDING_ACCEPTANCE | #46 carries PostgreSQL 18 evidence semantics but remains Draft without exact-head native/hosted GREEN. |
| Ontology / semantic-layer generation | P0_GAP | Evidence-bound discovery, alignment, validation, review and publication engines remain incomplete. |
| LLM integration | OWNER_BOUND | Production model calls use released `contextual-orchestrator`; output stays proposed/inferred until steward validation/publication. |
| Security / governance | PENDING_EXACT_HEAD | Purpose-bound PII, least privilege, authenticated checks, formal review, immutable receipts, recovery and release evidence remain acceptance gates. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback are not executed on protected head. |

## P0 commercial / semantic gaps

1. Complete the concrete Rust PostgreSQL Source Observation adapter with registry/credential ACL, read-only stable snapshot, schema admission, non-resetting operation budgets, cancellation, row/byte/concurrency limits, disappearance handling and deterministic frozen-fixture conformance.
2. Observe PostgreSQL domains/enums/indexes/comments/quoted identifiers/cross-schema collisions as evidence without importing source business truth.
3. Build evidence-bound ontology candidate discovery for terms, concepts, taxonomy and non-taxonomic relations with abstention for unsupported semantics.
4. Build semantic-layer candidate generation for dimensions, measures, grain, units, relationships, physical mappings and deterministic calculation contracts.
5. Implement retrieval/pruning/structural alignment followed only by bounded optional LLM assistance, with OAEI-style evaluation and reproducible steward-visible matching evidence.
6. Implement RDF/OWL/SKOS/SHACL plus semantic-layer validation, conflict/duplicate/consistency checks and bounded reasoning with explicit unsupported-feature failure.
7. Add PostgreSQL 3NF candidate/evidence/validation/review/release/supersession receipts, idempotency/UPSERT/lock design, transactional outbox and temporal history only where semantically required.
8. Add Keyverse-backed identity context and product-owned auth UX for steward decisions, maker-checker where required, stale-decision protection and immutable publication receipts.
9. Publish only explicitly version-bound stable OWL/RDFS/SKOS/SHACL/JSON-LD contracts; draft/incubating formats must remain labeled as such.
10. Complete language-neutral client release/supersession/provenance/signature/compatibility/diff/resolution contracts while consumers retain physical authorization/execution.
11. Add KO/EN/JA/ZH/VI/ES/DE/FR semantic labels and CJK/font/text-expansion verification when UI or published labels become material; ontology locale labels remain separate from the versioned UI translation ledger.
12. Produce structured telemetry, backup/restore, security evidence, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.
13. When buyer-facing web/API paths materialize, measure async+k6/E2E p95 <=20 ms where applicable without sample shrinking or unrealistic warm-cache exclusions; profile query/I/O/runtime/render/GC and move genuine hot paths Rust-first when the budget is missed.

## DDD / governance constraints

Maintain explicit Subdomain/Bounded Context/Context Map/UL/Aggregate/Entity/VO/Domain Service/Repository/Event/Invariant alignment across code/API/DB/tests. External DTOs cross ACLs; no generic domain `utils/helpers/services/common` buckets. Source observations are evidence, not source-system business truth. Captured bytes or digests are evidence identities, not source authentication unless a canonical transport owner attests the origin. Published semantic truth is immutable; correction creates a new release plus supersession evidence. Production LLM results remain proposed/inferred. Purpose-bound PII, CSAP/SOC2 evidence readiness, deterministic receipts, recovery and auditability are release criteria rather than post-release documentation.

## Current merge/release rule

No #10, #35, #46 or dependent publication/release is authorized by this snapshot. First settle each canonical owner on an unchanged exact head. #10 requires native Rust and hosted/review acceptance after its ordinary parent and provenance repairs; research descendants then ordinary/non-force adopt it. Source Observation follows #46 acceptance -> complete #45 adoption -> fresh #45 acceptance -> #6 propagation. Product bootstrap waits for central protected handler/review settlement. Force push, destructive rebase, self-approval, review dismissal, fail-open substitution, provider bypass, no-op trigger commits, synthetic status, live Zotero mutation, premature semantic publication and predecessor-evidence transfer are not acceptance evidence.
