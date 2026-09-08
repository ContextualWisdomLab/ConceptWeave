# Product / Technical Gap Baseline

**Snapshot:** 2026-09-08. Live protected-branch, PR, issue, workflow, and exact-head state overrides dated evidence. Historical evidence remains in Git history and `docs/doctoring/`. A baseline-only commit does not record its own SHA; the live PR ref is the authority for that docs successor.

## Current protected truth and active stack

- Default `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`. No immutable ConceptWeave release exists. Organization ruleset `CWL Central required workflows` is active for this repository; classic branch status-check fields alone are not the complete protection authority.
- Foundation #1: `b538470c963e6524ddc0c3f652a46a4fc8265150`, OPEN Draft/mergeable, still behind Product-CI bootstrap #35.
- Product-CI bootstrap #35: `22709ec9b4d969bf67ec74db402813e74d11f7ca`, OPEN/non-Draft/mergeable. Security Scan `34204381232` and SAST `34204381260` are success; CodeQL PR `34204381235` is terminal failure after a successful current-head dispatch job. Protected central `.github/main` is `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`; central owner `.github#2040@d93a78ab4262c5228af7eda258ee0af58b880de7` is the current non-Draft repair lane for the nested rerun-envelope and sibling-wake race. Neither dispatch success nor an unmerged owner repair retroactively makes the #35 leaf CodeQL GREEN.
- Client Consumption #5: `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open; owner of provider-independent released semantic-contract admission, integrity, compatibility, diff/resolution and supersession consumption.
- Source Observation #6: `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`, Draft/open on #5; source-system business truth stays with its owner.
- Research Intake #9: `b98ba39f9efa4c18c1bcdc3bcae4a8ef4f0abd41`, OPEN Draft/mergeable on Foundation; canonical owner of the read-only Zotero snapshot, deterministic research classification, source inventory and report-publication safety seams.
- Pending-source resolution #40: current exact head `8d469af4e4e46c84c34b823a6651d26609997e3b` before this baseline-only successor, OPEN Draft/mergeable on #9. Reality RED `6b28fc38afaae455a70901c57b28116b0b88510d` is causally repaired by source commit `c19c787cd9153ed39f3c34370d2f9fa18fef7c8a`; `d774c5b6382e71c78a32359cdef2c75c7f80a04d` is formatting-only and `8d469af...` is docs-only. #40 owns only the typed read-only exact-snapshot resolution aggregate; it has no Zotero write, semantic publication or approval authority.
- Full-text/write/recovery #39: `6f8075b46afe480e2f0c14a1ede41f18905559ff`, OPEN Draft/mergeable on #38 `7678236ed3ec467e93b97bb2ad7ad26b3dc0e5b9`. This independent writer may advance normally; live PR metadata wins. Historical repository-count KPIs remain withdrawn.

Predecessor checks and reviews never transfer automatically to a changed head. A queued or failed external-owner lane blocks only that acceptance lane; it does not justify no-op pushes, fabricated receipts, manual-dispatch-as-GREEN, self-approval, review dismissal or gate weakening.

## Research Intake and source-resolution integrity

Research Intake #9 retains causal repairs for proposal-only truth/publication projection, multilingual abstention classification, replayable abstention evidence, DOI duplicate provenance, fragmented HTTP-header testing, matched-tag provenance, provider-shaped Zotero object-key admission, Local API transport coverage, bounded/private report publication and post-publication cleanup safety. Those are #9 owner-local repairs; #40/#39 evidence cannot reverse-prove #9 GREEN.

At #9 `b98ba39...`, owned coverage remains RED rather than hidden behind exclusions. Exact-head acceptance still requires locked Rust 1.98 workspace tests, fmt, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks and independent review.

Three standalone PDFs plus one standalone note remain governed pending sources. Empty pending keys, deterministic classification, duplicate grouping, local report serialization, full-text availability or a source-resolution object never grants semantic approval or Zotero mutation authority.

### Stored source-resolution artifact integrity

`SourceResolutionReview` constructor admission rejects missing/blank report server identity, wrong server/library/item identity, duplicate/unknown decisions, missing decisions, blank reasons and missing retained inventory. Stored JSON must retain independent completeness evidence and revalidate the same identity boundary rather than validating only whichever decisions happen to remain in the artifact.

