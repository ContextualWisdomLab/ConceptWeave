# Product / Technical Gap Baseline

**Snapshot:** 2026-09-02

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed from protected `main` yet.

## Active foundation slice — PR #1

| Area | Owner | Status | Evidence / action / next verification |
| --- | --- | --- | --- |
| Product boundary | ConceptWeave | ACTIVE_PR | PRD/TRD/ADR/context map define Semantic Model Engineering and CWL boundaries. Revalidate against the exact PR #1 head before merge. |
| Truth/publication lifecycle | Governance & Publication | ACTIVE_PR | Rust domain lifecycle defines Draft -> Proposed -> Validated -> Reviewed -> Published. Draft 2020-12 candidate schema enforces candidate shape and Published -> Authoritative consistency. |
| Rust baseline | ConceptWeave | ACTIVE_PR | Rust 1.98.0 workspace, unsafe forbidden, public docs required. |
| Quality gate | ConceptWeave | ACTIVE_PR | Product workflow checks exact checkout, fmt, Clippy, tests, docs, exact owned coverage, JSON contracts, lock freshness, and clean tree. Repository-owned Product was green on foundation head `5cd7d1de742fe34aa99900641cc8b124e7c65f9e`; predecessor results never transfer to a newer head. |
| Standards/research | ConceptWeave | ACTIVE_PR | Stable-vs-draft standards plus paper-by-paper Generation/Client/Bridge/cross-cutting capability and evaluation traceability. |
| Security/test/operability | ConceptWeave | ACTIVE_PR | Baselines added; published semantic truth is immutable with correction by superseding release; no production service claimed. |

## Active Client Consumption slice — PR #5 / Issue #3

PR #5 is intentionally stacked on PR #1 because the client reuses only the foundation's public evidence/truth/publication types. The canonical exact head is the live GitHub PR head; it is not duplicated as a self-referential constant in this file because editing this baseline itself changes that SHA. Check results are valid only for the unchanged live PR head.

| Gap | Owner | Status | Evidence | Action | Next verification |
| --- | --- | --- | --- | --- | --- |
| Offline release admission | Client Consumption | IMPLEMENTED_ACTIVE_PR | Test-first commits define and implement `SemanticReleaseClient`; authoritative use requires exact supported contract version + Published + Authoritative. | Keep deterministic and provider-independent. | Exact-head Rust tests/Clippy/docs/coverage. |
| Versioned semantic-release shape | Client Consumption / Governance & Publication seam | IMPLEMENTED_ACTIVE_PR | `contracts/semantic-release.schema.json` + fixtures; Rust `SemanticRelease` carries release/contract/ontology identity, truth/publication state, digest identity, provenance, unique concept IDs. | Stabilize compatibility/deprecation semantics before generated language bindings. | Exact-head AJV + Rust contract parity. |
| Declared digest identity | Client Consumption | PARTIAL | `ReleaseDigest` accepts only `sha256:<64 hex>`. | Add exact serialized-byte hashing and digest comparison before integrity is claimed; later add signature/provenance verification where release design warrants it. | Golden artifact mutation/tamper fixtures. |
| Release compatibility | Client Consumption | PARTIAL | Exact-version admission exists. | Add supported-version range/deprecation policy, malformed/unknown/older-supported/superseded cases. | Compatibility matrix fixtures. |
| Release diff / stale handling | Client Consumption | GAP | Research traceability maps OM4OV to explicit version-change semantics. | Implement typed `diff` and supersession/staleness outcomes without treating ordinary ontology matching as versioning. | Added/removed/changed entity golden fixtures. |
| Match / resolve / explain | Model Alignment + Client Consumption | GAP | OLaLa/LLMs4OM/MILA/KROMA research register defines retrieve/filter/match constraints. | Implement deterministic candidate retrieval first; optional LLM work only through `contextual-orchestrator`; never auto-authorize correspondences. | OAEI-style P/R/F1, retrieval recall, abstention and LLM-call-reduction evidence. |
| Query-plan contract | Client Consumption | GAP | Issue #3 requires plans without owning physical execution. | Define versioned semantic query-plan DTO and consuming-product ACL seam. | GRC golden round-trip with no cross-service SQL. |
| Consumer authorization | Downstream product / Keyverse boundary | EXTERNAL_OWNERSHIP | ConceptWeave client performs governance/compatibility admission only. | Keep tenant/purpose authorization and physical execution in consuming products. | Cross-tenant/purpose denial tests in each consumer. |

## Causal control-plane gap

Organization-required Security/SAST runner admission is owned by `ContextualWisdomLab/.github`, not by a leaf ConceptWeave source workaround. Central PR #1618 pins the affected required workflows from floating `ubuntu-latest` to explicit `ubuntu-24.04` with a regression contract and has demonstrated Security Scan and SAST Semgrep success on its own exact head. ConceptWeave must continue to require fresh exact-head central evidence after that control-plane repair lands; no predecessor result, no-op retrigger, or governance bypass is acceptable.

## Remaining P0 product gaps

1. **Source Observation vertical** — relational schema snapshot contract, real PostgreSQL introspection adapter, immutable digest/location receipts, hostile-input bounds.
2. **Ontology induction** — deterministic observations plus contextual-orchestrator structured candidate generation for concepts, taxonomy, and non-taxonomic relations.
3. **Semantic-layer induction** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts.
4. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection, bounded reasoning.
5. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, releases, transactional outbox, bitemporal history where applicable.
6. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale decision protection, immutable publication receipt.
7. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie semantic-model export.
8. **Client completion** — byte-level integrity verification, compatibility/deprecation, diff/stale handling, match/resolve/explain/query-plan, generated bindings only when contract stability warrants them.
9. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC, and EA through published contracts only.
10. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility, multilingual cases.
11. **Secure external research** — SearXNG discovery and safe source fetch through the correct CWL egress boundary for ontology grounding, never search snippets as truth.
12. **Observability** — shared CWL OpenTelemetry import/bootstrap contract, detailed structured logs, SIEM security-event projection where applicable.
13. **Release** — SBOM, provenance, signed artifacts, migration/backup/restore evidence, versioned changelog, protected release pipeline.

## DDD fitness gaps and invariants

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Adapters must remain outside `conceptweave-domain`.
- Client Consumption may consume versioned public contracts but not generator-private implementation.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` must not become ConceptWeave persistence, and ConceptWeave must not become an SDP clone.
- Consuming-product authorization/query execution stays downstream; ConceptWeave does not own foreign application tables.
- External forks/tools can be optional adapters but are not CWL-owned product authorities.
- Future persistence uses descriptive two-or-more-word `snake_case` objects, 3NF by default, explicit item-level UPSERT/idempotency contracts, and immutable published releases.
