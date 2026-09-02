# Product / Technical Gap Baseline

**Snapshot:** 2026-09-02

## Shipped on protected `main`

Only the repository bootstrap README exists before the foundation PR. No production capability is claimed.

## Active foundation slice — PR #1

The exact PR head is the live GitHub branch head; check evidence is valid only for that unchanged SHA. Current foundation head `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` has terminal repository-owned Product and SAST success. The central Security Scan is not complete because its Dependency Review lane has not produced authoritative terminal evidence.

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
| Deterministic ordering | IMPLEMENTED_PENDING_CHECKS | Tables sort by exact `(schema_name, table_name)`, columns by one-based source ordinal then exact name, and constraints by exact source constraint name. |
| Fail-closed metadata | IMPLEMENTED_PENDING_CHECKS | Unicode-whitespace-only required fields, zero ordinals, duplicate table coordinates, duplicate column names/ordinals, empty/duplicate ordered constraint coordinates, duplicate constraint names, unknown local coordinate columns, blank CHECK definitions, and foreign-key arity mismatch are rejected with typed errors. |
| Snapshot provenance | IMPLEMENTED_PENDING_CHECKS | Source connection reference, canonical lowercase `sha256:<64 hex>` snapshot identity, extractor revision, observation time, and verified typed table/column/constraint locations are retained. Candidate-level discovery-method/proposal provenance remains open. |
| PK/unique/FK relationship evidence | IMPLEMENTED_PENDING_CHECKS | Composite PK/unique/FK evidence preserves deterministic table binding, local-column existence, exact cross-schema referenced coordinates, and column order. Test-first commit `91f6dc57ee6f522b4154c878daa2c27eddbe3059` specified exact foreign-key `ON UPDATE`/`ON DELETE`, match type, deferrability/initial timing, plus explicit absence when source behavior was not observed. Production retains typed reference behavior without deriving defaults. |
| PostgreSQL 18 FK validation/enforcement evidence | IMPLEMENTED_PENDING_CHECKS | Test-first commit `350b7c11c801f7356e0e602513bb54f42e90d0ae` requires exact `convalidated`/`conenforced` preservation including explicit `false`, and requires absence to remain `None` rather than fabricate PostgreSQL defaults. Production commit `4df2fd7b5acbbfd9406015daf977ea13f7c0b866` adds immutable optional validation/enforcement state to `ForeignKeyObservation`; CHANGELOG and architecture evidence are reconciled on later heads. Hosted Product evidence is still required on the final unchanged documentation head. |
| PostgreSQL 18 CHECK evidence | IMPLEMENTED_PENDING_CHECKS | Test-first commit `416d012676edf0dbe03670e8fdec7bbb28b0f0fd` specified exact CHECK definition plus `validated`, `enforced`, and `no_inherit` status and required blank definitions to fail closed. Production commit `098972ae64ee754f1f0e21b72fcb9832cbc0fddc` added `CheckConstraintObservation` and table binding. The RED-to-production compare modified only `crates/conceptweave-observation/src/lib.rs` (+76/-5). CHECK expression text is retained as evidence without guessing ordered expression-column coordinates. This matches PostgreSQL 18 `pg_constraint` (`conenforced`, `convalidated`, `connoinherit`, `conbin`) and its recommendation to use `pg_get_constraintdef()` to reconstruct CHECK definitions; PostgreSQL 18 added `NOT ENFORCED` support for CHECK and foreign-key constraints. |
| Source Observation port | IMPLEMENTED_PENDING_CHECKS | New Rust workspace crate `conceptweave-source-port` defines fail-closed positive statement-timeout, row, byte, and concurrency budgets; an explicit non-empty exact schema allowlist; a stable non-credential source reference; caller cancellation; and typed cancellation/source-disappearance/timeout/resource-limit outcomes. Test-first commit `7cafba262aca070fa6bdccc95284641436a81224` required the contract before production commit `016b0aff5a6866d6071e02dd1afa6e116a8ce92b`. The port deliberately contains no driver, credential resolution, SQL, semantic inference, or snapshot fabrication. |
| PostgreSQL adapter | OPEN | No live adapter is claimed. Next implementation must implement the Source Observation port with read-only PostgreSQL catalog access, populate the existing typed contracts including FK reference behavior plus exact validation/enforcement state, preserve exact source evidence, enforce all request bounds and cancellation, and avoid direct foreign application-table coupling. Domains/enums/indexes, remaining comments/type details, source disappearance during capture, and a frozen GRC fixture remain open. |
| Verification | WAITING_EXACT_HEAD | Hosted Product evidence for the latest implementation/documentation head must execute on that unchanged SHA. Predecessor workflow results are non-transferable. Local Rust validation is not claimed because the available runtime did not expose `cargo`/`rustc`/`rustfmt`. |

