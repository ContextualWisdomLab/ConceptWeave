# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed history through exact `b1de4de974ac2b96495d054060ac48cb1cfe7f52` is preserved losslessly in `docs/archive/product-technical-gap-baseline-through-b1de4de9.md`; focused decisions after that point remain in `docs/doctoring/`. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack and single-writer boundary

- Protected/default ConceptWeave `main` remains repository acceptance authority; the active Source Observation branch is not release authority.
- #46 `codex/pr6-v3-index-evidence` is the active Draft Source Observation writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a`.
- #45 and #6 must not duplicate, partially cherry-pick, or independently reimplement this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.
- Central reusable workflow ownership remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, locally mutate, or wake that owner lane.

## Current Source Observation authority

All source-repaired relation-partition/index evidence archived through `b1de4de9...` remains retained: rowtype/`atttypmod`, topology/validity/uniqueness/access method, mapped key/`INCLUDE`, operator-family/exclusion semantics, canonical expression/predicate and relation-`Var` equality, exact collation catalog coordinates and database encoding, PostgreSQL-18 provider/field/encoding shape, material/effective database-default definitions, provider-version presence/drift/coherence, copied `ucs_basic`, libc encoding-independent C/POSIX rows, and provider-`d` delegation. No issued digest domain is rewritten by the current repairs.

### Database-default libc C-UTF8 encoding binding

Review `5213788571` found that `DatabaseDefaultCollationDefinitionObservation::validate_database_encoding()` treated every libc database-default definition as database-encoding compatible. That admitted source-unreachable governed evidence such as PostgreSQL database encoding LATIN1 with libc `datcollate` or `datctype` equal to `C.UTF-8`/`C.utf8`.

PostgreSQL 18 `CreateDatabase()` executes `check_encoding_locale_matches(encoding, dbcollate, dbctype)`. `pg_get_encoding_from_locale()` treats `C`/`POSIX` as SQL_ASCII-compatible and otherwise obtains the actual codeset from the host locale implementation. An available C-UTF8 locale resolves to UTF8, so LATIN1 is rejected. PostgreSQL separately retains the explicit superuser SQL_ASCII exception; that source-reachable catalog state must not be rejected.

- Source RED: `9e9379359406787d0e9cf7b964e7800f63055496`, `database_default_libc_c_utf8_database_encoding_contract.rs`.
- Initial minimal production repair: `336c42aaadeb729a65f2343bf3501826c9478343`.
- Source-review correction retaining PostgreSQL's SQL_ASCII exception: contract `68d4a6064822999ee4b1544c965476431fb8bd1f`, production `d690f3f94f71cae8a6e137693383ef6271d0614f`.
- Focused doctoring currentization: `fd6ade25d4ee3a5270d23f65b2d34c90d3039c76`, `docs/doctoring/postgresql-database-default-libc-c-utf8-encoding-integrity.md`.
- CHANGELOG currentization: `d135772a3dd471e8c90edc370a5b4ae58e780d52`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

Current invariant: for provider `c`, if either raw `datcollate` or `datctype` is case-insensitive `C.UTF-8`/`C.utf8`, the bounded PostgreSQL database encoding must be UTF8 (`6`) or the explicit source-reachable SQL_ASCII (`0`) path. LATIN1 is rejected. `C` and `POSIX` remain encoding-independent controls. Arbitrary libc locale names are not assigned a codeset by string parsing; PostgreSQL obtains that truth from the runtime/OS, so the remaining general libc compatibility proof belongs to concrete transport/live differential evidence.

### Column declaration relation-kind binding

Review `5214601753` found that the complete `pg_attribute.attidentity` and `attgenerated` families validated coordinates and completeness but did not constrain non-empty declaration modes by the owning `pg_class.relkind`. That admitted source-unreachable governed evidence such as identity state on a view or generated-column state on a materialized view.

The first source repair correctly constrained generated-column state to table/partitioned-table/foreign-table observations, but it incorrectly rejected identity state on foreign tables. Review `5214795520` corrected that source-domain claim. PostgreSQL 18 `CreateForeignTableStmt` uses `OptTableElementList`; `transformCreateStmt()` routes foreign-table `ColumnDef` nodes through the shared `CONSTR_IDENTITY` path, and that path rejects typed tables and `PARTITION OF` children but does not reject `cxt->isforeign`. The abbreviated `CREATE FOREIGN TABLE` documentation synopsis therefore cannot be used to erase a source-reachable catalog state.

