# Product / Technical Gap Baseline

**Snapshot:** 2026-09-14

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. Its current lineage contains the PostgreSQL 18 first-class NOT NULL family, signed-`int2` `coninhcount` bound, corrected PRIMARY KEY completeness, enforced-only NOT NULL source domain, partitioned-table `NO INHERIT` rejection, partition-child locality/direct-ancestor validation, same-family parent-row resolution, corresponding-column validation, and partition-parent `NO INHERIT` rejection for nonzero `conparentid`. Latest enforcement review `5193694445` is followed by executable RED `02207332a7a1e437e20424d3057ed8bd25817637`, production repair `a47d916c212db16a4acbcde317a01a03ce14d81e`, contract alignment `948282a3615b613175a8fa0f6c49e6a232f686d0`, and primary-source doctoring `e35b880f484860c86e27b986c02590adf69686e2`. Exact-head native and hosted acceptance remain pending.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN Ready and mechanically mergeable. It is the canonical prerequisite for repository-owned Product `pull_request` evidence because the workflow is still absent from protected ConceptWeave `main`.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column collation, generation declaration mode, default/generated expressions, identity declaration mode, and first-class NOT NULL constraint evidence. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained source-repair families include exclusion-operator inference removal, PERIOD referenced-key completeness, temporal backing-index shape/lifecycle coherence, source collation and FK consistency, identity/nullability consistency, generation/default expression identity, and PostgreSQL 18 first-class NOT NULL identity. Frozen `ColumnObservationV3` remains unchanged. The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`.

## PostgreSQL 18 NOT NULL constraint identity

`ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores explicit table NOT NULL specifications as `pg_constraint.contype = 'n'` rows. The domain-separated NOT NULL family retains exact schema/relation kind/relation/column coordinate, constraint name, `convalidated`, the source `conenforced` bit under the invariant that it must be true, `conislocal`, `coninhcount`, `connoinherit`, and a stable parent coordinate when `conparentid != 0`. Catalog OIDs are capture-time joins and never governed identity.

The retained lineage is:

- initial first-class family and canonical digest framing: `5189886354 -> 8ce7fd7c... -> 8912039b... -> c5ac66ff... -> e60cfec3...`;
- partition-parent relation-kind validation: `5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07...`;
- signed-`int2` `coninhcount` domain: `5190362750 -> 28916758... -> 385bf437... -> 1b16ce65...`;
- final PostgreSQL 18 PRIMARY KEY completeness correction: `5190470810 -> d4df54d9... -> 0cdd4fd3... -> 6c37275c...`;
- partitioned-table `NO INHERIT` rejection: `8ef85d71... -> 7331fe06... -> e311a497...`;
- partition-child `conparentid` locality/direct-ancestor tuple: `5192015894 -> 57c7f6c4... -> 03bb42ac... -> 4f032eb8...`;
- same-family parent-row resolution integrity: `5193008264 -> b20a54c3... -> 7d62df17... -> 238a4811...`;
- corresponding parent-column integrity: `5193201137 -> 212d17e9... -> cfe1851f... -> d1012cd3...`;
- partition-parent inheritance-flag integrity: `5193490988 -> 1bd8e20e... -> e1618a20... -> c84ad970...`;
- NOT NULL enforcement-domain integrity: `5193694445 -> 02207332... -> a47d916c... -> 948282a3... -> e35b880f...`.

The latest repair closes an impossible source-domain state that the original contract explicitly admitted. PostgreSQL 18 exposes generic `ENFORCED`/`NOT ENFORCED` constraint syntax but documents `NOT ENFORCED` as currently supported only for foreign-key and CHECK constraints. The PostgreSQL 18 release announcement describes the same boundary: foreign keys and CHECK constraints gain `NOT ENFORCED`, while NOT NULL gains names, `NOT VALID`, and `NO INHERIT`. Therefore a caller-supplied `contype='n'` row with `conenforced=false` is contradictory evidence rather than a second valid NOT NULL identity. `NotNullConstraintObservation::new` now rejects it as `not_null_constraint_enforcement`; `NOT VALID` remains valid and continues to alter governed identity.

The previous repair closes a separate authority gap at parent attachment. An ordinary `RelationKind::Table` NOT NULL row may use `NO INHERIT` for classic inheritance, so the constructor continues to allow that state. Once a nonzero partition-parent coordinate is attached, however, the row represents the inherited copy of a partitioned-table constraint. PostgreSQL declarative partitioning always inherits parent NOT NULL/CHECK constraints into partitions, and an attached partition must carry the parent's NOT NULL/CHECK constraints without `NO INHERIT`. `with_parent_constraint()` therefore rejects `connoinherit=true` as `not_null_constraint_parent_no_inherit`; generic inheritance with `conparentid=0` remains unchanged.

The parent-resolution repair resolves every `ParentNotNullConstraintCoordinate` to an observed parent row in the same family and requires `parent_observation.column_name() == observation.column_name()`. Missing rows fail `not_null_constraint_parent_coordinate`; cross-column links fail `not_null_constraint_parent_column`. Parent and child constraint names are not required to be equal, and raw OIDs never become governed identity.

