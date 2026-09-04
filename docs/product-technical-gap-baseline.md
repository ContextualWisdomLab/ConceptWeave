# Product / Technical Gap Baseline

**Snapshot:** 2026-09-04

This branch is the Source Observation child of Foundation PR #1. Exact PR/run coordinates are evidence snapshots, never mutable production dependencies. Protected/live GitHub state is authoritative when it advances after this snapshot.

## Stack authority

- Protected/default `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; no ConceptWeave release exists.
- Foundation PR #1 advanced by documentation-only live-state repair from `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` to `447aa0723abd7b582b9acc478ed90238d0d59214`. The delta between those heads is only this gap-baseline file. Current Foundation Product `33835025486`, job `100905656541`, is queued before runner assignment; predecessor Product/SAST success and Security failure remain historical evidence only.
- Source Observation PR #6 pre-restack head `1fdfb3af14c126c270861eb541e9e57d47418bb8` is Draft. Product `33834639272`, job `100904527699`, remains queued before runner assignment and is superseded for acceptance by this Foundation-adoption restack.
- Client PR #5 has independently adopted the same Foundation documentation delta through a non-force restack; its semantic-release client remains a sibling bounded context, not Source Observation implementation.

## Source Observation capability status

| Contract | Status | Evidence / invariant | Next verification |
| --- | --- | --- | --- |
| Immutable relational snapshot | IMPLEMENTED_PENDING_CURRENT_HEAD | `conceptweave-observation` owns private-field `PostgresSchemaSnapshot`, table/column observations and deterministic immutable evidence. | Exact-head Rust tests/Clippy/rustdoc/coverage after restack. |
| Exact identifier preservation | IMPLEMENTED_PENDING_CURRENT_HEAD | Schema/table/column/constraint identifiers retain exact source text; no case folding, fuzzy matching or quoted-identifier normalization. | Quoted/case/path-delimiter edge fixtures. |
| Deterministic ordering | IMPLEMENTED_PENDING_CURRENT_HEAD | Tables sort by exact `(schema_name, table_name)`, columns by source ordinal then exact name, constraints by exact source name. | Golden replay equality. |
| Fail-closed metadata | IMPLEMENTED_PENDING_CURRENT_HEAD | Blank required fields, zero ordinals, duplicate coordinates/names/ordinals, malformed constraint coordinates, unknown local columns, blank CHECK definitions and FK arity mismatch are rejected with typed errors. | Full edge coverage on restacked head. |
| Snapshot digest / location receipts | IMPLEMENTED_PENDING_CURRENT_HEAD | Canonical lowercase `sha256:<64 hex>` snapshot identity plus typed table/column/constraint location receipts are retained. RFC 6901 escaping protects delimiter-bearing identifiers. | Digest/location edge tests and rustdoc. |
| UTC observation provenance | REPAIRED_PENDING_CURRENT_HEAD | Product `33696875090`, job `100467545647`, executed predecessor `2817df62d0b7b41c0b0dd1bcbd34a444b8a5a092`, passed CI/fmt/Clippy and failed because `observed_at_utc="time"` was accepted. `e27ffaf4a40d746781b8012e9fe71467e7e6511f` added explicit UTC `Z`, Gregorian date/clock and optional fractional-second validation with malformed zones/offsets/dates/clocks failing closed. | Exact-head Product after restack. |
| PK / unique / FK / CHECK evidence | IMPLEMENTED_PENDING_CURRENT_HEAD | Composite keys preserve ordered exact coordinates; FK reference behavior preserves update/delete actions, match type and deferrability; PostgreSQL 18 `convalidated`/`conenforced` remain explicit optional evidence; CHECK preserves exact definition, validated/enforced/no-inherit without inferring expression-column semantics. | Exact-head tests plus replay fixture. |
| Bounded Source Observation port | IMPLEMENTED_PENDING_CURRENT_HEAD | `conceptweave-source-port` owns positive statement-timeout/row/byte/concurrency budgets, exact non-empty schema allowlist, caller cancellation and typed source/resource failures. Credential resolution and catalog SQL remain adapter-local. | Port contract tests on restacked head. |
| Opaque source registry identity | INTENTIONAL_RED_PENDING | `ObservationRequest` accepts only ≤128-byte lowercase multiword `snake_case` registry keys and rejects DSNs/credential-shaped material, one-word identifiers, mixed case, hyphens and malformed underscores. `PostgresSchemaSnapshot::new` still checks `source_connection_key` only for nonblank and `source_receipt()` copies it into immutable `source_id`. | Execute the existing cross-boundary test to semantic RED, then add the smallest production validator repair and require exact-head GREEN. |
| Concrete PostgreSQL adapter | GAP_AFTER_CURRENT_RED | No live adapter is claimed. ADR 0004 stays Proposed. | Maintained Rust driver, read-only enforcement, adapter-local registry/credential resolution, explicit schema allowlist, timeout/cancellation/row/byte/concurrency budgets, complete-or-fail capture and frozen anonymized GRC-shaped replay. |

## Current registry-identity TDD lineage

Test-first `c9af2255fb721b8e05e608e6b2525017b1f59151` added `source_registry_identity.rs`, requiring immutable snapshot provenance to reject DSN/credential-shaped and malformed registry identities while accepting `grc_readonly_connection`. Product `33760465773`, job `100665220457`, eventually acquired hosted runner `1001655745` and checked out that exact head, but failed at `cargo fmt --all --check` before Clippy or the intended semantic test.

`0ae2ac357b8d83cd0b26d54031f3518c759f0f61` repaired the test-file formatting. A subsequent whole-file connector write accidentally introduced unrelated edits in `2c146a1da42b28dcf5ad2a045197414d48b1e3b4`; it was immediately neutralized without force-push by forward commit `43693f1850e083c7fb38d119d78205e6a56d243f`, whose tree compares exactly equal to `0ae2ac...` (`files=[]`). No accidental semantic delta remains.

`1fdfb3af14c126c270861eb541e9e57d47418bb8` then applies exactly one intended production-source hunk: rustfmt wrapping of the pre-existing UTC `split_once('.')` match guard. Production registry-key validation remains unchanged. The Foundation restack changes documentation ancestry only; it does not authorize skipping the real registry semantic RED.

## Authoritative Source Observation references

- Klyne, G., & Newman, C. (2002). *Date and time on the Internet: Timestamps* (RFC 3339). Internet Engineering Task Force. https://www.rfc-editor.org/rfc/rfc3339
- Sharma, U., & Bormann, C. (2024). *Date and time on the Internet: Timestamps with additional information* (RFC 9557). Internet Engineering Task Force. https://www.rfc-editor.org/rfc/rfc9557
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Date/time types*. https://www.postgresql.org/docs/18/datatype-datetime.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2025). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html

## Central control-plane evidence

Protected central source is `.github/main@07d9ec23fb265c76539d23249e1dfa124ea7b23b` at this snapshot; this is evidence, not a ConceptWeave dependency.

- `.github#810` owns authoritative Dependency Review availability. OSV/Trivy/Scorecard/SAST are not substitutes and 403 cannot be treated as success.
- `.github#712/#1531` own selective/intermittent runner admission and review/queue amplification.
- `.github#1796` has test-first Draft #1821 `test/1796-org-sweep-queue-owner@9c79cf775ad6a125a94dedcae9683c20a65a0339`, separating organization-sweep queue inventory from target-repository exact-head coalescing. Production central source remains unchanged until its RED executes.
- `.github#1822@7a5cc1b1c43946d210405cd051ae629ff2c44966` is a separate Draft fix for documented `CoalescingRefused` safe-no-op behavior; other exceptions remain fail closed.
- Fresh central queued inventory reached 1,906 runs. Aggregate backlog movement is diagnostic only; acceptance still requires actual runner assignment, exact checkout and terminal evidence on unchanged current heads.

