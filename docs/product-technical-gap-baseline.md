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

Retained source-repaired checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` structure, mapped simple-column identity, qualified-name key collation, resolved operator-family identity, exclusion operator/procedure/strategy semantics, relation-partition rowtype mapping, structured raw `pg_attribute.atttypmod`, canonical expression/partial-predicate representation, complete stable relation-`Var` equality evidence, and the composed node-schema-plus-relation-Var whole-tree proof. Detailed lineage remains in the focused doctoring records.

### Rowtype and expression proof boundary

`RelationPartitionTypeModifierSnapshot` keeps exact raw signed `pg_attribute.atttypmod` for every bounded column and compares direct parent/child modifiers by stable column name. The concrete PostgreSQL adapter and live rowtype differential remain transport work after representation acceptance.

`IndexExpressionSemanticsSnapshot` preserves canonical expression/predicate trees because PostgreSQL `CompareIndexInfo()` maps child Vars and then applies internal `equal()`. Historical node-schema v1 proves the modeled non-Var `FuncExpr`/`OpExpr` fields, relation-Var v1 proves complete equality state for every canonical relation-column leaf, and `IndexExpressionRelationVarNodeSchemaSnapshot` composes those exact proofs without weakening node-schema v2. Whole-row Vars, typed `Const`, and unsupported nodes remain fail closed until complete PostgreSQL equality schemas exist.

Concrete semantic-expression extraction and live attachment differential remain transport work and must not be pulled ahead of terminal exact-head acceptance of the representation slice.

### PostgreSQL collation catalog-row identity — new P1 source branch

Review `5203297768` found that the historical key-collation check was not catalog-exact. `QualifiedCollationName` carries only namespace/name, but PostgreSQL 18 declares `pg_collation_name_enc_nsp_index` UNIQUE over `(collname, collencoding, collnamespace)`. `CompareIndexInfo()` compares the resolved per-key collation OIDs. Distinct `pg_collation` rows can therefore share the same namespace/name while differing in `collencoding`, which allowed the historical index-partition proof to collapse two distinct OID identities.

Behavioral compile RED `db761588b9e3a79da4cd6f2839b756ce06992601` fixes the counterexample: parent and child retain the same issued `pg_catalog.C` qualified name while resolved catalog identities differ by raw encoding (`-1` versus `6`). Production `d1d2423e2f20c328f459bf912f0e7c4e2c4d44ae` adds `CollationCatalogIdentity`, `IndexKeyCollationIdentityObservation`, and `IndexPartitionCollationIdentitySnapshot` under a new domain-separated digest. Export `255f414327d95a680c61e96c80d553f8e07e1137` and focused boundary/receipt coverage `0279cb46e486e8a3c74cb446915ad9c95bc53a61` preserve all predecessor digests unchanged.

The successor rebound-validates the exact index-partition predecessor, requires complete one-per-key-semantic evidence, binds namespace/name back to the issued v3 key semantics, preserves raw signed `collencoding`, compares full catalog identity across every direct attached parent/child key, and issues exact provenance receipts. Decision record: `docs/doctoring/postgresql-index-partition-collation-catalog-identity-integrity.md`.

This is not yet the final whole-expression collation proof. `QualifiedCollationName` also occurs in collation-bearing expression-node fields and relation-`Var` evidence. Those occurrences still need a domain-separated catalog-identity proof and explicit composition with the existing node-schema/relation-Var whole-tree proof. The historical values must not be rewritten in place. Column-collation evidence outside this attachment lane also retains its historical qualified-name contract until a versioned successor is introduced.

## Current state

**NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / RELATION_PARTITION_STRUCTURED_ATTTYPMOD_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_QUALIFIED_NAME_PREDECESSOR_PRESERVED / INDEX_PARTITION_COLLATION_CATALOG_IDENTITY_SOURCE_BRANCH_REPAIRED / INDEX_COLLATION_CATALOG_IDENTITY_COMPOSITION_OPEN / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / INDEX_EXPRESSION_NODE_SCHEMA_V1_PRESERVED / INDEX_EXPRESSION_NODE_SCHEMA_V2_VAR_SAFE_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_NODE_SCHEMA_COMPOSITION_SOURCE_REPAIRED / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the moved #46 head. One unchanged exact representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation/index-partition/type-modifier/expression-schema/relation-Var/composed-proof/collation-catalog contracts plus retained Source Observation contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still lacks the repository-owned Product PR workflow, so zero hosted runs on #46 are absence of acceptance evidence, not GREEN. Central workflow-owner work remains in `.github`; ConceptWeave must not copy or locally weaken that owner contract.

## Next causal work

1. Keep the collation repair within the representation lane: add catalog-exact identity for collation-bearing expression-node fields and relation-`Var` leaves, then compose that proof with the current whole-tree equality proof and the new per-key catalog-identity successor. Do not rewrite `QualifiedCollationName` or predecessor digests.
2. Converge the canonical central workflow owner, then obtain compatible fresh unchanged-head acceptance for Product bootstrap #35 and land it normally on protected/default ConceptWeave `main`.
3. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
4. Only after that representation gate, extend #46 ordinary-forward with the concrete PostgreSQL 18 semantic-expression extractor/live attached-index differential oracle and raw `pg_attribute.atttypmod` adapter/rowtype differential. The transport descendant must reacquire exact-head acceptance.
5. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
