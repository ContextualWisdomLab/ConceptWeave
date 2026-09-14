# Product / Technical Gap Baseline

**Snapshot:** 2026-09-14

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. The current lineage contains PostgreSQL 18 first-class NOT NULL evidence, signed-`int2` `coninhcount`, corrected PRIMARY KEY completeness, relation-kind-specific NOT NULL enforcement state, fail-closed table-like relation ownership, relation-wide `pg_constraint` name integrity across the standard and NOT NULL evidence families, partition `NO INHERIT` rejection, parent-row/column resolution, parent-graph acyclicity, direct declarative-partition parent validation against an independently captured `pg_inherits` witness, fail-closed `inhdetachpending` handling, PostgreSQL-compatible parent/child `convalidated` asymmetry, and fail-closed inheritance-origin coherence for `conislocal=false / coninhcount=0`. Review `5196207955` isolates the foreign-table enforcement gap; executable RED `6d2b22891aecdc6be6375a0ca56c648fc853b64b`, production repair `41de3e390905299c3b77e7ba9dbd950c2fa90d46`, governed-identity regression `5f786846a03974dbe8a6eb085fb9e151d20202d2`, and doctoring `72bdbc51383adcd6d817b20ab766eaa3971b8b11` preserve the current causal lineage. Exact-head native and hosted acceptance remain pending.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN Ready and mechanically mergeable. It remains the prerequisite for repository-owned Product `pull_request` evidence because the workflow is absent from protected ConceptWeave `main`.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column collation, generation declaration mode, default/generated expressions, identity declaration mode, and first-class NOT NULL constraint evidence. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Frozen `ColumnObservationV3` remains unchanged.

## PostgreSQL 18 NOT NULL constraint identity

`ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores explicit table NOT NULL specifications as `pg_constraint.contype = 'n'` rows. The domain-separated NOT NULL family retains exact schema/relation kind/relation/column coordinates, constraint name, `convalidated`, `conenforced`, `conislocal`, `coninhcount`, `connoinherit`, and a stable parent coordinate when `conparentid != 0`. Catalog OIDs are capture-time joins and never governed identity.

Retained repair lineage:

- first-class family and digest framing: `5189886354 -> 8ce7fd7c... -> 8912039b... -> c5ac66ff... -> e60cfec3...`;
- parent relation-kind validation: `5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07...`;
- signed-`int2` `coninhcount`: `5190362750 -> 28916758... -> 385bf437... -> 1b16ce65...`;
- PostgreSQL 18 PRIMARY KEY completeness: `5190470810 -> d4df54d9... -> 0cdd4fd3... -> 6c37275c...`;
- partitioned-table `NO INHERIT`: `8ef85d71... -> 7331fe06... -> e311a497...`;
- child locality/direct-ancestor tuple: `5192015894 -> 57c7f6c4... -> 03bb42ac... -> 4f032eb8...`;
- same-family parent-row resolution: `5193008264 -> b20a54c3... -> 7d62df17... -> 238a4811...`;
- corresponding parent-column integrity: `5193201137 -> 212d17e9... -> cfe1851f... -> d1012cd3...`;
- partition-parent inheritance flags: `5193490988 -> 1bd8e20e... -> e1618a20... -> c84ad970...`;
- ordinary-table NOT NULL enforcement domain: `5193694445 -> 02207332... -> a47d916c... -> 948282a3... -> e35b880f...`;
- foreign-table NOT NULL enforcement correction: `5196207955 -> 6d2b2289... -> 41de3e39... -> 5f786846... -> 72bdbc51...`;
- parent-constraint graph acyclicity: `5193997785 -> a8ef4462... -> 2a064395... -> 1d529c02...`;
- direct partition-parent relation integrity: `5194258395 -> 51a8f268... -> 4cbb426d... -> d7c11072... -> 610a5575...`;
- direct-parent detach-state integrity: `5194302440 -> ee12c677... -> e4d33a9d... / 4359936e... -> eaa354bb... -> 5f557299...`;
- inherited parent-validation compatibility: `5194758807 -> 6cdadfa9... -> 503e88bb... -> 1e0514e3...`;
- inheritance-origin coherence: `5195128595 -> 2940852b... -> 8a7dfae2... -> ec0e23e6...`;
- NOT NULL owning relation-kind integrity: `5195739325 -> 7c39e5ef... -> 2e3be259... -> 461959dd...`;
- relation-wide constraint-name integrity: `5195981967 -> 79d91622... -> aaf4b314... -> 0cff8f0a... -> ffad14c0...`.

### Direct parent relation and detach-state repair

PostgreSQL 18 `pg_constraint.conparentid` identifies the corresponding constraint on the parent partitioned table, while `pg_inherits` separately records every direct parent-child relation with `inhrelid`, `inhparent`, and `inhdetachpending`. Declarative partitioning permits a partition no parent other than the partitioned table it belongs to. Parent-constraint resolution and the direct relation edge are therefore separate source facts and must agree.

`NotNullConstraintObservation` carries a direct parent witness intended to be populated from the adapter's independent `pg_inherits` join. For every nonzero parent constraint, canonicalization requires exact schema/relation agreement with the relation owning the resolved `conparentid` target. A witness without a parent constraint, a missing witness, or a mismatched witness fails as `not_null_constraint_partition_parent_relation` before governed hashing. The witness API also requires the observed `inhdetachpending` value; `true` fails immediately as `not_null_constraint_partition_parent_detach_pending` rather than being silently normalized to stable partition membership.

The stable parent relation coordinate is not hashed twice because it already participates in governed identity through `ParentNotNullConstraintCoordinate`; the integrity property is the mandatory agreement of independently captured source joins and explicit rejection of transitional detach state.

### Inherited validation-state compatibility

`convalidated=false` remains a legitimate first-class PostgreSQL 18 NOT NULL state and stays in governed identity. The parent-edge rule is narrower. PostgreSQL `REL_18_STABLE` `AdjustNotNullInheritance()` rejects an existing child NOT NULL constraint that is still NOT VALID when the desired inherited parent constraint is valid, while explicitly allowing the opposite direction. Canonicalization therefore rejects `parent.validated() && !child.validated()` as `not_null_constraint_parent_validation` only after the exact parent row and corresponding column have resolved. A NOT VALID parent with an already-valid child remains accepted.

This preserves source semantics rather than turning validation into a symmetric equality check. The executable regression contains both directions so a future refactor cannot “simplify” the asymmetric PostgreSQL rule into an invalid global prohibition on `NOT VALID`.

### Inheritance-origin coherence

PostgreSQL 18 creates a new inherited NOT NULL row with `conislocal=false` and `coninhcount=1`; an inherited addition to an existing NOT NULL increments `coninhcount`, and declarative partition parent assignment likewise sets `conislocal=false` while incrementing the count from zero to one. Therefore `conislocal=false / coninhcount=0` has neither a local origin nor an inheritance ancestor and is not a source-representable NOT NULL state.

`NotNullConstraintObservation::new()` rejects only that impossible combination as `not_null_constraint_inheritance_origin`. Pure local `(true,0)`, inherited-only `(false,>=1)`, and local-plus-inherited `(true,>=1)` remain valid. This intentionally does not infer one catalog field from the other; the adapter must capture both independently from the same bounded snapshot.

### Owning relation-kind integrity

The v3 relation vocabulary is intentionally broader than PostgreSQL table-constraint ownership: it includes ordinary tables, partitioned tables, foreign tables, views, materialized views, sequences, and standalone composite types. Merely resolving a relation and a non-nullable column is therefore insufficient evidence that `pg_constraint.contype='n'` can exist on that relation.

`NotNullConstraintObservation::new()` admits only `RelationKind::Table`, `RelationKind::PartitionedTable`, and `RelationKind::ForeignTable`. PostgreSQL 18 `CREATE TABLE` owns ordinary/partitioned NOT NULL constraints; `CREATE FOREIGN TABLE` also admits NOT NULL and foreign-table partitions. View, materialized-view, sequence, and standalone composite-type observations fail as `not_null_constraint_relation_kind` before governed identity is created. The rule does not conflate foreign-table constraint declaration with remote enforcement: source truth and enforcement semantics remain separate.

### Foreign-table enforcement integrity

PostgreSQL 18 has distinct ordinary-table and foreign-table constraint grammars. `CREATE TABLE` currently permits `NOT ENFORCED` only for foreign-key and `CHECK` constraints, so ordinary and partitioned-table NOT NULL rows with `conenforced=false` remain rejected. `CREATE FOREIGN TABLE`, however, explicitly permits `[ ENFORCED | NOT ENFORCED ]` on column-level and table-level `NOT NULL` constraints and explains that PostgreSQL core does not itself enforce foreign-table CHECK/NOT NULL declarations.

`NotNullConstraintObservation::new()` therefore scopes the fail-closed enforcement guard to non-foreign owners. `RelationKind::ForeignTable` preserves both observed `conenforced` values, and the existing digest already hashes the bit. The focused regression now constructs both states as bounded foreign-table snapshots and proves their digests differ. The adapter must capture the catalog value; it may neither normalize it to true based on ordinary-table semantics nor synthesize false from the general fact that core PostgreSQL does not enforce foreign-table constraints.

### Relation-wide constraint-name integrity

PostgreSQL 18 stores CHECK, NOT NULL, PRIMARY KEY, UNIQUE, FOREIGN KEY, and exclusion constraints in `pg_constraint`, and requires constraint names to be unique across the constraints attached to a particular table or domain. ConceptWeave intentionally models first-class NOT NULL rows in a separate optional family, but that representation split is not a second source naming namespace.

`canonicalize_not_null_constraints()` therefore resolves the exact owning relation and rejects a NOT NULL `constraint_name` already present in that relation's standard `TableConstraintObservation` collection with the existing `DuplicateConstraintName` error. Same-name constraints on different relations remain valid, matching PostgreSQL rather than the SQL standard's stricter schema-wide naming rule. The preservation fixture keeps the CHECK-only relation nullable so NOT NULL-family completeness does not introduce an unrelated missing-row error. The adapter must preserve exact names and kinds; it must not rename or pre-deduplicate source rows to avoid this check.

Earlier repairs remain unchanged: unsupported ordinary/partitioned-table `conenforced=false` NOT NULL rows fail closed; parent rows must exist in the same family; parent and child constrained columns must correspond; partition-linked children require `conislocal=false`, `coninhcount=1`, `connoinherit=false`; and the resolved parent-constraint graph must be acyclic.

Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_KIND_SOURCE_REPAIRED / CONSTRAINT_NAME_SOURCE_REPAIRED / ENFORCEMENT_SOURCE_REPAIRED / FOREIGN_TABLE_ENFORCEMENT_SOURCE_REPAIRED / INHERITANCE_ORIGIN_SOURCE_REPAIRED / PARENT_RESOLUTION_SOURCE_REPAIRED / PARENT_COLUMN_SOURCE_REPAIRED / PARENT_NO_INHERIT_SOURCE_REPAIRED / PARENT_GRAPH_SOURCE_REPAIRED / PARTITION_PARENT_RELATION_SOURCE_REPAIRED / PARTITION_PARENT_DETACH_STATE_SOURCE_REPAIRED / PARENT_VALIDATION_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host does not provide a usable repository Rust toolchain, and protected/default ConceptWeave `main` still lacks the Product PR workflow. The executable contracts and source repairs are source-level evidence only; no local Rust RED/GREEN or hosted Product acceptance is claimed. Draft/Ready cycling, synthetic status, copied central workflows, manual/no-op reruns, self-approval, review dismissal, force-push, destructive rebase, or gate weakening are prohibited.

## Central Product-CI and review owners

Central workflow ownership remains outside ConceptWeave; central evidence never transfers to a ConceptWeave leaf head. Protected `.github/main` is `91be6442906c7b6b4f600272c953699708394327` after ordinary protected merge #2194.

- `.github#2106` has already adopted the protected advance through ordinary non-force helper #2195. Its current exact head is `9defd52f4a3b42d6a63a9520d6da82224d8c864d`, OPEN Draft / mechanically mergeable on direct base `main@91be6442906c7b6b4f600272c953699708394327`. Fresh exact-head Runtime Quality `34825970200`, Python Security `34825970282`, Security Scan `34825970259`, and SAST Semgrep `34825970371` remain queued; CodeQL PR `34825970251` remains pending. Its current owner-local documentation finding is incomplete cross-repository identity in `docs/product-technical-gap-baseline.md`; the durable references must include `ContextualWisdomLab/` owner names before this Draft can seek fresh acceptance. No prior #2106 evidence transfers to this restacked head.
- `.github#2170@c346b8324fa23e23d4007799d26ad3a8ac6ae4c3` is OPEN Ready / mechanically mergeable directly on current protected main after ordinary reverse helper #2199. Fresh exact-head Runtime Quality `34826735972`, Security Scan `34826736000`, Python Security `34826735889`, SAST Semgrep `34826735939`, and CodeQL PR `34826735991` are non-terminal; predecessor evidence does not transfer.
- `.github#2079@2b2f348d823d48f81ea6c1f7e36d0870ce35e5c4` is OPEN Ready / mechanically mergeable directly on current protected main after ordinary reverse helper #2198. Fresh CodeQL PR `34826683838`, SAST Semgrep `34826683831`, Security Scan `34826683861`, and Python Security `34826683797` are non-terminal; a fresh Required OpenCode generation and qualifying independent review must also bind this head.
- ConceptWeave #35 exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` remains the Product-workflow bootstrap. Its historical CodeQL PR run is terminal failure; Semgrep and Security succeeded. Fresh compatible unchanged-head acceptance remains required before normal landing.