## P0 product gaps after registry identity

1. Implement the concrete bounded read-only PostgreSQL Source Observation adapter and deterministic frozen GRC-shaped replay fixture.
2. Bind every semantic candidate to exact immutable source receipts plus discovery/proposal evidence.
3. Add deterministic ontology discovery for concepts, taxonomy and non-taxonomic relations; relational facts remain evidence, not semantic authority.
4. Add semantic-layer discovery for dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts.
5. Route all optional production LLM proposal/alignment behavior through released `contextual-orchestrator`; output remains proposed/inferred until steward validation/publication.
6. Add deterministic RDF/OWL/SKOS/SHACL and semantic-layer validation, conflict/duplicate checks and bounded reasoning.
7. Add governance persistence, Keyverse-backed review context, immutable publication/supersession receipts and versioned publication adapters.
8. Complete Client Consumption cross-language contracts, provenance/signature, relation/mapping/dimension/measure resolution, research-backed match/align/explain and semantic query-plan seams.
9. Prove multilingual/evaluation, observability/recovery, package/SBOM/provenance/signing, reproducibility and rollback before immutable release.

## DDD and release invariants

- Source-access budgets, schema allowlists, opaque registry references, cancellation and failure semantics belong to `conceptweave-source-port`; concrete PostgreSQL driver/credentials/catalog SQL belong to an adapter ACL.
- Adapters remain outside `conceptweave-domain`, `conceptweave-observation` and `conceptweave-source-port`.
- Source Observation preserves evidence and ordering; it does not infer semantic authority or duplicate source-system business truth.
- `semantic-data-portal` remains catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- No source copying, cross-service SQL or mutable foreign-head dependency.
- Published semantic truth is immutable; correction creates a distinct successor plus supersession evidence.
- Release requires exact protected head, version/CHANGELOG/tag/package/immutable `semantic_release`, SBOM, provenance, reproducibility and rollback evidence.