- Initial finding review: `5214601753` on predecessor exact `619a43b493ee4e3751eecb43e25e688ae5afda58`.
- Initial source RED: `e5a5f10140d8b66299e317612b36a20d28f52ac1`, `column_declaration_relation_kind_contract.rs`.
- Initial identity repair converged at `2faafe07c2b53b9538a56d31a6febc5c07eae16e`; an intermediate ordinary-forward full-file replacement introduced an `encode_len` typo and the converged commit restores the original helper without rewriting history.
- Generation repair: `6f4d8871fc41e0ad283d950a00b80d6d5c8c410d`.
- Corrective foreign-table identity review: `5214795520` on exact `72dd82517c8d49b520100a45c12596373554e8d9`.
- Corrective source RED: `1c188a6a63470d89db6867da0cd689fa0ee667b6`; foreign-table identity moves into the positive-control set while view/materialized-view/sequence/composite-type identity remains rejected.
- Corrective production repair: `c3804597f89f6c471f9d203b386325b308cad4ee`; only `relation_kind_supports_identity()` is widened to `Table | PartitionedTable | ForeignTable`.
- Focused doctoring correction: `2d60e2a6e8063773595cde7bed62d547f0537d80`, `docs/doctoring/postgresql-column-declaration-relation-kind-integrity.md`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

Current invariant: non-empty identity and generated-column modes are admissible on table/partitioned-table/foreign-table observations. Explicit identity declaration is not admissible in `CREATE ... PARTITION OF`; partition identity is inherited from the partitioned-table hierarchy. Explicit `not_identity` and `not_generated` remain valid for every modeled relation kind. Existing digest domains are unchanged.

### Direct partition column declaration coherence

Review `5214826228` found a composition hole between the direct `pg_inherits` partition edge and the already-observed complete `attidentity` / `attgenerated` families. The relation-partition successor could accept a child identity mode that differed from its partitioned-table parent, or a child generated-column kind that differed from its parent, even though PostgreSQL 18 rejects those hierarchies.

PostgreSQL 18 requires partition identity properties to remain consistent across the partition hierarchy. For generated columns, parent and child must agree on ordinary/generated status and, when generated, on `STORED` versus `VIRTUAL`; the generation expression itself may differ and is intentionally outside this invariant.

