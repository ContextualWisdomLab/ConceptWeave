# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed lineage through exact `7c65090c2a90ba11dff575fde15fde37838795f6` is preserved byte-for-byte in `docs/archive/product-technical-gap-baseline-through-7c65090c.md`; this active baseline stays intentionally compact so live gaps do not disappear inside historical chronology. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Evidence from an earlier head never transfers after head movement.

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

Retained source-repaired index-partition checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` cardinality and role boundary, simple-column versus expression slot shape, mapped simple-column identity, corresponding key collation, resolved operator-family identity, and exclusion operator/procedure/strategy semantics. Earlier detailed chronology and NOT NULL repair lineage remain in the archived baseline referenced above.

### Relation-partition rowtype mapping — source repaired, acceptance pending

PostgreSQL declarative partitions must expose the same user-column set as their partitioned parent. `ALTER TABLE ... ATTACH PARTITION` requires matching column types, while PostgreSQL's `build_attrmap_by_name()` intentionally allows physically different attribute order by matching names and then rejecting type/type-modifier mismatch.

- Review `5201119054` on exact `28236f1f68ab86a621047de5cb547b8db7597514` identified that `RelationPartitionSnapshot` previously validated parent kind/topology/NOT NULL coherence but not the parent/child rowtype.
- Source RED `37062c4b49229ba8f7aae56b7c4ac607f59e0398` covers missing and extra child columns, different qualified type identity, different type-modifier rendering, and a positive control whose physical ordinals differ.
- Production `ad371e2feddb74c7457e1f28f4fb9992f758ea6b` validates exact name sets before digest, compares frozen-v3 qualified type bindings, and—because frozen v3 does not structurally expose `pg_attribute.atttypmod`—uses the exact adapter-rendered `data_type` witness as a temporary fail-closed modifier check. It deliberately does not compare `ordinal_position`.
- Primary-source decision record: `docs/doctoring/postgresql-relation-partition-rowtype-integrity.md`.

The rendered type witness is a bounded compatibility bridge, not a new claim that display text is preferred semantic identity. A future domain-separated structured `atttypmod` successor may replace this check without rewriting frozen v3 or current predecessor receipts.

### Operator-family successor — source repaired, acceptance pending

PostgreSQL 18 `CompareIndexInfo()` compares operator-family identity for every key after mapped attributes and collation. v3 retained `pg_opclass` but not `pg_opfamily`, so review `5200439852` on exact `7c65090c2a90ba11dff575fde15fde37838795f6` identified a P1 representation gap.

- Source contract `89394d73f5a468fcfde958f181a5bf8ba059e14f` requires a direct parent/child family mismatch to fail, preserves different operator classes that resolve to the same family, and requires complete family evidence bound to the observed class.
- Production `9e0d28324719b59bb6dc03e24b651e86a98250c3` adds a domain-separated `IndexOperatorFamilySnapshot`; `a6e4a4a55ccf6c6b983bdbb95a309e130ed62dc2` repairs its explicit module path.
- The successor rebound-validates the exact predecessor stack, resolves class→family evidence as access-method/schema/name rather than catalog OID, compares direct parent/child families, issues receipts, and frames the exact index-partition predecessor digest.
- Primary-source decision record: `docs/doctoring/postgresql-index-partition-operator-family-integrity.md`.

### Exclusion-semantics successor — source repaired, acceptance pending

PostgreSQL 18 `CompareIndexInfo()` requires exclusion presence to agree and, for exclusion indexes, compares each key's exclusion operator, underlying procedure, and operator-family strategy.

- Review `5200580545` and source RED `198414959168b29ee92965576c3c91bd768a12a5` isolate one-sided exclusion presence plus operator/procedure/strategy mismatch and completeness.
- Production `c6c97b2bc545b064c57faafeca67f7bea431235f`, exported by `bd0e4e54fbc67f5e2ed32a72738dbadc90c91336`, adds `IndexExclusionSemanticsSnapshot` over the exact operator-family predecessor.
- Operators/procedures are stable schema/name/type signatures rather than database-local OIDs; strategy numbers remain exact PostgreSQL evidence.
- Primary-source decision record: `docs/doctoring/postgresql-index-partition-exclusion-integrity.md`.

### Expression/predicate semantic successor — representation repaired, extractor/differential verification open

PostgreSQL 18 `CompareIndexInfo()` does not compare rendered expression text. It maps child `ii_Expressions` and `ii_Predicate` through the partition attribute map, rejects an unpreservable child whole-row reference, and then applies internal node equality. Frozen v3 only carried server-rendered expression/predicate strings, so exact `a97c25eb087c1aeda0bc5e534e40c1ce51dfc994` admitted expression/expression slot shape without proving this semantic equality. Review `5201012106` records the P1.

- Source RED `660f1adda178ac3ba0436cda14003f414e0a0e2b` requires unequal expression trees, unequal partial predicates, and direct-child whole-row references to fail; it preserves equality when parent/child physical column ordinals differ but stable column identity is the same; and it requires semantic evidence for every raw expression and predicate.
- Production `697597edb4f40e1dffedd7fcfd2f54cfaea91c87`, exported by `09028cc1945e078aee0f661b15bb01a11ec80365`, adds `IndexExpressionSemanticsSnapshot` over the exact exclusion predecessor.
- Relation-local Vars become exact column names rather than `attnum`; type/collation/operator/function identities use qualified stable coordinates/signatures rather than catalog OIDs; nested expression lists preserve order; semantic node fields are deterministically named/sorted; whole-row state remains explicit.
- The new digest is domain-separated, frames the exact exclusion predecessor, and records semantic schema revision `postgresql-18-equal-v1`; frozen v3 rendered evidence is not rewritten.
- Direct attached edges compare canonical parent/child semantic trees. Child whole-row expression/predicate references fail closed. All semantic column references are checked against the exact owning relation.
- Primary-source decision record: `docs/doctoring/postgresql-index-partition-expression-predicate-integrity.md`.

This closes the in-memory representation/comparison gap but not the live PostgreSQL extraction proof. The concrete PostgreSQL adapter must still derive the canonical tree from the PostgreSQL 18 node/catalog structure, resolve every OID-bearing `equal()` field to stable coordinates, fail closed for unsupported node kinds, and pass a differential oracle against real PostgreSQL attachment behavior. Raw `pg_get_expr`, raw `pg_node_tree`/`nodeToString`, or reconstructed `pg_get_indexdef`/DDL is not an acceptable fallback.

State: **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_ROWTYPE_MAPPING_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_EXCLUSION_SOURCE_REPAIRED / INDEX_PARTITION_EXPRESSION_PREDICATE_REPRESENTATION_REPAIRED / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the current moved head. The available execution host does not provide the repository-pinned Rust 1.98 toolchain, and protected ConceptWeave `main` still lacks the Product PR workflow at this checkpoint. The committed contracts and causal source changes are source evidence only.

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation-partition rowtype/index-partition/NOT NULL/operator-family/exclusion/expression-predicate contracts plus retained expression/generation/identity/collation/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement restarts exact-head acceptance.

## Next causal work

1. Implement the concrete PostgreSQL 18 semantic-expression extractor at the Source Observation adapter boundary. It must cover every admitted `equal()`-participating semantic field, resolve OIDs to stable coordinates, and reject unsupported nodes rather than degrade to text.
2. Add a real PostgreSQL differential oracle: generate admissible expression/partial-index parent/child cases, compare ConceptWeave admission with PostgreSQL's actual partition-index attachment result, and retain mismatch fixtures as regressions.
3. Replace the temporary frozen-v3 rendered type-modifier witness with a domain-separated structured modifier observation before claiming complete PostgreSQL rowtype semantic parity; do not rewrite current predecessor identity to do so.
4. Converge central workflow ownership; obtain compatible fresh unchanged-head acceptance for #35 and land it normally; then obtain one unchanged #46 native+hosted GREEN.
5. Only after terminal #46 exact-head GREEN may the complete child flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation.
6. PostgreSQL transport completion, semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
