# Product / Technical Gap Baseline

**Snapshot:** 2026-09-01

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed.

## Active foundation slice

| Area | Status | Evidence / next action |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define Semantic Model Engineering and CWL boundaries. |
| Truth/publication lifecycle | ACTIVE_PR | Rust domain lifecycle defines Draft -> Proposed -> Validated -> Reviewed -> Published. Draft 2020-12 JSON Schema enforces candidate shape, non-blank evidence identities, and Published -> Authoritative consistency; lifecycle history/pre-Reviewed publication is not a JSON-Schema responsibility. A test-first regression currently requires Reviewed -> Published to fail if required evidence is absent before this slice can be merge-ready. |
| Rust baseline | ACTIVE_PR | Rust 1.98.0 workspace, unsafe forbidden, public docs required. |
| Quality gate | ACTIVE_PR | Product workflow for fmt/clippy/tests/docs/coverage/Draft-2020-12 schema fixtures/lock/clean-tree; exact current-head hosted execution remains required. |
| Standards/research | ACTIVE_PR | Stable-vs-draft standards plus paper-by-paper Generation/Client/Bridge/cross-cutting capability and evaluation traceability. |
| Security/test/operability | ACTIVE_PR | Baselines added; published semantic truth is specified as immutable with correction by superseding release; no production service claimed. |

## P0 product gaps after foundation

1. **Source Observation vertical** — relational schema snapshot contract, real PostgreSQL introspection adapter, immutable digest/location receipts, hostile-input bounds.
2. **Ontology induction** — deterministic observations plus contextual-orchestrator structured candidate generation for concepts, taxonomy, and non-taxonomic relations.
3. **Semantic-layer induction** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts.
4. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection, bounded reasoning.
5. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, releases, transactional outbox, bitemporal history where applicable.
6. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale decision protection, immutable publication receipt.
7. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie semantic-model export.
8. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC, and EA through published contracts only.
9. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility, multilingual cases.
10. **Secure external research** — SearXNG discovery and safe source fetch through the correct CWL egress boundary for ontology grounding, never search snippets as truth.
11. **Observability** — shared CWL OpenTelemetry import/bootstrap contract, detailed structured logs, SIEM security-event projection where applicable.
12. **Release** — SBOM, provenance, signed artifacts, migration/backup/restore evidence, versioned changelog, protected release pipeline.

## DDD fitness gaps

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Adapters must remain outside `conceptweave-domain`.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` must not become ConceptWeave persistence, and ConceptWeave must not become an SDP clone.
- External forks/tools can be optional adapters but are not CWL-owned product authorities.