- Earlier source repairs reject nested provider/library drift, blank stored item keys/reasons and duplicate/unsorted decision sets. RED `99d36562a7f101c3e1c2db39f9d493d228b2b133` and repair `fdee1fb4d70481bdb31a604e5235470653925b23` remain in ancestry; `4c05fcd59aa31b70b1054c4d8b53697628530d27` directly covers the blank stored-key branch.
- Minimal repair `6dbc013d81a4cff1e46fbe43f4d920d968c425c0` rejects blank/whitespace stored `rule_revision` without hard-pinning to the current revision value. Future nonblank revisions remain representable; semantic authority and Zotero behavior are unchanged. AGENTS records the same invariant.
- Stored-completeness RED `6b28fc38afaae455a70901c57b28116b0b88510d` starts from a constructor-valid two-pending-source review, serializes it, removes one decision and requires deserialization rejection. Source repair `c19c787cd9153ed39f3c34370d2f9fa18fef7c8a` adds `pending_source_item_keys` to the review/wire envelope, copies the report's canonical pending-key sequence into constructor output, rejects non-strictly-ordered expected keys and requires exact equality between expected keys and restored decision keys. The new field has no serde default, so an older artifact lacking this evidence fails closed rather than silently regaining typed completeness.
- Style successor `d774c5b6382e71c78a32359cdef2c75c7f80a04d` changes only formatting in the affected regressions. Docs successor `8d469af4e4e46c84c34b823a6651d26609997e3b` refreshes the recorded coordinate. Neither changes the source semantics introduced by `c19c787...`.
- No predecessor execution transfers to these successors. Exact-head Rust/coverage/hosted/independent-review evidence remains required before GREEN or thread resolution.

Current #40 status is **SOURCE_TEST_REPAIRED_PENDING_CI**, not GREEN. Keep Draft and keep valid review findings unresolved until one unchanged exact successor completes the full Rust/coverage/hosted/independent-review gate.

## Quality and release gates

Rust 1.98.0 is the production baseline. Unsafe code is forbidden in owned Rust, public APIs require beginner-readable rustdoc, and deprecations are repaired at the cause rather than suppressed. Owned production acceptance requires 100% documented/function/normalized-region/branch coverage where tooling exposes it; raw LLVM denominators are reported separately and are not silently normalized into a false GREEN.

Product/CI acceptance requires exact PR-head checkout verification, fmt, all-target Clippy, workspace tests, warnings-denied rustdoc and release build, owned coverage, schema/fixture checks where applicable, lockfile freshness, clean tree, applicable SAST/security/dependency/review checks and independent approval. No predecessor GREEN transfers after source or base movement.

A release begins only from an exact protected release-ready head. Version, CHANGELOG, tag, package, immutable `semantic_release`, SBOM, provenance/attestation, reproducibility and rollback proof are all required. No immutable ConceptWeave release currently exists.

## Foundation capability status

| Area | Status | Current gap / acceptance |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map keep ConceptWeave responsible for `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable client contracts. Foreign domain truth remains behind released/versioned ports and ACLs. |
| Research lifecycle | SOURCE_REPAIRED_PENDING_CI | Proposal truth and publication state remain separate and proposal-only until steward validation/review/publication. Current-head execution remains required. |
| Research evidence integrity | SOURCE_REPAIRED_PENDING_CI | Abstention replay, DOI provenance, matched-tag source order/deduplication, provider key validation, transport and publication durability have causal repairs; exact current-head owned coverage/hosted evidence remains incomplete. |
| Pending-source resolution | SOURCE_TEST_REPAIRED_PENDING_CI | #40 now rejects missing provider identity, nested server/library drift, duplicate/unsorted/blank stored decisions, blank stored rule revision and truncated stored decision sets. Full unchanged-head acceptance remains outstanding. |
| Source Observation | ACTIVE_CHILD | Concrete bounded read-only PostgreSQL adapter/conformance remains outstanding. No source-system domain truth may be imported into ConceptWeave. |
| Client Consumption | ACTIVE_CHILD | Released semantic-contract admission/integrity/compatibility exists as a child lane; protected exact-head evidence and prerequisite integration remain outstanding. |
| Product CI bootstrap | BLOCKED_OWNER | #35 has Security/SAST success but terminal CodeQL failure. Central `.github#2040` owns the active rerun-settlement repair; the leaf remains stable until authenticated terminal exact-head evidence exists. |
| Full-text/write terminal | ACTIVE_DOWNSTREAM | #39 remains downstream and must consume governed exact pending-source resolution before clearing or inferring pending sources. Historical repository-count KPIs are withdrawn. |
| Release | NOT_STARTED | No protected immutable ConceptWeave release exists. |

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind the source port; exact-binding credential resolution; read-only stable snapshot; schema allowlist; operation/statement deadlines; cancellation and row/byte/concurrency budgets; complete immutable snapshot or fail closed; disappearance handling; deterministic frozen anonymized conformance fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers, null-comparison semantics and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and explicit abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; relational structure alone is not business authority.
5. **LLM proposal** — every production model call through a released `contextual-orchestrator`; output remains proposed/inferred with source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipts.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD and explicitly version-bound additional formats; draft/incubating formats cannot be represented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream owners retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels and CJK/font/text-expansion checks where published labels or UI are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; correction creates a new release plus supersession evidence rather than in-place overwrite.
