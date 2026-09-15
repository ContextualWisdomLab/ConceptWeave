# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This file is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete prior baseline through exact `74ef156e0eefe250ec4d74bcba68ab60e256c90a` is preserved losslessly in `docs/archive/product-technical-gap-baseline-through-74ef156e.md`; focused source decisions remain in `docs/doctoring/`. Exact SHAs, review IDs, workflow IDs, and statuses are evidence coordinates only. Evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack and single-writer boundary

- Protected/default ConceptWeave `main` remains repository acceptance authority; the active Source Observation branch is not release authority.
- #46 `codex/pr6-v3-index-evidence` is the active Draft Source Observation writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a`.
- #45 and #6 must not duplicate, partially cherry-pick, or independently reimplement this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.
- Canonical reusable workflow ownership remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, locally mutate, or wake that owner lane.

## Retained Source Observation authority

All valid Source Observation repairs through exact `74ef156e...` remain retained without rewriting issued digest domains. This includes rowtype/`atttypmod`, relation/index partition topology, uniqueness/access method, key/`INCLUDE` mapping, operator-family/exclusion semantics, canonical expression/predicate and relation-`Var` equality, collation catalog/provider/version/database-encoding semantics, copied `ucs_basic`, libc C/POSIX and database-default C-UTF8 rules, column identity/generated-kind relation binding, direct partition declaration/collation coherence, foreign-table partitioned-index behavior, and valid-parent/valid-child index composition. Detailed rationale and predecessor evidence are in the archived baseline and focused doctoring records.

## Attached child constraint backing

Review `5216568917` found a remaining PostgreSQL 18 composition gap after index-definition and child-validity repair. `IndexPartitionSnapshot` admitted an attached child index whenever the index definitions were compatible, even when the parent index backed an observed `PRIMARY KEY` or `UNIQUE` constraint and the child index backed no child constraint.

PostgreSQL 18 `ATExecAttachPartitionIdx()` performs a separate constraint step after `CompareIndexInfo()`. If `get_relation_idx_constraint_oid()` resolves a constraint for the parent index, PostgreSQL requires the child index to resolve its own constraint before index and constraint parentage are installed. Missing child constraint state raises `ERRCODE_INVALID_OBJECT_DEFINITION`; definition equivalence alone is insufficient.

- Finding review: `5216568917` on exact predecessor `74ef156e0eefe250ec4d74bcba68ab60e256c90a`.
- Source regression contract: `ead7376a2c497c354a544d76ce80325bbe037524`, `crates/conceptweave-relation-partition/tests/index_partition_constraint_backing_contract.rs`.
- Minimal production repair: `72d7000e9130fa4b51d4b9b03a801a37d3b72351`, `crates/conceptweave-relation-partition/src/index_partition_base.rs`.
- Focused doctoring: `f66ee6481d697a556c3d8b232cac0d9bdf7053dd`, `docs/doctoring/postgresql-index-partition-child-constraint-integrity.md`.
- Lossless predecessor-baseline archive: `ccddc2d631f5f5d4b74ba60c409345a75f66b9cb`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()`.

Current invariant: an attached standalone parent index does not imply any constraint. If, however, the parent index is already bound by the v3 observation contract to an observed primary-key or unique constraint, the attached child index must likewise be bound to an observed primary-key or unique constraint on its own relation. ConceptWeave composes existing relation-local constraint/backing-index truth; it does not infer constraints from index shape, does not copy constraint truth into the index-partition owner, and does not change the existing index-partition digest domain.

The current repair intentionally does not claim a stronger parent/child constraint-subtype equality rule than established by the inspected PostgreSQL attach path. Any stronger rule requires a separate source finding and executable contract.

## Current state

**INDEX_PARTITION_CHILD_CONSTRAINT_SOURCE_REPAIRED / INDEX_PARTITION_CHILD_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_FOREIGN_CHILD_SOURCE_REPAIRED / RELATION_PARTITION_COLUMN_COLLATION_SOURCE_REPAIRED / RELATION_PARTITION_COLUMN_DECLARATION_SOURCE_REPAIRED / FOREIGN_TABLE_IDENTITY_SOURCE_REPAIRED / COLUMN_DECLARATION_RELATION_KIND_SOURCE_REPAIRED / INDEX_DATABASE_DEFAULT_LIBC_C_UTF8_ENCODING_BINDING_SOURCE_REPAIRED / POSTGRESQL_INDEX_PARTITION_CHILD_CONSTRAINT_DIFFERENTIAL_OPEN / POSTGRESQL_INDEX_PARTITION_CHILD_VALIDITY_DIFFERENTIAL_OPEN / POSTGRESQL_INDEX_PARTITION_FOREIGN_CHILD_DIFFERENTIAL_OPEN / POSTGRESQL_PARTITION_COLUMN_COLLATION_DIFFERENTIAL_OPEN / POSTGRESQL_PARTITION_COLUMN_DECLARATION_DIFFERENTIAL_OPEN / POSTGRESQL_COLUMN_DECLARATION_RELATION_KIND_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_LIBC_GENERAL_ENCODING_DIFFERENTIAL_OPEN / POSTGRESQL_COLLATION_DEFINITION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_COLLATION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_ENCODING_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed after the current ordinary-forward head movement. One unchanged exact #46 representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new child-constraint backing contract and every retained Source Observation/relation-partition focused contract, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still requires repository-owned Product PR workflow convergence through #35. Central workflow-owner work remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, wake, or locally weaken that owner contract.

## Next causal work

1. Converge the canonical central workflow owner and obtain compatible fresh unchanged-head acceptance for Product bootstrap #35; land #35 normally on protected/default ConceptWeave `main` only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
3. Only after that representation gate, extend #46 ordinary-forward with concrete PostgreSQL 18 extractor/live differentials. The new child-constraint differential must prove that `ALTER INDEX parent_index ATTACH PARTITION child_index` fails when the parent index backs a key constraint but the compatible child index is standalone, and succeeds when the child index already backs its child constraint; then verify index and constraint parentage from source catalogs. All retained child-validity, foreign-table, direct-partition collation/declaration, database-encoding/collation/version, expression, and `atttypmod` differentials remain required.
4. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
