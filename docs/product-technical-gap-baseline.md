# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed lineage through exact `7c65090c2a90ba11dff575fde15fde37838795f6` is preserved byte-for-byte in `docs/archive/product-technical-gap-baseline-through-7c65090c.md`; this active baseline stays intentionally compact so live gaps do not disappear inside historical chronology. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

- Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this source-edit checkpoint.
- #6 remains the upstream product stack owner; #45 remains its Draft v3 representation child.
- #46 `codex/pr6-v3-index-evidence` remains the active Draft Source Observation writer stacked on #45. The head moved after the prior `7c65090c...` evidence, so no earlier execution/review acceptance transfers.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.

#45 and #6 must not duplicate or partially cherry-pick this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## Current Source Observation state

The frozen v3 snapshot still owns relation/type/index/constraint coordinates and relation-scoped index semantics. Domain-separated successors now preserve declarative relation partitioning, index-partition topology, NOT NULL inheritance coherence, and PostgreSQL attachment-definition evidence without changing frozen predecessor digest meaning.

Retained source-repaired index-partition checks include uniqueness, `NULLS NOT DISTINCT`, access method, key/`INCLUDE` cardinality and role boundary, simple-column versus expression slot shape, mapped simple-column identity, and corresponding key collation. Earlier detailed chronology and NOT NULL repair lineage remain in the archived baseline referenced above.

### Operator-family successor — source repaired, acceptance pending

PostgreSQL 18 `CompareIndexInfo()` compares operator-family identity for every key after mapped attributes and collation. v3 retained `pg_opclass` but not `pg_opfamily`, so review `5200439852` on exact `7c65090c2a90ba11dff575fde15fde37838795f6` identified a P1 representation gap.

- Source contract `89394d73f5a468fcfde958f181a5bf8ba059e14f` requires a direct parent/child family mismatch to fail, preserves different operator classes that resolve to the same family, and requires complete family evidence bound to the observed class.
- Production `9e0d28324719b59bb6dc03e24b651e86a98250c3` adds a domain-separated `IndexOperatorFamilySnapshot` complete over every bounded index key. It rebound-validates the exact base/relation-partition/index-partition stack, resolves class→family evidence as access-method/schema/name, compares direct parent/child families, issues receipts, and hashes the predecessor index-partition digest plus canonical family evidence.
- Static module-resolution repair `a6e4a4a55ccf6c6b983bdbb95a309e130ed62dc2` fixes the explicit successor module path. The predecessor implementation is preserved byte-for-byte in `index_partition_base.rs`; `index_partition.rs` is now a thin composition wrapper.
- Primary-source decision record: `docs/doctoring/postgresql-index-partition-operator-family-integrity.md`.

State: **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_COLLATION_SOURCE_REPAIRED / INDEX_PARTITION_OPERATOR_FAMILY_SOURCE_REPAIRED / INDEX_PARTITION_DEFINITION_EQUIVALENCE_OPEN / ACCEPTANCE_PENDING**.

`INDEX_PARTITION_DEFINITION_EQUIVALENCE_OPEN` is now limited to canonical expression-tree equality under the partition attribute map, partial predicate equality, and exclusion operator/procedure/strategy semantics. Raw rendered expressions, predicates, index DDL, names, comments, tablespaces, or lifecycle flags are not accepted as semantic substitutes.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed for the current moved head. The available execution host does not provide the repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the Product PR workflow. The operator-family contract and causal source repair are source evidence only.

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused relation-partition/index-partition/NOT NULL/operator-family contracts plus retained expression/generation/identity/collation/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review evidence. Any head movement restarts exact-head acceptance.

## Next causal work

1. Statistically and semantically canonicalize PostgreSQL expression-tree equality under the partition attribute map without raw-text comparison.
2. Preserve and compare partial-index predicates using canonical expression evidence.
3. Preserve and compare exclusion operator/procedure/strategy semantics required by PostgreSQL attachment equivalence.
4. After source definition-equivalence closes, converge central workflow ownership, land #35 normally after compatible fresh acceptance, obtain one unchanged #46 native+hosted GREEN, then ordinary/non-force adopt the complete child into #45, re-accept #45, and propagate to #6.
5. PostgreSQL transport, semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, or premature publication/release is authorized.