### PostgreSQL 18 authoritative references

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2025). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: system information functions and operators*. https://www.postgresql.org/docs/18/functions-info.html

## Causal control-plane state

`ContextualWisdomLab/.github` PR #1618 is merged and repaired the prior floating runner selector at the owning control plane. Current same-workflow evidence shows explicit `ubuntu-24.04` jobs can still be delayed before runner assignment; `.github#712` owns runner-acquisition RCA. `.github#810` separately owns the public non-fork Dependency Review availability/configuration incident. OSV, Trivy, Scorecard, SAST, and model reviews are not substitutes for authoritative Dependency Review.

The active organization ruleset still requires one approving review on the default branch while declaring no required reviewers; `.github#772` owns the solo-maintainer governance repair. No self-approval, administrator bypass, or gate weakening is accepted here.

The central review scheduler already has a distinct stacked-PR dispatch lane and `ORG_SWEEP_STACKED_REVIEW_DISPATCH_LIMIT`; `.github#1219` owns measured throughput/fairness acceptance rather than leaf workflow duplication. ConceptWeave PR #5/#6 remain live stacked canaries and require exact-head OpenCode evidence before that control-plane gap can be called complete.

## P0 product gaps after current slices

1. **PostgreSQL Source Observation adapter** — implement the new bounded `conceptweave-source-port` against a real read-only PostgreSQL driver, enforce timeout/cancellation/row/byte/concurrency limits, surface source disappearance without partial success, emit immutable extractor receipts, observe domain/enum/index/comment evidence, and prove deterministic replay with a frozen anonymized GRC reference fixture; populate implemented PK/unique/FK/CHECK/reference-behavior/validation/enforcement contracts rather than duplicate relationship semantics in the adapter.
2. **Observation-to-candidate provenance** — exact source location plus discovery method/proposal receipt so every candidate remains traceable to one immutable observation snapshot.
3. **Ontology induction** — deterministic observations plus contextual-orchestrator structured candidate generation for concepts, taxonomy, and non-taxonomic relations.
4. **Semantic-layer induction** — dimensions, measures, grain, units, relationships, and physical mappings with deterministic calculation contracts.
5. **Validation engine** — RDF/OWL/SKOS/SHACL publication validation, consistency checks, duplicate/conflict detection, bounded reasoning.
6. **Governance persistence** — PostgreSQL 3NF candidates, evidence, validation receipts, review decisions, releases, transactional outbox, bitemporal history where applicable.
7. **Review workflow** — Keyverse tenant/role/purpose context, steward review, maker-checker where required, stale decision protection, immutable publication receipt.
8. **Publication adapters** — OWL/RDFS/SKOS/SHACL/JSON-LD and version-bound Apache Ossie semantic-model export.
9. **Client Consumption** — stacked PR #5 / Issue #3 owns offline release admission, integrity, compatibility, diff/match/resolve/explain/query-plan contracts; its current exact head must be re-read before any owner-side write because a concurrent writer may be active.
10. **CWL integration** — `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, GRC, and EA through published contracts only.
11. **Evaluation harness** — ontology-learning/matching golden fixtures, structural/semantic metrics, human-reviewed cases, replay reproducibility, multilingual cases.
12. **Observability/release** — shared OpenTelemetry import/bootstrap, structured security events, SBOM, provenance, signed artifacts, backup/restore evidence, and protected release pipeline.

## DDD fitness gaps

- No generic `utils/helpers/services/common` domain buckets are permitted.
- Source-access budgets, allowlists, cancellation, and failure semantics belong to `conceptweave-source-port`; concrete PostgreSQL driver/credential/catalog behavior belongs to an adapter outside domain and immutable observation contracts.
- Adapters must remain outside `conceptweave-domain`, `conceptweave-observation`, and `conceptweave-source-port`.
- Source Observation preserves evidence and ordering; it does not infer semantics or claim source-system authority.
- Source key/relationship/CHECK observations are source facts only; they must not be promoted to semantic relationships or rules without candidate generation, validation, and governance.
- Client Consumption may depend only on versioned public release/domain contracts, never generator-private classes or persistence.
- Foreign product DTOs require Anti-Corruption Layers.
- `semantic-data-portal` must not become ConceptWeave persistence, and ConceptWeave must not become an SDP clone.
- Consuming-product authorization and physical query execution stay downstream.
- External forks/tools can be optional adapters but are not CWL-owned product authorities.
