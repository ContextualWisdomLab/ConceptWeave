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

Retained source-repaired checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` structure, mapped simple-column identity, corresponding key collation, resolved operator-family identity, exclusion operator/procedure/strategy semantics, relation-partition rowtype mapping, structured raw `pg_attribute.atttypmod`, canonical expression/partial-predicate representation, complete stable relation-`Var` equality evidence, and the composed node-schema-plus-relation-Var whole-tree proof. Detailed lineage remains in the focused doctoring records.

### Relation-partition rowtype and raw type modifiers

PostgreSQL `build_attrmap_by_name()` maps partition columns by name, permits different physical attribute order, and rejects mismatched `atttypid` or `atttypmod`. Production `ad371e2feddb74c7457e1f28f4fb9992f758ea6b` rejects missing/extra child columns and mismatched qualified type identity without comparing `ordinal_position`.

Review `5202118195`, behavioral RED `2523a121214bd03b3e90bd3e45a391f7615f69c5`, and production `ccbea8dc919a13184488002b1b6f274802d93834` add `RelationPartitionTypeModifierSnapshot`: complete one-per-column raw signed `i32` `atttypmod`, exact predecessor rebound validation, direct parent/child equality by stable column name, deterministic digest framing, and receipts. The predecessor rendered-type bridge remains historical fail-closed behavior until an explicit version transition.

Open gate: the concrete PostgreSQL adapter must emit raw `pg_attribute.atttypmod` from the same bounded observation and pass a real PostgreSQL rowtype/partition differential oracle. Decision records: `docs/doctoring/postgresql-relation-partition-rowtype-integrity.md` and `docs/doctoring/postgresql-relation-partition-type-modifier-integrity.md`.

### Index-definition and expression successors

`IndexOperatorFamilySnapshot` resolves observed operator classes to stable access-method/schema/family coordinates and compares direct attached parent/child families. `IndexExclusionSemanticsSnapshot` preserves exclusion presence plus per-key operator, underlying procedure, and strategy semantics. Mapped key/`INCLUDE`, collation, uniqueness, `NULLS NOT DISTINCT`, and access-method checks remain in the index-partition predecessor.

PostgreSQL 18 `CompareIndexInfo()` maps child `ii_Expressions` and `ii_Predicate` through the partition attribute map, rejects an unpreservable child whole-row reference, and then applies internal `equal()` rather than rendered-text equality. `IndexExpressionSemanticsSnapshot` therefore preserves canonical semantic trees. Raw `pg_get_expr`, `pg_node_tree`/`nodeToString`, and reconstructed DDL remain invalid semantic substitutes.

Historical `node_schema.v1` models complete field sets for supported `FuncExpr`/`OpExpr` nodes but historically admits column-name-only `Column`/`WholeRow` leaves. Review `5202734131` exposed that limitation. V2 (`969b193f14d37af80d5f9522ab289792e2a2b2f3`) preserves v1 and adds a new fail-closed branch that rejects incomplete relation-`Var` leaves while continuing to accept modeled no-Var trees; regression `bdaa26c9f20627a1275a9163e8aae98ea37fd614` locks both contracts.

### Complete relation `Var` successor — source repaired

Review `5202984522` found a composability error in the prior causal plan: a relation-`Var` successor cannot literally be layered on a successful v2 snapshot because v2 intentionally rejects every relation-Var-bearing tree. Weakening v2 would rewrite an issued proof. The relation-`Var` family therefore branches from the exact expression-semantics predecessor while preserving both node-schema versions unchanged.

Behavioral compile RED `817f83ce9f3be644134bec5739d7a713cb9fa919`, production `b506f5154865a732244cc0da8df927c64619515e`, export wiring `03c60171976693830ddc8842035e5ef3256d55e8`, and focused source-binding tests `b6e7adb6dd89a27a58a885ae15498362e9d2a93c` introduce `IndexExpressionRelationVarSnapshot` under its own digest domain.

The successor requires exactly one relation-`Var` observation for every canonical `Column` leaf and preserves or proves all material PostgreSQL 18 `Var` equality state without retaining statement/database-local coordinates: indexed-relation role instead of raw `varno`; attribute-map-normalized column name instead of physical child `attnum`; qualified value type instead of `vartype` OID; exact raw `vartypmod`; qualified collation or explicit no-collation instead of `varcollid` OID; explicit empty `varnullingrels`; zero `varlevelsup`; and `VAR_RETURNING_DEFAULT`. Equality-ignored `varnosyn`, `varattnosyn`, and parse location remain outside governed identity.

The relation-Var snapshot rebound-validates exact expression-semantics and structured type-modifier predecessors, validates type/typmod/collation against source-authoritative bounded column families, compares direct attached parent/child relation-`Var` evidence after stable column-name normalization, frames deterministic successor identity, and issues exact receipts. Decision record: `docs/doctoring/postgresql-expression-relation-var-successor-integrity.md`.

### Composed whole-tree equality proof — source repaired

Review `5203122666` found that relation-Var v1 could prove every `Column` leaf while the enclosing `CanonicalExpression::Node` remained under-specified. The current relation-Var fixture made the defect concrete: incomplete `FuncExpr`/`OpExpr` fields and an unsupported rendered-value `Const` could still receive relation-Var evidence even though historical node-schema v1 rejects that enclosing tree. PostgreSQL attachment applies `equal()` to the complete mapped tree, not to each Var independently.

Behavioral compile RED `62635024cf93204d4be447fe9a4859b469a35312`, production `33beb01932d1fdd47d37b8f60c54680d343af84c`, and export wiring `197a5f7f89caee7d99f4427ac7b028f6e7aba6ab` add `IndexExpressionRelationVarNodeSchemaSnapshot` under a new `...relation_var.node_schema.v1` digest family. The successor preserves node-schema v1, node-schema v2, and relation-Var v1 unchanged. It rebound-validates historical node-schema v1 from the exact expression-semantics predecessor, rebound-validates relation-Var v1 from that same bounded source stack, requires exact provenance agreement, and frames the conjunction of both exact digests.

This closes the proof seam for the currently modeled Var-bearing `FuncExpr`/`OpExpr` surface: node-schema v1 proves the complete supported enclosing node fields and relation-Var v1 proves the complete relation-column leaves. Whole-row Vars remain rejected by relation-Var v1; `Const` and unsupported node kinds remain fail closed until typed PostgreSQL equality schemas are implemented. Decision record: `docs/doctoring/postgresql-expression-relation-var-node-schema-composition-integrity.md`.

This is source repair, not PostgreSQL adapter completion. The concrete semantic-expression extractor and live differential oracle remain open, as does the concrete raw `pg_attribute.atttypmod` adapter/oracle.

## Current state

**NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / RELATION_PARTITION_STRUCTURED_ATTTYPMOD_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / INDEX_EXPRESSION_NODE_SCHEMA_V1_PRESERVED / INDEX_EXPRESSION_NODE_SCHEMA_V2_VAR_SAFE_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_SOURCE_REPAIRED / INDEX_EXPRESSION_RELATION_VAR_NODE_SCHEMA_COMPOSITION_SOURCE_REPAIRED / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the moved #46 head. One unchanged exact head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation/index-partition/type-modifier/expression-schema/relation-Var/composed-proof contracts plus retained Source Observation contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement resets exact-head acceptance.

The current #46 slice is still representation/source-contract work. Concrete database I/O, live PostgreSQL attachment attempts, and adapter extraction remain transport work and must not be pulled ahead of terminal exact-head acceptance of this representation slice.

## Next causal work

1. Converge the canonical central workflow owner; repair and accept its current exact head without ConceptWeave-side source duplication or wake commits.
2. Obtain compatible fresh unchanged-head acceptance for Product bootstrap #35 and land it normally on protected/default ConceptWeave `main`.
3. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
4. Only after that representation gate, extend #46 ordinary-forward with the concrete PostgreSQL 18 semantic-expression extractor and live attached-index differential oracle. The extractor must emit only supported complete node/leaf schemas, resolve OIDs to stable coordinates, and reject unsupported nodes rather than degrade to text; that transport descendant must reacquire its own exact-head acceptance.
5. Add typed constant/Datum semantics and additional node schemas only when PostgreSQL 18 equality can be represented completely; do not broaden generic node acceptance first. Wire the concrete PostgreSQL adapter to emit exact `pg_attribute.atttypmod` for every bounded column and add a live rowtype differential oracle, again with fresh exact-head acceptance.
6. Only after the complete #46 child — including required transport evidence — is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.