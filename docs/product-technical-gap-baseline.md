# Product / Technical Gap Baseline

**Snapshot:** 2026-09-08. Live protected-branch, PR, issue, workflow, and exact-head state overrides dated evidence. Historical detail remains traceable in Git history and the `docs/doctoring/` evidence records; this baseline keeps the current product/technical gaps and active acceptance coordinates code-current. A baseline-only commit does not self-record its own SHA because doing so would create recursive documentation churn; the live PR ref is authoritative for the exact docs successor.

## Current protected truth and active stack

- Protected/default `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`. No immutable ConceptWeave release exists.
- Foundation #1: `b538470c963e6524ddc0c3f652a46a4fc8265150`, OPEN Draft/mergeable. It remains behind Product-CI bootstrap #35.
- Product-CI bootstrap #35: `22709ec9b4d969bf67ec74db402813e74d11f7ca`, OPEN/non-Draft/mergeable. Security Scan `34204381232` and SAST `34204381260` are terminal success. CodeQL PR `34204381235` is terminal failure: language detection succeeded, compatibility verdict enforcement failed, and the separate current-head dispatch job succeeded. Dispatch success is not authenticated terminal CodeQL GREEN. Protected central `.github/main` is `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`; exact status/SARIF/receipt recovery remains central owner work.
- Client Consumption #5: `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open. It owns provider-independent released semantic-contract admission, integrity, compatibility, diff/resolution, and supersession consumption.
- Source Observation #6: `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`, Draft/open on #5. It remains read-only source-observation evidence; source-system business truth stays with its owner.
- Research Intake #9: live branch `feat/zotero-research-classification@b98ba39f9efa4c18c1bcdc3bcae4a8ef4f0abd41`, OPEN Draft/mergeable on Foundation. It is the canonical owner for the read-only Zotero snapshot, deterministic research classification, source inventory, and report-publication safety seams.
- Pending-source resolution #40: latest live documentation head `d956984bd202ed1678d1144e4bb1980ffa28e5aa`, OPEN Draft/mergeable on #9. The live #40 branch may contain only later baseline/PR-documentation descendants; PR metadata is authoritative for that exact head. #40 owns the typed read-only exact-snapshot resolution aggregate and does not gain Zotero write, semantic publication, or approval authority.
- Full-text/write/recovery PR #39: latest live documentation head `ae572879524a15944d1a9df6e46deb4b154059ba`, OPEN Draft/mergeable on #38 `7678236ed3ec467e93b97bb2ad7ad26b3dc0e5b9`. Previously reported unique-repository coverage counts were withdrawn after duplicate audits and are not current KPIs; live PR metadata wins if this independent writer advances.

Predecessor review/check evidence never transfers automatically to a changed head. A queued or failed external-owner lane blocks only that acceptance lane; it is not a reason for no-op pushes, fabricated receipts, manual-dispatch-as-GREEN, self-approval, review dismissal, or gate weakening.

## Current Research Intake / source-resolution integrity

PR #40 exact head `d6867f916b00948586a53129da9b0cf17cf2bbad` added a RED test
for blank stored `rule_revision`; the owner repair now rejects whitespace-only
revisions during deserialization. Focused source-resolution tests and strict
Clippy pass locally. The repair remains pending unchanged-head hosted checks,
owned coverage and independent review.

Research Intake #9 retains the earlier source/test repairs for proposal-only truth/publication projection, multilingual abstention classification, replayable abstention evidence, DOI duplicate provenance, fragmented HTTP-header testing, matched tag provenance, provider-shaped Zotero object-key admission, Local API transport coverage, bounded/private report publication, and post-publication cleanup safety. These are owner-local repairs; no later #40 or #39 evidence can reverse-prove #9 GREEN.

The current #9 owner coverage checkpoint remains RED rather than being hidden by exclusions. At `b98ba39f9efa4c18c1bcdc3bcae4a8ef4f0abd41`, the platform-explicit fixture removes the final runtime-only normalized branch while normalized regions/raw generic instantiations remain incomplete. Exact-head Rust 1.98 workspace tests, fmt, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, hosted checks, and independent review remain the acceptance gate.

Three standalone PDFs plus one standalone note remain governed pending sources. Empty pending keys, deterministic classification, duplicate grouping, local report serialization, full-text availability, or a source-resolution object never grants semantic approval or Zotero mutation authority.

### Stored source-resolution artifact repair

`SourceResolutionReview` constructor admission already rejects missing/blank report server identity, wrong server/library/item identity, duplicate/unknown decisions, missing decisions, blank reasons, and missing retained inventory. Stored JSON must preserve the invariants that are provable from the artifact itself rather than bypassing constructor admission.

