# Product / Technical Gap Baseline

**Snapshot:** 2026-09-02

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed.

## Active foundation slice — PR #1

The exact PR head is the live GitHub branch head; check evidence is valid only for that unchanged SHA. Current head `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` has terminal repository-owned Product and SAST success. The central Security Scan is not complete because its Dependency Review lane has not produced authoritative terminal evidence.

| Area | Status | Evidence / action / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define Semantic Model Engineering and CWL boundaries. |
| Truth/publication lifecycle | ACTIVE_PR | Rust domain lifecycle defines Draft -> Proposed -> Validated -> Reviewed -> Published with authorization required at steward/publication boundaries; candidate JSON Schema enforces public structural shape and Published -> Authoritative consistency. |
| Rust baseline | ACTIVE_PR | Rust 1.98.0 workspace, unsafe forbidden, public docs required. |
| Quality gate | ACTIVE_PR | Product requires exact checkout, fmt, Clippy, tests, rustdoc, 100% owned line/function/region/source-branch coverage, Draft-2020-12 schema fixtures, lock freshness, and clean tree. |
| Standards/research | ACTIVE_PR | Stable-vs-draft standards plus paper-by-paper Generation/Client/Bridge/cross-cutting capability and evaluation traceability. |
| Security/test/operability | BLOCKED_EXTERNAL | Product and SAST are green; central Dependency Review availability/runner evidence remains unresolved under `.github#810` / `.github#712`. No leaf bypass is permitted. |

## Active Source Observation slice — PR #6 / Issue #2

PR #6 is stacked on the foundation and advances the first Generation-side commercialization gap without adding a database connection prematurely.

| Contract | Exact-head state | Evidence / action / next verification |
| --- | --- | --- |
| Immutable relational snapshot | IMPLEMENTED_PENDING_CHECKS | `conceptweave-observation` defines `PostgresSchemaSnapshot`, `TableObservation`, and `ColumnObservation` as private-field Rust contracts. |
| Identifier preservation | IMPLEMENTED_PENDING_CHECKS | Exact schema/table/column text is preserved; no lowercasing, fuzzy matching, or quoted-identifier normalization occurs. Same table names in different schemas remain distinct. |
| Deterministic ordering | IMPLEMENTED_PENDING_CHECKS | Tables sort by exact `(schema_name, table_name)` and columns by one-based source ordinal then exact name. |
| Fail-closed metadata | IMPLEMENTED_PENDING_CHECKS | Unicode-whitespace-only required fields, zero ordinals, duplicate table coordinates, duplicate column names, and duplicate ordinals are rejected with typed errors. |
| Snapshot provenance | IMPLEMENTED_PENDING_CHECKS | Source connection reference, canonical lowercase `sha256:<64 hex>` snapshot identity, extractor revision, and observation time are retained. Test-only head `4e961c1ad221e0b0b71ae113485bddf42be8e561` established the contract against production that accepted any nonblank digest; production commit `47f21bfdb4657048b98ca719fa3ce14c7237d598` added the minimal canonical digest validator. Candidate-level discovery method and exact observation-location binding remain open. |
| PostgreSQL adapter | OPEN | No live adapter is claimed. Next implementation must be read-only and bounded, observe constraints/keys/types/comments safely, and preserve source evidence without direct foreign application-table coupling. |
| Verification | WAITING_EXACT_HEAD | The prior Product run for `4e961c1…` remained queued before checkout. Production and documentation commits changed the head, so predecessor workflow results are non-transferable; the resulting exact PR head must receive fresh Product/security/SAST/review evidence before this slice can be called GREEN. |

## Causal control-plane state

`ContextualWisdomLab/.github` PR #1618 is merged and repaired the prior floating runner selector at the owning control plane. Current same-workflow evidence shows several explicit `ubuntu-24.04` security jobs can run while Dependency Review can still remain queued; `.github#712` owns runner-acquisition RCA. `.github#810` separately owns the public non-fork Dependency Review availability/configuration incident. OSV, Trivy, Scorecard, SAST, and model reviews are not substitutes for authoritative Dependency Review.

The active organization ruleset still requires one approving review on the default branch while declaring no required reviewers; `.github#772` owns the solo-maintainer governance repair. No self-approval, administrator bypass, or gate weakening is accepted here.

## P0 product gaps after current slices

1. **Source Observation adapter** — real PostgreSQL introspection behind a port, immutable bounded receipts, PK/unique/FK/domain/enum/index/comment evidence, hostile-input/resource bounds, cancellation, and source-disappearance behavior.
2. **Observation-to-candidate provenance** — exact source location plus discovery method/proposal receipt so every candidate remains traceable to one immutable observation snapshot.
3. **Ontology induction** — deterministic observations plus contextual-orchestrator structured candidate generation for concepts, taxonomy, and non-taxonomic relations.
4. **Semantic-layer induction** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts.
5. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection, bounded reasoning.
6. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, releases, transactional outbox, bitemporal history where applicable.
7. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale decision protection, immutable publication receipt.
8. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie semantic-model export.
9. **Client Consumption** — stacked PR #5 / Issue #3 owns offline release admission, integrity, compatibility, diff/match/resolve/explain/query-plan contracts; its current exact head must be re-read before any owner-side write because a concurrent writer is active.
10. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC, and EA through published contracts only.
11. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility, multilingual cases.
12. **Observability/release** — shared OpenTelemetry import/bootstrap, structured security events, SBOM, provenance, signed artifacts, backup/restore evidence, and protected release pipeline.

## DDD fitness gaps

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Adapters must remain outside `conceptweave-domain` and `conceptweave-observation`.
- Source Observation preserves evidence and ordering; it does not infer semantics or claim source-system authority.
- Client Consumption may depend only on versioned public release/domain contracts, never generator-private classes or persistence.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` must not become ConceptWeave persistence, and ConceptWeave must not become an SDP clone.
- Consuming-product authorization and physical query execution stay downstream.
- External forks/tools can be optional adapters but are not CWL-owned product authorities.