PostgreSQL 18 `pg_constraint` defines `conparentid` as the corresponding parent-partition constraint, `connoinherit` as the non-inheritable flag, and `conkey` as constrained column numbers. PostgreSQL 18 partitioning rules make parent NOT NULL constraints mandatory on partitions; `ALTER TABLE ... ATTACH PARTITION` requires the attached table to have the parent's NOT NULL and CHECK constraints without `NO INHERIT`. `REL_18_STABLE` `findNotNullConstraintAttnum()` selects a NOT NULL row by relation plus exact constrained attribute number before partition linkage. The governed edge therefore binds parent-row identity, corresponding constrained column, and an inheritance-compatible child flag tuple.

### Corrected PRIMARY KEY chronology

An April 2024 development-state discussion temporarily motivated treating PRIMARY KEY as sufficient backing for `attnotnull=true` without a separate NOT NULL row. Later PostgreSQL work superseded that model: PostgreSQL 18 PRIMARY KEY processing creates/queues first-class NOT NULL constraints. The bounded captured `contype='n'` column set therefore must equal the captured `attnotnull=true` set, including PRIMARY KEY columns. The superseded intermediate evidence remains historical only.

Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / ENFORCEMENT_SOURCE_REPAIRED / PARENT_RESOLUTION_SOURCE_REPAIRED / PARENT_COLUMN_SOURCE_REPAIRED / PARENT_NO_INHERIT_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current #46 generation cannot yet obtain repository-owned Product workflow evidence because protected ConceptWeave `main` still lacks the Product PR workflow. The current execution host also does not provide `cargo`/`rustc`, so the new executable RED, production repair, and aligned contract are source-level evidence only; no local Rust RED/GREEN claim is made. Do not toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, self-approve, dismiss review, force-push, destructively rebase, or weaken a gate.

## Central Product-CI and review owners

Central workflow ownership remains outside ConceptWeave; central evidence never transfers to a ConceptWeave leaf head.

Protected `.github/main` is `7f07029381a9ca770d0a68b7f3938dd652799d4d`.

- `.github#2170` is exact `d493cc4c53d0b77d67ef4af13005dcda8fb33c7f`, OPEN Ready and mechanically mergeable. Exact-head Agent Review Runtime Quality `34794949744` and SAST Semgrep `34794949745` are terminal success; Security Scan `34794949825`, Python Security `34794949775`, and CodeQL PR `34794949758` remain queued. Its body still describes predecessor coordinates, so metadata/current head is authoritative.
- `.github#2106` is exact `44901e45636e655cf84cc609e5fe62789216cde9`, OPEN Ready and mechanically mergeable. Exact-head Agent Review Runtime Quality `34794865763` and SAST Semgrep `34794865812` are terminal success; Security Scan `34794865771`, Python Security `34794865728`, and CodeQL PR `34794865762` remain queued. This remains the canonical protected CodeQL handler bootstrap; no leaf workaround is allowed.
- `.github#2079` is exact `2d27e0c13f9b118ca844f5299b0fcdde420fa66b`, OPEN Ready and mechanically mergeable. Exact-head SAST Semgrep `34794916614` is terminal success; Python Security `34794916564`, CodeQL PR `34794916559`, and Security Scan `34794916586` remain queued. The finding↔confirmed-probe and touched-callable docstring repairs remain owner-local. Its body also contains predecessor coordinates; metadata/current head is authoritative.
- ConceptWeave #35 exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` remains the canonical Product-workflow bootstrap. Its historical CodeQL result does not authorize a current merge; central handler/review settlement and fresh unchanged-head acceptance are required before normal landing.

Only after central owners settle and #35 lands normally can one unchanged #46 head obtain meaningful hosted Product acceptance. The complete #46 delta then flows ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull`, all bounded `pg_constraint.contype = 'n'` rows, and parent joins inside the same catalog snapshot. `conkey` resolves to exactly one bounded column. `conenforced=false` is contradictory for a NOT NULL row and fails before immutable representation; `convalidated=false` remains valid NOT NULL evidence. `coninhcount` is read as signed `int2`; negative or out-of-domain values fail before conversion. A nonzero `conparentid` requires a real same-snapshot join to the referenced parent `pg_constraint` row and parent `pg_class.relkind = 'p'`, plus `conislocal=false`, `coninhcount=1`, and `connoinherit=false`. The adapter must extract the referenced parent row's NOT NULL `conkey`, resolve the parent attribute, and prove that it is the corresponding partition column before emitting the stable parent coordinate. It must not emit a parent coordinate from caller text or a partial join. Missing, unauthorized out-of-scope, contradictory, duplicate, cross-column, `NO INHERIT`, unenforced, or partially resolved parent evidence fails closed before immutable snapshot construction.

For column collation, generation, identity, expressions, temporal constraints and backing indexes, the existing source-authoritative families and exact optional-family ordering remain unchanged. Expression evidence preserves exact server-rendered `pg_get_expr(adbin, adrelid)` while catalog OIDs/internal node serialization remain capture-time details only.

## Primary authority

- PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/18/release-18.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table Partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
- Herrera, Á. (2024, September 25). *Re: not null constraints, again* [PostgreSQL hackers message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202409252014.74iepgsyuyws%40alvherre.pgsql
- Herrera, Á. (2025, April 1). *Re: Support NOT VALID / VALIDATE constraint options for named NOT NULL constraints* [PostgreSQL hackers message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202504012022.wzrtvfhrltud%40alvherre.pgsql
- Herrera, Á. (2024, April 12). *Re: Can't find not null constraint, but \\d+ shows that* [superseded development-state evidence]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202404120752.6ebv4q5zwnfw%40alvherre.pgsql