- Nested provider-identity and library-revision deserialization guards are retained from the earlier RED/repair sequences.
- Reality RED `99d36562a7f101c3e1c2db39f9d493d228b2b133` mutates a valid stored review into duplicate item keys, reversed decision order, and a whitespace-only reason.
- Minimal production repair `fdee1fb4d70481bdb31a604e5235470653925b23` rejects blank stored item keys/reasons and requires strictly increasing `resolved_sources` item keys. That single invariant rejects both duplicate and unsorted stored decision sets without changing `prepare_source_resolution_review`, semantic authority, or Zotero behavior.
- Ordinary merge `fa8ba9c27718bc290b3c2c2c5738c3033f9f063c` preserves the concurrent RED and production repair. Local evidence recorded at that code checkpoint includes workspace/focused source-resolution tests, strict Clippy, and diff checks; owned coverage remained RED.
- Docs checkpoint `f5b05f50696be72c85646be85d30de43ff5e34b0` records the repair. Exact test-only successor `4c05fcd59aa31b70b1054c4d8b53697628530d27` adds the direct whitespace-only stored `item_key` regression required to exercise the newly introduced deserialization branch. Production code is unchanged by that successor.

Current #40 status is therefore **SOURCE_TEST_REPAIRED_PENDING_CI**, not GREEN. The latest source/test checkpoint `4c05fcd...` had no pull-request workflow run when checked, and prior local execution cannot be transferred across later test/docs heads. Keep the canonical review thread unresolved until one unchanged exact live successor has the full Rust/coverage/hosted/independent-review evidence.

## Quality and release gates

Rust 1.98.0 is the production baseline. Unsafe code is forbidden in owned Rust, public APIs require beginner-readable rustdoc, and deprecations are repaired at the cause rather than suppressed. Owned production acceptance requires 100% documented/function/normalized-region/branch coverage where tooling exposes it. Raw LLVM denominators remain reported separately; duplicate generic instantiations are not silently treated as owner gaps or suppressed to manufacture GREEN.

Product/CI acceptance requires exact PR-head checkout verification, fmt, all-target Clippy, workspace tests, warnings-denied rustdoc and release build, owned coverage, schema/fixture checks where applicable, lockfile freshness, clean tree, applicable SAST/security/dependency/review checks, and independent approval. No predecessor GREEN transfers after source or base movement.

A release can begin only from an exact protected release-ready head. Version, CHANGELOG, tag, package, immutable `semantic_release`, SBOM, provenance/attestation, reproducibility, and rollback proof are all required. No immutable ConceptWeave release currently exists.

## Foundation capability status

| Area | Status | Current gap / acceptance |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map keep ConceptWeave responsible for `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases, and stable client contracts. Foreign domain truth remains behind released/versioned ports and ACLs. |
| Research lifecycle | SOURCE_REPAIRED_PENDING_CI | Proposal truth and publication state remain separate and proposal-only until steward validation/review/publication. Current-head execution still required. |
| Research evidence integrity | SOURCE_REPAIRED_PENDING_CI | Abstention replay, DOI provenance, matched-tag source order/deduplication, provider key validation, transport and publication durability have causal repairs; exact current-head owned coverage/hosted evidence remains incomplete. |
| Pending-source resolution | SOURCE_TEST_REPAIRED_PENDING_CI | #40 rejects missing provider identity, nested server/library drift, duplicate/unsorted/blank stored decisions and preserves exact snapshot binding. `4c05fcd...` adds direct blank stored-key coverage. Full unchanged-head acceptance remains outstanding. |
| Source Observation | ACTIVE_CHILD | Concrete bounded read-only PostgreSQL adapter/conformance remains outstanding. No source-system domain truth may be imported into ConceptWeave. |
| Client Consumption | ACTIVE_CHILD | Released semantic-contract admission/integrity/compatibility exists as a child lane; protected exact-head evidence and prerequisite integration remain outstanding. |
| Product CI bootstrap | BLOCKED_OWNER | #35 has current Security/SAST success but terminal CodeQL failure despite successful dispatch. Central owner must provide authenticated terminal exact-head verdict; leaf no-op reruns are prohibited. |
| Full-text/write terminal | ACTIVE_DOWNSTREAM | #39 remains downstream and must consume governed exact pending-source resolution before clearing or inferring pending sources. Historical repository-count KPIs are withdrawn. |
| Release | NOT_STARTED | No protected immutable ConceptWeave release exists. |

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind the source port; exact-binding credential resolution; read-only stable snapshot; schema allowlist; operation/statement deadlines; cancellation and row/byte/concurrency budgets; complete immutable snapshot or fail closed; disappearance handling; deterministic frozen anonymized conformance fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers, null-comparison semantics, and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and explicit abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts; relational structure alone is not business authority.
5. **LLM proposal** — every production model call through a released `contextual-orchestrator`; output remains proposed/inferred with source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility, and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning, and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox, and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection, and immutable publication receipts.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD and explicitly version-bound additional formats; draft/incubating formats cannot be represented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream owners retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA, and other consumers; no source copying, cross-service SQL, or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, and CJK/font/text-expansion checks where published labels or UI are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build, and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables, or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; correction creates a new release plus supersession evidence rather than in-place overwrite.
