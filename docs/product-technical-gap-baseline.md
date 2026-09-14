# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed history through exact `7c65090c2a90ba11dff575fde15fde37838795f6` remains byte-for-byte in `docs/archive/product-technical-gap-baseline-through-7c65090c.md`; later decisions are retained in focused `docs/doctoring/` records. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

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

Retained source-repaired checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` structure, mapped simple-column identity, corresponding key collation, resolved operator-family identity, exclusion operator/procedure/strategy semantics, relation-partition rowtype mapping, structured raw `pg_attribute.atttypmod`, and canonical expression/partial-predicate representation. Detailed evidence and repair lineage remain in the focused doctoring records.

### Relation-partition rowtype and raw type modifiers

PostgreSQL `build_attrmap_by_name()` maps partition columns by name, permits different physical attribute order, and rejects mismatched `atttypid` or `atttypmod`. Production `ad371e2feddb74c7457e1f28f4fb9992f758ea6b` rejects missing/extra child columns and mismatched qualified type identity without comparing `ordinal_position`.

Review `5202118195`, behavioral RED `2523a121214bd03b3e90bd3e45a391f7615f69c5`, and production `ccbea8dc919a13184488002b1b6f274802d93834` add the domain-separated `RelationPartitionTypeModifierSnapshot`: complete one-per-column raw signed `i32` `atttypmod`, exact predecessor rebound validation, direct parent/child equality by stable column name, deterministic digest framing, and receipts. The predecessor rendered-type bridge remains historical fail-closed behavior until an explicit version transition.

Open gate: the concrete PostgreSQL adapter must emit raw `pg_attribute.atttypmod` from the same bounded observation and pass a real PostgreSQL rowtype/partition differential oracle. Decision records: `docs/doctoring/postgresql-relation-partition-rowtype-integrity.md` and `docs/doctoring/postgresql-relation-partition-type-modifier-integrity.md`.

### Index-definition successors

`IndexOperatorFamilySnapshot` resolves observed operator classes to stable access-method/schema/family coordinates and compares direct attached parent/child families. `IndexExclusionSemanticsSnapshot` preserves exclusion presence plus per-key operator, underlying procedure, and strategy semantics. Mapped key/`INCLUDE`, collation, uniqueness, `NULLS NOT DISTINCT`, and access-method checks remain in the index-partition predecessor.

Decision records include `docs/doctoring/postgresql-index-partition-operator-family-integrity.md`, `docs/doctoring/postgresql-index-partition-exclusion-integrity.md`, and the earlier focused index-partition records.

### Expression/predicate representation and node schemas

PostgreSQL 18 `CompareIndexInfo()` maps child `ii_Expressions` and `ii_Predicate` through the partition attribute map, rejects an unpreservable child whole-row reference, and then applies internal `equal()` rather than rendered-text equality. Production `697597edb4f40e1dffedd7fcfd2f54cfaea91c87`, exported by `09028cc1945e078aee0f661b15bb01a11ec80365`, introduced `IndexExpressionSemanticsSnapshot`. Raw `pg_get_expr`, `pg_node_tree`/`nodeToString`, and reconstructed DDL remain invalid semantic substitutes.

Review `5201557492`, RED `cbf819f55b1ef54360c1ae47b2a23317d4a2dd23`, contract correction `fdc78e63fed2e933cd25ae8105541f7deadcecf3`, production `0c7e12d05c0da2bf279cc94a13a180043ae6dbd5`, and export `d7767cf93cc02cea684c039ba3c1385ab9143999` introduced the historical `node_schema.v1` gate. V1 models complete field sets for supported `FuncExpr`/`OpExpr` nodes and rejects `Const`/unknown nodes, but historically admits relation-local `Column`/`WholeRow` leaves without complete PostgreSQL `Var` equality state.

### Relation `Var` equality — v1 preserved, v2 fail-closed successor source repaired

Review `5202734131` found that a column-name-only `CanonicalExpression::Column` could be certified by v1 even though PostgreSQL 18 `CompareIndexInfo()` calls `map_variable_attnos()` and then `equal()` on the complete mapped tree. `map_variable_attnos_mutator()` copies the complete child `Var` and changes only the mapped attribute number (plus the equality-ignored syntactic attribute when applicable), so material `Var` fields remain part of PostgreSQL equality.

Behavioral source RED `60fe5da642bf32db0d6487a9c9a1f998dcfd39fe` demonstrated the unsafe completeness claim. Test-contract isolation `3eac59e2d8d50fe1d154b9d5cedd01f341ce5edd` separated positive `FuncExpr`/`OpExpr` schema controls from incomplete relation Vars.

The first production attempt `afbecf97fb126bfb59edb95911d7214d375849c9` hardened v1 in place. Review `5202777524` correctly rejected that approach: changing the validator while retaining the same `node_schema.v1` digest domain/revision would redefine what an existing immutable v1 digest proves.

The repair is now versioned:

- preservation RED `af93518aa6d4467abe680a4fee225278aad41780` requires v1 to keep its original `Column`/`WholeRow` admission behavior;
- production `969b193f14d37af80d5f9522ab289792e2a2b2f3` restores v1 semantics and adds `validate_postgres18_equal_schema_v2()` plus `IndexExpressionNodeSchemaSnapshotV2` under `node_schema.v2` / revision `postgresql-18-equal-supported-v2-var-safe`;
- v2 rebound-validates the exact expression-semantics predecessor and exact v1 digest before applying the stricter leaf rule, then hashes the v1 digest under the new domain;
- regression `bdaa26c9f20627a1275a9163e8aae98ea37fd614` locks both contracts: v1 remains historically reproducible; v2 rejects current `Column`/`WholeRow` leaves and accepts modeled no-Var `FuncExpr`/`OpExpr` trees.

A later domain-separated relation-`Var` semantic successor must preserve or prove target-relation role, attribute-map-normalized column identity, stable qualified value type, exact raw type modifier, stable qualified collation, nulling relations, `varlevelsup`, and `varreturningtype`. PostgreSQL-equality-ignored `varnosyn`, `varattnosyn`, and parse location must stay out of governed identity. Decision record: `docs/doctoring/postgresql-expression-var-equality-integrity.md`.

This repair removes the unsafe authoritative path without rewriting v1. It does not claim the concrete PostgreSQL semantic-expression extractor or attachment differential oracle is finished.

## Current state

**NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / RELATION_PARTITION_STRUCTURED_ATTTYPMOD_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / INDEX_EXPRESSION_NODE_SCHEMA_V1_PRESERVED / INDEX_EXPRESSION_NODE_SCHEMA_V2_VAR_SAFE_SOURCE_REPAIRED / POSTGRESQL_EXPRESSION_VAR_SUCCESSOR_OPEN / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the moved #46 head. One unchanged exact head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation/index-partition/type-modifier/expression-schema contracts plus retained Source Observation contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement resets exact-head acceptance.

## Next causal work

1. Add the domain-separated PostgreSQL 18 relation-`Var` semantic successor on top of `IndexExpressionNodeSchemaSnapshotV2`. Re-admit a relation variable only after every `equal()`-participating field is represented with stable identity or proven fixed at the stored-index boundary.
2. Implement the concrete PostgreSQL 18 semantic-expression extractor. It must emit only supported complete node/leaf schemas, resolve OIDs to stable coordinates, and reject unsupported nodes rather than degrade to text.
3. Add a live PostgreSQL differential oracle comparing ConceptWeave admission with PostgreSQL's actual index-partition attachment outcome, including attribute-order mapping and Var type/typmod/collation mismatch cases.
4. Add typed constant/Datum semantics and additional node schemas only when PostgreSQL 18 equality can be represented completely; do not broaden generic node acceptance first.
5. Wire the concrete PostgreSQL adapter to emit exact `pg_attribute.atttypmod` for every bounded column and add a live rowtype differential oracle.
6. Converge central workflow ownership; obtain compatible fresh unchanged-head acceptance for #35 and land it normally; then obtain one unchanged #46 native+hosted GREEN.
7. Only after terminal #46 exact-head GREEN may the complete child flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. PostgreSQL transport completion, semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