No ConceptWeave leaf copies or edits these central owner sources. #2106 proceeds through its owner-local traceability repair and fresh exact-head gates; #2170/#2079 have adopted protected `91be6442...` but must complete their own fresh acceptance. Only after central owners settle and #35 lands normally can one unchanged #46 head obtain meaningful hosted Product acceptance. The complete #46 delta then flows ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull`, bounded `pg_constraint.contype = 'n'` rows including exact `conenforced`, referenced parent constraints, owning `pg_class` rows, the complete bounded standard `pg_constraint` naming inventory for each relation, and the child relation's direct `pg_inherits` edge inside the same catalog snapshot. `conkey` resolves to exactly one bounded column. The owning `pg_class.relkind` must resolve without coercion to ordinary table (`r`), partitioned table (`p`), or foreign table (`f`); other modeled relation kinds fail closed. Ordinary/partitioned NOT NULL with `conenforced=false` fails closed, while foreign-table `conenforced` is preserved as observed. Exact `conname` is preserved across kinds, and a NOT NULL name colliding with another constraint on the same owning relation fails closed rather than being renamed or normalized. `conislocal` and signed-`int2` `coninhcount` are captured independently; `conislocal=false / coninhcount=0` fails closed rather than being normalized. A nonzero `conparentid` requires a real parent-constraint join, parent `pg_class.relkind = 'p'`, `conislocal=false`, `coninhcount=1`, `connoinherit=false`, corresponding constrained columns, PostgreSQL-compatible parent/child `convalidated` state, an acyclic parent-constraint graph, and an independent direct `pg_inherits` witness whose `inhrelid` is the child relation, `inhparent` is the same parent relation that owns the resolved parent constraint, and `inhdetachpending=false`. The adapter must not manufacture the witness from `conparentid`, caller text, names, or column shape, nor default a missing detach-state, enforcement, or validation value. Missing, unauthorized, contradictory, origin-less, unsupported-relation-kind, duplicate, cross-family-name-colliding, cross-column, cyclic, invalid parent/child validation, unsupported ordinary-table `NOT ENFORCED`, `NO INHERIT`, detach-pending, or partially resolved evidence fails closed before immutable snapshot construction.

For column collation, generation, identity, expressions, temporal constraints and backing indexes, the existing source-authoritative families and exact optional-family ordering remain unchanged.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_inherits*. https://www.postgresql.org/docs/18/catalog-pg-inherits.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table Partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-createforeigntable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Composite Types*. https://www.postgresql.org/docs/18/rowtypes.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: heap.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/heap.c
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
