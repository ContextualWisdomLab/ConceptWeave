# Product / Technical Gap Baseline

**Snapshot:** 2026-09-03

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed from protected `main` yet.

## Active foundation slice — PR #1

| Area | Owner | Status | Evidence / action / next verification |
| --- | --- | --- | --- |
| Product boundary | ConceptWeave | ACTIVE_PR | PRD/TRD/ADR/context map define Semantic Model Engineering and CWL boundaries. Revalidate against exact PR #1 head `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` before merge. |
| Truth/publication lifecycle | Governance & Publication | ACTIVE_PR | Rust domain lifecycle defines Draft -> Proposed -> Validated while public transition APIs fail closed at steward-reviewed/publication boundaries; Draft 2020-12 candidate schema enforces public candidate shape and Published -> Authoritative consistency. |
| Rust baseline | ConceptWeave | ACTIVE_PR | Rust 1.98.0 workspace, unsafe forbidden, public docs required. |
| Quality gate | ConceptWeave | ACTIVE_PR | Exact PR #1 head `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` has Product run `33527150325` and SAST run `33527150417` terminal success. |
| Security/test/operability | ConceptWeave + central `.github` | CONTROL_PLANE_BLOCKED | Security Scan run `33527150445` reached exact checkout; dependency-review job `100059571813` failed at `Check dependency review support`, so the pinned Dependency Review step was skipped. OSV/Trivy/Scorecard succeeded but are not substitutes. |
| Merge governance | ContextualWisdomLab/.github | CONTROL_PLANE_BLOCKED | Live organization ruleset `18156473` requires one approving review and thread resolution on `~DEFAULT_BRANCH`; fresh foundation threads are resolved but no qualifying independent human APPROVED review exists. No self-approval or routine admin bypass is accepted. |

## Active Client Consumption slice — PR #5 / Issue #3

PR #5 is intentionally stacked on PR #1 because the client reuses only the foundation's public evidence/truth/publication types. It remains Draft. The canonical exact head is the live GitHub PR head; editing this file changes that SHA, so check evidence is valid only for the later unchanged branch head.

| Gap | Owner | Status | Evidence | Action | Next verification |
| --- | --- | --- | --- | --- | --- |
| Offline release admission | Client Consumption | IMPLEMENTED_PENDING_CHECKS | `SemanticReleaseClient` requires explicit compatibility plus Published + Authoritative and remains provider/network independent. | Preserve downstream tenant/purpose authorization and physical execution boundaries. | Exact-head Rust tests/Clippy/docs/coverage. |
| Versioned semantic-release shape | Client Consumption / Governance & Publication seam | IMPLEMENTED_PENDING_CHECKS | `contracts/semantic-release.schema.json` + fixtures; Rust `SemanticRelease` carries release/contract/ontology identity, truth/publication state, digest identity, provenance and unique concept IDs. | Keep language-neutral contract stable before generated bindings. | Exact-head AJV + Rust contract parity. |
| Detached artifact integrity | Client Consumption | IMPLEMENTED_PENDING_CHECKS | `verify_detached_artifact` hashes exact caller-supplied detached immutable semantic-artifact bytes with SHA-256 and compares the canonical declared digest after authoritative-use admission. Predecessor `0c32a7b55d3c687ab76cee789962866573496ba1` produced the expected hosted `E0599` RED when this API was absent; current production head `9c278598001c502a733100d11e901538c3dc2677` contains the minimal repair. | Keep the manifest digest scoped to detached artifact bytes; add signature/provenance-chain verification only when publication defines a stable signing contract. | Fresh exact-head Clippy/tests/rustdoc/coverage on the final documentation successor. |
| Release diff | Client Consumption | IMPLEMENTED_PENDING_CHECKS | Deterministic `diff` admits both releases through the same governance/compatibility gate and reports sorted added/removed concept IDs. | Extend only when typed relation/mapping/measure diff contracts exist. | Golden added/removed fixtures and exact-head Product run. |
| Exact concept resolution | Client Consumption | IMPLEMENTED_PENDING_CHECKS | `resolve_concept` performs exact deterministic lookup after authoritative-use admission; no fuzzy/LLM inference. | Add relation and physical-mapping resolution next. | Exact-head edge cases for unknown/blank IDs. |
| Explicit legacy compatibility | Client Consumption | IMPLEMENTED_PENDING_CHECKS | Test-first `091c36b24330671952de378d3596afcde5f62351`; production `2a4596e88d016e01a3bffded7a8436b14d55ec18` implements Current / SupportedLegacy / Unsupported without version-order inference. | Keep support explicit and bounded; unknown versions remain fail closed. | Hosted exact-head Product/Clippy/tests on final unchanged head. |
| Explicit immutable supersession | Client Consumption / Governance & Publication seam | IMPLEMENTED_PENDING_CHECKS | Test-first `67132eda0e25d23a4185d4b98f0c6dc3b11e17a4` required exact predecessor/successor id+digest references, nonblank rationale, self-supersession rejection, and ordinary authoritative-use admission. Production `2c4a7954ad3a4fb0dd0a5482a6870fcc0d2996a3` implements `SemanticReleaseReference`, `ReleaseSupersession`, and `validate_supersession`; follow-up tests exercise digest mismatch and both predecessor/successor admission paths. | Add a language-neutral supersession/publication-receipt schema before cross-language completeness; do not infer replacement from version order/time/diff. ADR 0004 stays Proposed while PR #5 is Draft/checks incomplete. | Exact-head fmt/Clippy/tests/rustdoc/100% coverage plus public-contract parity. |
| Documentation/API parity | Client Consumption | REPAIRED_PENDING_CHECKS | Public Rust API is `verify_detached_artifact`; PRD/TRD/UML/ADR/TEST_STRATEGY/SECURITY and this baseline are aligned to detached-artifact semantics, with a Rust documentation contract preventing the retired `verify_serialized_artifact` name from returning. | Preserve manifest-vs-detached-artifact scope across future bindings and schemas. | Exact-head documentation contract + full Product gate. |
| Match / align / explain | Model Alignment + Client Consumption | GAP | OLaLa/LLMs4OM/MILA/KROMA research traceability defines retrieve/filter/match constraints. | Deterministic candidate retrieval first; optional LLM only through `contextual-orchestrator`; never auto-authorize correspondences. | OAEI-style P/R/F1, retrieval recall, abstention and LLM-call-reduction evidence. |
| Query-plan contract | Client Consumption | GAP | Issue #3 requires semantic plans without owning physical execution. | Define versioned semantic query-plan DTO and consuming-product ACL seam. | GRC golden round-trip with no cross-service SQL. |
| Consumer authorization | Downstream product / Keyverse boundary | EXTERNAL_OWNERSHIP | ConceptWeave performs governance/compatibility admission only. | Keep tenant/purpose authorization and physical execution downstream. | Cross-tenant/purpose denial tests in each consumer. |

