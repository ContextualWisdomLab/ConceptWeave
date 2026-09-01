# Product / Technical Gap Baseline

**Snapshot:** 2026-09-02

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed.

## Active foundation slice — PR #1

The exact PR head is the live GitHub branch head; check evidence is valid only for that unchanged SHA. The predecessor head `5cd7d1de742fe34aa99900641cc8b124e7c65f9e` reached terminal repository-owned Product success (exact checkout, fmt, Clippy, tests, rustdoc, exact owned coverage, JSON contract, lock freshness, clean tree). This baseline update intentionally creates a newer documentation-only head so all checks must be re-established rather than transferred.

| Area | Status | Evidence / action / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define Semantic Model Engineering and CWL boundaries. Revalidate on the new exact head. |
| Truth/publication lifecycle | ACTIVE_PR | Rust domain lifecycle defines Draft -> Proposed -> Validated -> Reviewed -> Published with explicit governance authorization at steward/publication boundaries; candidate JSON Schema enforces public structural shape and Published -> Authoritative consistency. The earlier missing-evidence and branch-coverage defects were repaired and proven by predecessor exact-head Product success. |
| Rust baseline | ACTIVE_PR | Rust 1.98.0 workspace, unsafe forbidden, public docs required. |
| Quality gate | ACTIVE_PR | Product workflow requires exact checkout, CI-contract validation, fmt, Clippy, tests, rustdoc, 100% owned line/function/region/source-branch coverage, Draft-2020-12 schema fixtures, lock freshness, and clean tree. Fresh exact-head execution is required after this documentation change. |
| Standards/research | ACTIVE_PR | Stable-vs-draft standards plus paper-by-paper Generation/Client/Bridge/cross-cutting capability and evaluation traceability. |
| Security/test/operability | ACTIVE_PR | Baselines added; published semantic truth is immutable with correction by superseding release; no production service is claimed. |

## Causal control-plane repair

`ContextualWisdomLab/.github` PR #1618 has now merged. It repaired the organization-required Security Scan and SAST Semgrep runner selectors at the owning control plane by replacing the observed-starved floating `ubuntu-latest` selectors with explicit `ubuntu-24.04`, while preserving scanners, permissions, thresholds, action pins, exact-head validation, and fail-closed behavior. The central repair demonstrated Security Scan and SAST success on its own exact head before merge.

The older ConceptWeave required-workflow runs on `5cd7d1de742fe34aa99900641cc8b124e7c65f9e` were created before that central merge and remain queued; their workflow snapshot cannot be treated as repaired in place. This new ConceptWeave head exists partly to cause a fresh PR synchronize event so required workflows are instantiated from the repaired central source. Do not bypass or transfer predecessor results.

## P0 product gaps after foundation

1. **Source Observation vertical** — relational schema snapshot contract, real PostgreSQL introspection adapter, immutable digest/location receipts, hostile-input bounds.
2. **Ontology induction** — deterministic observations plus contextual-orchestrator structured candidate generation for concepts, taxonomy, and non-taxonomic relations.
3. **Semantic-layer induction** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts.
4. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection, bounded reasoning.
5. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, releases, transactional outbox, bitemporal history where applicable.
6. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale decision protection, immutable publication receipt.
7. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie semantic-model export.
8. **Client Consumption** — stacked PR #5 / Issue #3 currently adds offline release admission and a versioned semantic-release contract; byte-level integrity verification, compatibility/deprecation, release diff/stale handling, match/resolve/explain/query-plan remain open.
9. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC, and EA through published contracts only.
10. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility, multilingual cases.
11. **Secure external research** — SearXNG discovery and safe source fetch through the correct CWL egress boundary for ontology grounding, never search snippets as truth.
12. **Observability** — shared CWL OpenTelemetry import/bootstrap contract, detailed structured logs, SIEM security-event projection where applicable.
13. **Release** — SBOM, provenance, signed artifacts, migration/backup/restore evidence, versioned changelog, protected release pipeline.

## DDD fitness gaps

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Adapters must remain outside `conceptweave-domain`.
- Client Consumption may depend only on versioned public release/domain contracts, never generator-private classes or persistence.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` must not become ConceptWeave persistence, and ConceptWeave must not become an SDP clone.
- Consuming-product authorization and physical query execution stay downstream.
- External forks/tools can be optional adapters but are not CWL-owned product authorities.
