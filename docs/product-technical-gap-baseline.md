# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This file is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete prior baseline through exact `74ef156e0eefe250ec4d74bcba68ab60e256c90a` is preserved losslessly in `docs/archive/product-technical-gap-baseline-through-74ef156e.md`; focused source decisions remain in `docs/doctoring/`. Exact SHAs, review IDs, workflow IDs, and statuses are evidence coordinates only. Evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack and single-writer boundary

- Protected/default ConceptWeave `main` remains repository acceptance authority; the active Source Observation branch is not release authority.
- #46 `codex/pr6-v3-index-evidence` is the active Draft Source Observation writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a` unless fresh GitHub state says otherwise.
- #45 and #6 must not duplicate, partially cherry-pick, or independently reimplement this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.
- Canonical reusable workflow ownership remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, locally mutate, or wake that owner lane.

## Retained Source Observation authority

All valid Source Observation repairs through the prior exact Source Observation lineage remain retained without rewriting issued digest domains. This includes rowtype/`atttypmod`, relation/index partition topology, uniqueness/access method, key/`INCLUDE` mapping, operator-family/exclusion semantics, canonical expression/predicate and relation-`Var` equality, collation catalog/provider/version/database-encoding semantics, copied `ucs_basic`, libc C/POSIX and database-default C-UTF8 rules, column identity/generated-kind relation binding, direct partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child index composition, and constraint-backed child-index presence. Detailed rationale and predecessor evidence remain in the archived baseline and focused doctoring records.

## Exact index-constraint parentage

Review `5217351716` found that constraint presence was still weaker than PostgreSQL 18 source truth. `IndexPartitionSnapshot` could prove that an attached child index and parent index each backed an observed `PRIMARY KEY` or `UNIQUE` constraint, but no governed family retained resolved `pg_constraint.conparentid`. The same ConceptWeave evidence could therefore represent both the PostgreSQL state in which the child constraint is parented to the exact parent constraint and an incomplete/fabricated state in which the child constraint is merely local.

PostgreSQL 18 `ATExecAttachPartitionIdx()` resolves constraints by table/index OID and, when the parent index is constraint-backed, requires a child constraint before calling `ConstraintSetParentConstraint(child_constraint_oid, parent_constraint_oid, child_table_oid)`. PostgreSQL also explicitly supports the converse case in which a constraint-backed child index is attached below a non-constraint parent index; that child constraint remains local and must not acquire invented parentage. `pg_constraint.conparentid` is therefore material source identity, not derivable decoration.

The repair is a new domain-separated `IndexConstraintParentageSnapshot` above the exact `IndexPartitionSnapshot`. It is complete over observed primary-key and unique constraints, resolves `conparentid` to exact `(schema, relation, relation_kind, constraint_name)` coordinates, and requires an attached constraint-backed child to identify the exact parent constraint implied by the attached backing-index edge. Child constraints below non-constraint parent indexes remain root/local. Frozen v3, relation-partition, and index-partition digests are unchanged.

- Finding review: `5217351716` on predecessor `8f61b6352f87a18efb113cd6fae1f54fe55adbc9`.
- Source/compile RED contract: `1534eb3d01872a0f2cdc9c14ab5dff3cc5c86a99`, `crates/conceptweave-relation-partition/tests/index_constraint_parentage_contract.rs`.
- Production successor: `6b3b4ad06b79e51cee48c261244f1b216f86fa4e`, `crates/conceptweave-relation-partition/src/index_constraint_parentage.rs`.
- Public export: `5d94610f70a75292aa58c2b086fedfa17bf07122`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Focused doctoring: `81fd26686ecf76c4052c06bdc84ee86067124b25`, `docs/doctoring/postgresql-index-constraint-parentage-integrity.md`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()` and PostgreSQL 18 `pg_constraint` catalog documentation.

## Current state

**INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / INDEX_PARTITION_CHILD_CONSTRAINT_SOURCE_REPAIRED / INDEX_PARTITION_CHILD_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_FOREIGN_CHILD_SOURCE_REPAIRED / RELATION_PARTITION_COLUMN_COLLATION_SOURCE_REPAIRED / RELATION_PARTITION_COLUMN_DECLARATION_SOURCE_REPAIRED / FOREIGN_TABLE_IDENTITY_SOURCE_REPAIRED / COLUMN_DECLARATION_RELATION_KIND_SOURCE_REPAIRED / INDEX_DATABASE_DEFAULT_LIBC_C_UTF8_ENCODING_BINDING_SOURCE_REPAIRED / POSTGRESQL_INDEX_CONSTRAINT_PARENTAGE_DIFFERENTIAL_OPEN / POSTGRESQL_INDEX_PARTITION_CHILD_CONSTRAINT_DIFFERENTIAL_OPEN / POSTGRESQL_INDEX_PARTITION_CHILD_VALIDITY_DIFFERENTIAL_OPEN / POSTGRESQL_INDEX_PARTITION_FOREIGN_CHILD_DIFFERENTIAL_OPEN / POSTGRESQL_PARTITION_COLUMN_COLLATION_DIFFERENTIAL_OPEN / POSTGRESQL_PARTITION_COLUMN_DECLARATION_DIFFERENTIAL_OPEN / POSTGRESQL_COLUMN_DECLARATION_RELATION_KIND_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_LIBC_GENERAL_ENCODING_DIFFERENTIAL_OPEN / POSTGRESQL_COLLATION_DEFINITION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_COLLATION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_ENCODING_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed after the current ordinary-forward head movement. One unchanged exact #46 representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new exact constraint-parentage contract and every retained Source Observation/relation-partition focused contract, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still requires repository-owned Product PR workflow convergence through #35. Central workflow-owner work remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, wake, or locally weaken that owner contract.

## Next causal work

1. Converge the canonical central workflow owner and obtain compatible fresh unchanged-head acceptance for Product bootstrap #35; land #35 normally on protected/default ConceptWeave `main` only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
3. Only after that representation gate, extend #46 ordinary-forward with concrete PostgreSQL 18 extractor/live differentials. The new parentage differential must verify `pg_constraint.conparentid` after successful constraint-backed index attachment, reject an absent/wrong parent coordinate in the governed successor, and preserve `conparentid = 0` when a constraint-backed child index is attached below a non-constraint parent index. All retained child-constraint, child-validity, foreign-table, direct-partition collation/declaration, database-encoding/collation/version, expression, and `atttypmod` differentials remain required.
4. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
