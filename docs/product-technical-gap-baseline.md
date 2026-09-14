# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed history through exact `7c65090c2a90ba11dff575fde15fde37838795f6` remains byte-for-byte in `docs/archive/product-technical-gap-baseline-through-7c65090c.md`; later decisions remain in focused `docs/doctoring/` records. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack and single-writer boundary

- Protected/default ConceptWeave `main` remains repository acceptance authority; the active Source Observation branch is not release authority.
- #6 remains the upstream product stack owner; #45 remains its Draft v3 representation child.
- #46 `codex/pr6-v3-index-evidence` is the active Draft Source Observation writer stacked on #45.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.
- #45 and #6 must not duplicate, partially cherry-pick, or independently reimplement this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## Source-repaired PostgreSQL 18 attachment evidence

The frozen v3 snapshot owns relation/type/index/constraint coordinates and raw relation-scoped index evidence. Domain-separated successors preserve declarative relation partitioning, index-partition topology, NOT NULL inheritance coherence, rowtype compatibility, and attached-index definition semantics without rewriting frozen predecessor digest meaning.

Retained source-repaired checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` structure, mapped simple-column identity, qualified-name historical key collation, catalog-exact key/expression/relation-`Var` collation identity, source-database encoding binding for those catalog identities, resolved operator-family identity, exclusion operator/procedure/strategy semantics, relation-partition rowtype mapping, structured raw `pg_attribute.atttypmod`, canonical expression/partial-predicate representation, complete stable relation-`Var` equality evidence, and the composed node-schema-plus-relation-Var whole-tree proof. Detailed lineage remains in focused doctoring records.

### Rowtype and expression proof boundary

`RelationPartitionTypeModifierSnapshot` keeps exact raw signed `pg_attribute.atttypmod` for every bounded column and compares direct parent/child modifiers by stable column name. The concrete PostgreSQL adapter and live rowtype differential remain transport work after representation acceptance.

`IndexExpressionSemanticsSnapshot` preserves canonical expression/predicate trees because PostgreSQL `CompareIndexInfo()` maps child Vars and then applies internal `equal()`. Historical node-schema v1 proves the modeled non-Var `FuncExpr`/`OpExpr` fields, relation-Var v1 proves complete equality state for every canonical relation-column leaf, and `IndexExpressionRelationVarNodeSchemaSnapshot` composes those exact proofs without weakening node-schema v2. Whole-row Vars, typed `Const`, and unsupported nodes remain fail closed until complete PostgreSQL equality schemas exist.

Concrete semantic-expression extraction and live attachment differential remain transport work and must not be pulled ahead of terminal exact-head acceptance of the representation slice.

### PostgreSQL collation catalog-row identity and database-encoding binding

Review `5203297768` found that the historical key-collation check was not catalog-exact. `QualifiedCollationName` carries only namespace/name, but PostgreSQL 18 declares `pg_collation_name_enc_nsp_index` UNIQUE over `(collname, collencoding, collnamespace)`. `CompareIndexInfo()` compares the resolved per-key collation OIDs. Distinct `pg_collation` catalog rows can therefore share a namespace/name while differing in `collencoding`, even though only rows compatible with the current database encoding are usable in that database.

Behavioral compile RED `db761588b9e3a79da4cd6f2839b756ce06992601` fixed the per-key catalog-row distinction. Production `d1d2423e2f20c328f459bf912f0e7c4e2c4d44ae` added `CollationCatalogIdentity`, `IndexKeyCollationIdentityObservation`, and `IndexPartitionCollationIdentitySnapshot`; export `255f414327d95a680c61e96c80d553f8e07e1137` and focused coverage `0279cb46e486e8a3c74cb446915ad9c95bc53a61` preserve all predecessor digests unchanged. Decision record: `docs/doctoring/postgresql-index-partition-collation-catalog-identity-integrity.md`.

Review `5203347945` then found the same identity collapse in `CanonicalExpressionValue::Collation` and `IndexExpressionRelationVarObservation.collation`. Behavioral compile RED `68576447e8f673098d9a94a1fddacc876db1abe0` and production `d67f454d796eb9a5d6b01a4dfac8370c90051c45` added `IndexExpressionCollationIdentitySnapshot`; export `ab5e50f772afb9c9db0ee07298db428f613f1d35` exposes it. The composition rebound-validates the exact whole-tree and key-collation proofs, enumerates every modeled expression/predicate collation occurrence, requires complete relation-`Var` collation evidence, and compares full namespace/name/raw-`collencoding` identity across direct attached edges. Decision record: `docs/doctoring/postgresql-expression-collation-catalog-identity-composition-integrity.md`.

Review `5203629766` found the next source-integrity seam: equality of raw `collencoding` is not proof that the supplied catalog row can be used in the observed database. PostgreSQL 18 creates/uses collations with `collencoding = -1` or the exact backend encoding of the current database; client-only encoding IDs are not valid `pg_database.encoding` values. A faulty extractor could previously supply the same incompatible encoding for both sides and all catalog-identity equality checks would pass.

Source RED `1abd33454fda57f8add1d47b81c4ce3a5305e59f` requires a UTF8 database observation to reject a LATIN1 catalog identity and bounds the PostgreSQL 18 database-encoding domain. Initial production repair `33dcb94d71cb1d5fb52b458da507d09941a691a5` adds `PostgresDatabaseEncodingObservation` plus domain-separated `IndexCollationDatabaseEncodingSnapshot`. The successor composes the exact key-collation and expression/relation-`Var` catalog-identity predecessor digests, requires source/policy/extractor/time agreement, validates every modeled non-null collation as `-1` or the exact observed database encoding, and issues an immutable source receipt. Historical collation-identity digests remain unchanged.

Follow-up review `5204038985` on exact `af01eef5d52a1edcf91a1b616b334829f8c958a9` found that the first constructor used only `0..=PG_ENCODING_BE_LAST`. PostgreSQL 18 also has the historical `PG_UNUSED_1 == 7` enum hole and its `PG_VALID_BE_ENCODING` macro explicitly rejects unused encodings. Source RED `365d189d14e2525774bb32657ea6b4127a105ee9` requires ID 7 to fail while keeping `PG_SQL_ASCII == 0` and `PG_KOI8U == 34` valid. Minimal repair `d438f8a77df9c892aa4bc65f62f65ad857bb7ea6` mirrors that backend-valid domain by excluding the unused hole without changing any issued successor digest framing. Decision record: `docs/doctoring/postgresql-index-collation-database-encoding-integrity.md`.

The earlier same-qualified-name `-1` versus database-specific fixture remains conservative historical identity coverage, not database-compatibility evidence: PostgreSQL also prevents same-namespace any-encoding and database-specific collations of the same name from shadowing one another. The database-encoding successor is the authoritative representation boundary for source usability. The separate Source Observation column-collation family still retains its historical qualified-name identity and is not a substitute for this attached-index proof.

## Current state

**NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / RELATION_PARTITION_STRUCTURED_ATTTYPMOD_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_QUALIFIED_NAME_PREDECESSOR_PRESERVED / INDEX_PARTITION_COLLATION_CATALOG_IDENTITY_SOURCE_REPAIRED / INDEX_EXPRESSION_COLLATION_CATALOG_IDENTITY_COMPOSITION_SOURCE_REPAIRED / INDEX_COLLATION_DATABASE_ENCODING_BINDING_SOURCE_REPAIRED / POSTGRESQL_DATABASE_ENCODING_ID_DOMAIN_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / INDEX_EXPRESSION_NODE_SCHEMA_V1_PRESERVED / INDEX_EXPRESSION_NODE_SCHEMA_V2_VAR_SAFE_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_NODE_SCHEMA_COMPOSITION_SOURCE_REPAIRED / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_ENCODING_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the moved #46 head. One unchanged exact representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation/index-partition/type-modifier/expression-schema/relation-Var/whole-tree/collation-catalog/database-encoding contracts plus retained Source Observation contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still lacks the repository-owned Product PR workflow, so zero hosted runs on #46 are absence of acceptance evidence, not GREEN. Central workflow-owner work remains in `.github`; ConceptWeave must not copy or locally weaken that owner contract.

## Next causal work

1. Converge the canonical central workflow owner, then obtain compatible fresh unchanged-head acceptance for Product bootstrap #35 and land it normally on protected/default ConceptWeave `main`.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN. The catalog-identity and database-encoding contracts are part of this gate.
3. Only after that representation gate, extend #46 ordinary-forward with the concrete PostgreSQL 18 semantic-expression extractor/live attached-index differential oracle and raw `pg_attribute.atttypmod` adapter/rowtype differential. The same bounded source observation must capture `pg_database.encoding`, reject enum holes/client-only IDs according to PostgreSQL's backend-valid encoding predicate, resolve nonzero collation OIDs to exact `(namespace, name, collencoding)` rows, and prove each resolved row is `-1` or that exact database encoding. The transport descendant must reacquire exact-head acceptance.
4. Add typed constant/Datum semantics and additional node schemas only when PostgreSQL 18 equality can be represented completely; do not broaden generic node acceptance first.
5. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
