# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed lineage through exact `7c65090c2a90ba11dff575fde15fde37838795f6` is preserved byte-for-byte in `docs/archive/product-technical-gap-baseline-through-7c65090c.md`. Later decisions are retained in focused doctoring records. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only; evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

- Protected/default ConceptWeave `main` remains the repository acceptance authority; the current Source Observation branch is not release authority.
- #6 remains the upstream product stack owner; #45 remains its Draft v3 representation child.
- #46 `codex/pr6-v3-index-evidence` remains the active Draft Source Observation writer stacked on #45. Any head movement invalidates earlier execution/review acceptance.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.

#45 and #6 must not duplicate or partially cherry-pick this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## Current Source Observation state

The frozen v3 snapshot still owns relation/type/index/constraint coordinates and raw relation-scoped index evidence. Domain-separated successors preserve declarative relation partitioning, index-partition topology, NOT NULL inheritance coherence, and PostgreSQL 18 attachment-definition evidence without changing frozen predecessor digest meaning.

Retained source-repaired attachment checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` structure, mapped simple-column identity, corresponding key collation, resolved operator-family identity, exclusion operator/procedure/strategy semantics, relation-partition rowtype mapping, and canonical expression/partial-predicate semantics. Detailed rationale remains in `docs/doctoring/`.

### Relation-partition rowtype mapping — source repaired, structured modifier successor open

PostgreSQL `build_attrmap_by_name()` maps partition columns by name, permits different physical attribute order, and rejects mismatched `atttypid` or `atttypmod`. Production `ad371e2feddb74c7457e1f28f4fb9992f758ea6b` therefore rejects missing/extra child columns and mismatched qualified type identity without comparing `ordinal_position`.

Frozen v3 does not structurally expose `pg_attribute.atttypmod`. The current relation-partition predecessor uses exact adapter-rendered `data_type` only as a temporary fail-closed modifier witness. That bridge is not preferred semantic identity and must be replaced by a domain-separated structured `atttypmod` observation before complete rowtype parity is claimed. Decision record: `docs/doctoring/postgresql-relation-partition-rowtype-integrity.md`.

### Index definition successors — source repaired, acceptance pending

Operator-family and exclusion successors preserve PostgreSQL 18 `CompareIndexInfo()` semantics that frozen v3 could not prove:

- `IndexOperatorFamilySnapshot` resolves observed operator classes to stable access-method/schema/family coordinates and compares direct attached parent/child families;
- `IndexExclusionSemanticsSnapshot` preserves exclusion presence plus per-key operator, underlying procedure, and strategy semantics;
- mapped key/`INCLUDE`, collation, uniqueness, `NULLS NOT DISTINCT`, and access-method checks remain in the index-partition predecessor.

Decision records: `docs/doctoring/postgresql-index-partition-operator-family-integrity.md`, `docs/doctoring/postgresql-index-partition-exclusion-integrity.md`, and the earlier focused index-partition records.

### Expression/predicate semantics — representation repaired, extractor/differential verification open

PostgreSQL 18 `CompareIndexInfo()` maps child `ii_Expressions` and `ii_Predicate` through the partition attribute map, rejects an unpreservable child whole-row reference, and applies internal node equality rather than rendered-text equality. Production `697597edb4f40e1dffedd7fcfd2f54cfaea91c87`, exported by `09028cc1945e078aee0f661b15bb01a11ec80365`, adds `IndexExpressionSemanticsSnapshot` over the exact exclusion predecessor.

Relation-local Vars use stable column names rather than physical `attnum`; OID-bearing type/collation/operator/function identities use stable qualified coordinates/signatures; nested list order remains semantic; whole-row state is explicit. Raw `pg_get_expr`, raw `pg_node_tree`/`nodeToString`, and reconstructed DDL remain invalid semantic substitutes. Decision record: `docs/doctoring/postgresql-index-partition-expression-predicate-integrity.md`.

### PostgreSQL expression node equality schema — source repaired, extractor coverage open

Review `5201557492` on exact `64e4ef2e77d3b56d68573e2cd5aa19fa8fc6338a` found a second P1 in the new semantic representation: generic `Node { kind, fields }` could claim completeness while omitting PostgreSQL 18 `equal()` fields. The then-current fixtures admitted `FuncExpr` with only function+arguments, `OpExpr` with only operator+arguments, and rendered-text `Const` evidence.

- Behavioral source RED `cbf819f55b1ef54360c1ae47b2a23317d4a2dd23` demonstrated the missing completeness gate.
- Contract correction `fdc78e63fed2e933cd25ae8105541f7deadcecf3` moved the gate to a versioned validation successor so the expression-semantics predecessor is not silently rewritten.
- Production `0c7e12d05c0da2bf279cc94a13a180043ae6dbd5`, exported by `d7767cf93cc02cea684c039ba3c1385ab9143999`, adds `validate_postgres18_equal_schema()` plus `IndexExpressionNodeSchemaSnapshot`.
- Supported `FuncExpr` evidence must include stable function/result identity, set-returning state, variadic state, result/input collation states, and ordered arguments. Supported `OpExpr` evidence must include stable operator identity, result type, set-returning state, result/input collation states, and ordered arguments.
- PostgreSQL `equal()`-ignored parse location and `CoercionForm` are deliberately excluded.
- `Const` and unknown node kinds fail closed. PostgreSQL custom `_equalConst()` depends on typmod, collation, length, null/by-value flags, and exact Datum equality; rendered constant text is not accepted as a substitute.

The successor frames the exact predecessor digest only after all expression/predicate trees satisfy the supported node schemas. This closes arbitrary field-subset admission for the modeled nodes; it does not implement the concrete PostgreSQL extractor. Decision record: `docs/doctoring/postgresql-expression-node-equality-schema-integrity.md`.

State: **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / INDEX_EXPRESSION_NODE_SCHEMA_SOURCE_REPAIRED / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / RELATION_PARTITION_STRUCTURED_ATTTYPMOD_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the current moved head. One unchanged #46 exact head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation/index-partition and expression-node-schema contracts plus retained Source Observation contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement restarts exact-head acceptance.

## Next causal work

1. Implement the concrete PostgreSQL 18 semantic-expression extractor at the Source Observation adapter boundary. It must emit only supported complete node schemas, resolve OIDs to stable coordinates, and reject unsupported nodes rather than degrade to text.
2. Add a real PostgreSQL differential oracle: compare ConceptWeave admission with PostgreSQL's actual index-partition attachment outcome and retain mismatches as regression fixtures.
3. Add typed constant/DATUM semantics and further node schemas only when PostgreSQL 18 equality can be represented completely; do not broaden generic node acceptance first.
4. Replace the temporary frozen-v3 rendered type-modifier witness with a domain-separated structured `pg_attribute.atttypmod` successor.
5. Converge central workflow ownership; obtain compatible fresh unchanged-head acceptance for #35 and land it normally; then obtain one unchanged #46 native+hosted GREEN.
6. Only after terminal #46 exact-head GREEN may the complete child flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. PostgreSQL transport completion, semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
