# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The decision surface through the first cross-input direction/function lineage repair is preserved at `docs/archive/product-technical-gap-baseline-through-71fd68c9.md`; earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The ordinary-forward converter chain retains all previously established converter definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, converter-function extension membership, and independent transform-object extension membership.

The transform-object successor remains one fact per exact `(constraint, key_position, transform_type, target_language)` `pg_transform` row. PostgreSQL 18 defines independent optional FROM SQL and TO SQL conversion functions for that row, so direction and exact converter function identity are material raw binding facts even though extension membership itself is object-level rather than direction-level.

The first cross-input lineage repair established that generation metadata plus a shared transform coordinate were insufficient. Finding review `5253304917` on `fcaa330965915a85ea7116a66c04286b79ef5931` -> hostile direction-set RED `3f3a947e31c3b80e1a66120b89705ec3e515e43c` -> production binding repair `37742d774d7c33a693dc3a0359a6583134bc5cbf` -> same-generation converter-function identity edge contract `276821d5d5fdafbd4776c9856acfc04221e470c1` -> focused doctoring `71fd68c93805d856171653e63c575075a7f5e16c`. That repair requires complete raw FROM SQL / TO SQL cardinality and exact constraint/key/type/direction/schema/function binding.

Fresh follow-up found a stronger ancestry defect that survives those checks. Two same-generation raw transform-converter snapshots can preserve the same transform type, target language, direction set, converter schema, and converter function names while differing in raw converter-definition material. Repeated identifiers therefore prove binding equality, not immutable ancestry. Finding review `5253398088` pinned this defect; behavioral RED `8ab7c431cdba2d0ba43a66313253b93b53b9e72a` keeps generation and reconstructed direction/function identities stable while changing the raw converter-definition root.

The ordinary-forward causal repair propagates the immutable raw transform-converter root digest through the converter-function successor chain as `converter_snapshot_digest`. At source head `7ee7847b629e4f2e5ec5a12f93f7da06f23f366c`, `IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new` first requires `converter_extension_membership_snapshot.converter_snapshot_digest()` to equal the separately supplied `transform_converter_snapshot.snapshot_digest()`. Only after exact root ancestry is proven do the retained direction cardinality and exact converter binding checks run. Root mismatch fails closed with `index_exclusion_constraint_operator_procedure_transform_extension_membership_lineage`.

This is an integrity repair, not a new PostgreSQL catalog successor. It neither infers extension membership from converter functions nor copies extension-owned truth. Transform-object membership remains one object-level fact, while converter-function membership remains its own lifecycle fact. Exact-head COMMENT review `5253528602` records source/contract coherence at `7ee7847b...`; it is not approval or GREEN evidence.

The extractor still must bind the exact `pg_transform` OID already resolved in the same source generation, inspect `pg_depend` for an extension-membership edge with `deptype='e'` and `objsubid=0`, and resolve the referenced extension OID to exact `pg_extension.extname` in that same snapshot. Zero edges means standalone. Unsupported ambiguous multiple membership must fail closed before contract construction.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: ALTER EXTENSION*. https://www.postgresql.org/docs/18/sql-alterextension.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html
- PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 documentation: 52.22. pg_extension*. https://www.postgresql.org/docs/18/catalog-pg-extension.html

## Observation versus governance

Source Observation records exact external database state before policy. It does not infer transform or converter-function extension membership. Validation/publish policy may later reject an unexpected member/standalone state, but it must reason over the observed edge instead of rewriting source facts.

The same rule continues to apply to mutable function state. Raw post-creation `provolatile='v'` remains observable drift evidence even though a volatile function is not admissible when creating a fresh transform. Observation and admission remain separate boundaries.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the same-name/different-definition raw-root regression plus retained direction/function drift regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation: preserve the existing converter facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function extension membership, exact raw FROM SQL / TO SQL direction-function bindings, and the transform object's own `pg_depend -> pg_extension` membership edge. The captured converter-function successor must remain bound to the exact raw converter root used for the same-generation transform binding. Reconstruction from naming conventions, packages, application metadata, converter-function membership, repeated identifiers, or another source generation is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The latest material finding is an ancestry invariant defect in the existing two-input composition, not another catalog-enumeration successor. The current `pg_transform` review still leaves no obvious uncaptured row field after OID/type/language/FROM SQL/TO SQL bindings, converter-function facts, converter-function extension membership, transform-object extension membership, and immutable raw-root binding.

No further converter/transform successor should be added merely by enumerating catalogs. A new successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that survives the existing bindings and memberships. Until such a distinction is proven, this sub-chain should move to exact-head acceptance plus the same-generation PostgreSQL 18 differential.

## Canonical prerequisite state

Fresh central authority remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`. The owner PR still requires ordinary/non-force path-wise reconciliation of the shared scheduler, preserving its v2 producer, no-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation, rationale/tests, and compatible current-main queue/coalescing/capacity behavior while enforcing the stronger repository-identity invariant. Whole-file ours/theirs or complete-file reconstruction is not an acceptable reconciliation.

Product bootstrap #35 remains OPEN / Ready at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its fresh body still records SAST Semgrep and Security Scan success but terminal CodeQL PR failure, so normal landing remains unauthorized. Sibling central or predecessor evidence does not transfer into #35 or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including immutable raw converter-root lineage plus converter-function and transform-object extension membership -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.