## Parallel Source Observation slice — PR #6 / Issue #2

PR #6 is a sibling stacked on PR #1 and is not copied into PR #5. Fresh live evidence on 2026-09-03 identifies exact head `2817df62d0b7b41c0b0dd1bcbd34a444b8a5a092`. Its Product run `33696875090`, rust-quality job `100467545647`, remains queued before checkout on explicit `ubuntu-24.04`; queued evidence is non-passing. The branch retains immutable PostgreSQL snapshot/source receipts, PK/unique/FK/CHECK evidence, FK reference behavior, PostgreSQL 18 FK validation/enforcement state, and the provider-independent `conceptweave-source-port` with explicit statement-timeout/row/byte/concurrency bounds, exact non-empty schema allowlists, caller cancellation, and typed source-disappearance/resource-limit outcomes. The current test-only lane tightens `observed_at_utc`; production validation must wait until that exact test head executes and demonstrates the intended RED. A concrete Rust read-only PostgreSQL adapter remains open.

## Central control-plane evidence

- `ContextualWisdomLab/.github#712` remains the owner for hosted-runner acquisition/queue health; current PR #5 and PR #6 Product jobs are still queued before checkout on explicit `ubuntu-24.04`, so no-op source churn is not an acceptable retry mechanism.
- `ContextualWisdomLab/.github#810` owns authoritative public non-fork Dependency Review availability/configuration. ConceptWeave must not substitute OSV/Trivy/Scorecard or fail open.
- `ContextualWisdomLab/.github#772` owns the solo-maintainer approval-governance defect. ConceptWeave does not self-approve or count model/bot reviews as an independent human approval.
- Central required-workflow/runtime repairs are evidence only after protected integration and unchanged ConceptWeave-head revalidation; predecessor runs do not transfer.

## Remaining P0 product gaps

1. **Source Observation adapter** — implement the new bounded source port with a real Rust read-only PostgreSQL adapter, immutable receipts, domains/enums/indexes/comments, hostile-input/resource bounds, cancellation/source-disappearance behavior, and a frozen anonymized GRC fixture.
2. **Observation-to-candidate provenance** — exact source receipt plus discovery method/proposal receipt for every generated candidate.
3. **Ontology induction** — deterministic observations plus `contextual-orchestrator` structured candidate generation for concepts, taxonomy and non-taxonomic relations.
4. **Semantic-layer induction** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts.
5. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection and bounded reasoning.
6. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, immutable releases, explicit supersession/publication receipts, transactional outbox and temporal history where warranted.
7. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale-decision protection and immutable publication receipt.
8. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie export.
9. **Client completion** — language-neutral supersession/publication receipt, relation/mapping/dimension/measure resolution, signature/provenance contract, research-backed match/align/explain, semantic query-plan API, and GRC reference fixtures.
10. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC and EA through published contracts only.
11. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility and multilingual cases.
12. **Observability/release** — shared OpenTelemetry bootstrap, structured security events, SBOM/provenance/signing, backup/restore and protected release evidence.

## DDD fitness gaps and invariants

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Adapters remain outside owned domain/client/source-observation contracts.
- Client Consumption consumes versioned public release contracts only, never generator-private implementation or persistence.
- Supersession validation is a Client Consumption contract; authority to issue the governed supersession/publication receipt remains Governance & Publication.
- Published releases are immutable; correction creates a distinct successor and preserves the predecessor.
- Source Observation preserves source evidence but does not infer semantic authority.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` remains catalog/governance/consumption plane, not ConceptWeave persistence.
- Consuming-product authorization/query execution stays downstream; ConceptWeave does not own foreign application tables.
- Future persistence uses descriptive two-or-more-word `snake_case` objects, 3NF by default, explicit item-level UPSERT/idempotency and immutable published releases.
