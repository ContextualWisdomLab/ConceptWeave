# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The pre-lineage-repair transform-object extension-membership surface is preserved at `docs/archive/product-technical-gap-baseline-through-fcaa3309.md`; earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The ordinary-forward converter chain retains all previously established converter definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, converter-function extension membership, and independent transform-object extension membership.

The transform-object successor remains one fact per exact `(constraint, key_position, transform_type, target_language)` `pg_transform` row. PostgreSQL 18 defines independent optional FROM SQL and TO SQL conversion functions for that row, so direction and exact converter function identity are material raw binding facts even though extension membership itself is object-level rather than direction-level.

Fresh review found an exact-lineage defect in the two-input constructor. `IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new` accepted a latest converter-function extension-membership snapshot plus a separately supplied raw transform-converter snapshot. It compared source connection, policy binding, extractor revision and observation timestamp, then only required that some converter-membership observation shared `(constraint, key_position, transform_type)`. A same-generation raw transform row with only FROM SQL could therefore be combined with a predecessor attesting both FROM SQL and TO SQL, and function identity could drift as well, without invalidating transform-object membership construction.

Cross-input lineage repair: finding review `5253304917` on exact `fcaa330965915a85ea7116a66c04286b79ef5931` -> hostile direction-set RED `3f3a947e31c3b80e1a66120b89705ec3e515e43c` -> minimum production repair `37742d774d7c33a693dc3a0359a6583134bc5cbf` -> converter-function identity edge contract `276821d5d5fdafbd4776c9856acfc04221e470c1` -> focused doctoring `71fd68c93805d856171653e63c575075a7f5e16c`.

The repaired constructor now requires the number of nonzero raw converter directions to equal the predecessor membership observation count and requires each raw FROM SQL / TO SQL converter to match exact constraint coordinate, key position, transform type, direction, converter schema and converter function name. This proves the two supplied snapshots describe the same converter binding before issuing the object-level extension-membership successor. It does not add a catalog field or duplicate the membership fact per direction.

The extractor still must bind the exact `pg_transform` OID already resolved in the same source generation, inspect `pg_depend` for an extension-membership edge with `deptype='e'` and `objsubid=0`, and resolve the referenced extension OID to exact `pg_extension.extname` in that same snapshot. Zero edges means standalone. Unsupported ambiguous multiple membership must fail closed before contract construction.

Converter-function extension membership is not a proxy for transform-object membership. Naming conventions, installed extension lists, function namespaces, application metadata or another source generation are not substitutes. The successor also does not copy `pg_extension.extversion`, extension configuration, control-file metadata, update scripts or other extension-owned truth.

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

**Source repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation: preserve the existing converter facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function extension membership, exact raw FROM SQL / TO SQL direction-function bindings, and the transform object's own `pg_depend -> pg_extension` membership edge. Reconstruction from naming conventions, packages, application metadata, converter-function membership or another source generation is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The latest finding was not another catalog-enumeration successor; it was an integrity defect in the existing two-input lineage. The current `pg_transform` review still leaves no obvious uncaptured row field after OID/type/language/FROM SQL/TO SQL bindings and transform-object extension membership. Normal dependency edges to those already-bound objects remain consequences of existing facts rather than new domain truth.

No further converter/transform successor should be added merely by enumerating catalogs. A new successor requires proof of an independent buyer, semantic, security, lifecycle or recovery distinction that survives the existing bindings and memberships. Until such a distinction is proven, this sub-chain should move to exact-head acceptance plus the same-generation PostgreSQL 18 differential.

## Canonical prerequisite state

Central `.github#2040` remains a separate canonical workflow prerequisite. Its last verified authority before this source movement was `12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`. It requires ordinary/non-force path-wise reconciliation of the shared scheduler while preserving its v2 producer, no-restamp behavior, repository-scoped credential proof, stale-run revalidation, rationale/tests, compatible current-main queue/coalescing/capacity behavior, and stronger repository-identity invariant. Ending fresh sweeps, not this paragraph, determine whether those coordinates remain current.

Queue-health and CodeQL dispatch repairs remain central-owner concerns. Current live PR authority must be used rather than stale issue references; the previously documented standalone `#2277` issue could not be verified in the fresh owner inventory and is not treated as a blocker or owner coordinate. Sibling central evidence does not transfer into #2040 or #46.

Product bootstrap #35 remains a separate prerequisite; its previously observed successful security lanes do not override terminal CodeQL failure evidence.

## Required order

fresh central prerequisite verification -> `.github#2040` causal reconciliation/repair if still current -> fresh central exact-head GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> exact #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including exact converter direction/function lineage plus converter-function and transform-object extension membership -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