- Finding review: `5214826228` on exact `725f0dbb5a0a2ce2c27a437c9e23cc23a0819903`.
- Source RED: `786e86ce94753d3cd7fb724e3c013ccdbcbf063b`, `relation_partition_column_declaration_contract.rs`.
- Minimal production repair: `ccbcd8711a493028f7b628dcdca8d2d878a5fb92`, adding optional-family composition in the existing `RelationPartitionSnapshot` canonicalization path without changing predecessor or relation-partition digest domains.
- Focused doctoring: `600fe0c51ff6f3ba9e738d7a31cf8a1d5fea463b`, `docs/doctoring/postgresql-relation-partition-column-declaration-coherence.md`.
- PostgreSQL authority: PostgreSQL 18 §§5.3, 5.4, 52.27 and `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

Current invariant: when a complete identity family is observed, every direct partition child column must preserve the parent's exact empty/ALWAYS/BY DEFAULT identity mode. When a complete generation family is observed, every direct partition child column must preserve the parent's exact ordinary/stored/virtual mode. If a family is unobserved, relation-partition evidence does not invent it. Generated expression equality is not required.

### Direct partition column-collation coherence

Review `5215040123` found a further composition hole at the same direct partition boundary. `RelationPartitionSnapshot` already rejected name/type/typmod, identity, generation, and NOT NULL contradictions, while the predecessor snapshot could also carry a complete resolved `pg_attribute.attcollation` family. The relation-partition owner did not compose that family, so otherwise-compatible parent/child `text` columns with different collations could enter governed evidence.

PostgreSQL 18 `CreateInheritance()` / attach-partition processing calls `MergeAttributesIntoExisting()`, which rejects a child attribute whose `attcollation` differs from the parent after type/typmod compatibility checks. The PostgreSQL 18 partitioning documentation independently requires attached table columns to match the parent. ConceptWeave therefore composes the already-governed qualified collation coordinate when the optional complete family was observed; it does not copy or infer collation truth inside the relation-partition owner.

- Finding review: `5215040123` on exact predecessor `17104b906369b8e6ade7fde4030bf2dba260e745`.
- Source RED: `7cf61bae74c72595fba47b92061d7366ef39c329`, `relation_partition_column_collation_contract.rs`, with parent `pg_catalog.C` versus child `pg_catalog.POSIX` and a matching `C`/`C` positive control.
- Minimal production repair: `107ac145b4bb9c18c45ae6d495cd97ceb3497ea9`, adding `validate_partition_column_collations()` to the existing relation-partition canonicalization path. An unobserved collation family remains unobserved; no issued predecessor or relation-partition digest domain changes.
- Focused doctoring: `b7f168087d5651eb0462ba36fac8ec695269cacd`, `docs/doctoring/postgresql-relation-partition-column-collation-integrity.md`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

Current invariant: if complete column-collation evidence is observed, every direct partition child column must preserve the parent's exact resolved qualified collation identity. The column-collation owner continues to own coordinate completeness and determinism consistency; relation-partition evidence only composes that owner fact across the direct edge. If the family was not observed, relation-partition evidence does not invent it.

## Current state

**RELATION_PARTITION_COLUMN_COLLATION_SOURCE_REPAIRED / RELATION_PARTITION_COLUMN_DECLARATION_SOURCE_REPAIRED / FOREIGN_TABLE_IDENTITY_SOURCE_REPAIRED / COLUMN_DECLARATION_RELATION_KIND_SOURCE_REPAIRED / INDEX_DATABASE_DEFAULT_LIBC_C_UTF8_ENCODING_BINDING_SOURCE_REPAIRED / POSTGRESQL_PARTITION_COLUMN_COLLATION_DIFFERENTIAL_OPEN / POSTGRESQL_PARTITION_COLUMN_DECLARATION_DIFFERENTIAL_OPEN / POSTGRESQL_COLUMN_DECLARATION_RELATION_KIND_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_LIBC_GENERAL_ENCODING_DIFFERENTIAL_OPEN / POSTGRESQL_COLLATION_DEFINITION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_COLLATION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_ENCODING_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

The complete archived Source Observation state through `b1de4de9...` remains authoritative unless explicitly superseded by a focused decision record. This concise baseline is not a deletion of earlier contracts or evidence; the archive is the retained history and this file is the current decision surface.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed after these ordinary-forward head moves. One unchanged exact #46 representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the declaration relation-kind contract, partition column-declaration coherence contract, partition column-collation coherence contract, libc C-UTF8 database-encoding contract, and every retained Source Observation focused contract, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still requires repository-owned Product PR workflow convergence through #35. Central workflow-owner work remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, wake, or locally weaken that owner contract.

## Next causal work

1. Converge the canonical central workflow owner and obtain compatible fresh unchanged-head acceptance for Product bootstrap #35; land #35 normally on protected/default ConceptWeave `main` only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
3. Only after that representation gate, extend #46 ordinary-forward with the concrete PostgreSQL 18 extractor/live differential. For direct partition collations, prove a matching parent/child collation is accepted, a different-collation attach is rejected by PostgreSQL and the equivalent governed tuple fails closed, and unobserved collation-family state remains unobserved. For column declarations, prove direct foreign-table identity and generated state, table/partitioned-table identity state, inherited leaf-partition identity properties, stored/virtual generated-kind coherence across each direct partition edge, permitted generation-expression differences, and catalog-empty declaration state on relation kinds that cannot own those declarations; distinguish direct foreign-table identity from a `PARTITION OF` child that cannot declare identity independently. For database-default libc, prove UTF8 + C-UTF8 acceptance, LATIN1 + C-UTF8 rejection/non-creation, the authorized SQL_ASCII + C-UTF8 path, and C/POSIX cross-encoding controls. For arbitrary libc locale names, observe runtime codeset compatibility instead of inferring it from spelling. All retained built-in/ICU/provider-`d`, version, copied-`ucs_basic`, expression, and `atttypmod` differentials remain required.
4. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.