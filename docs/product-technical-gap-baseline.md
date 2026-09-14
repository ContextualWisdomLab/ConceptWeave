# Product / Technical Gap Baseline

**Snapshot:** 2026-09-14

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. The current lineage contains PostgreSQL 18 first-class NOT NULL evidence, signed-`int2` `coninhcount`, corrected PRIMARY KEY completeness, enforced-only NOT NULL source state, partition `NO INHERIT` rejection, parent-row/column resolution, parent-graph acyclicity, and direct declarative-partition parent validation against an independently captured `pg_inherits` witness. The latest finding is review `5194258395`; RED contract `51a8f2680943f2526b1f03712ebf92f5cad30c10`, production repair `4cbb426d045f290a061820593fee0a870c7cbd94`, retained-contract alignment `d7c11072ab1c03d928129b547be9b0d608fc8039`, and doctoring `00a760bc0a0dc1135af0cb04615d7cc14699931a` preserve the causal lineage. Exact-head native and hosted acceptance remain pending.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN Ready and mechanically mergeable. It remains the prerequisite for repository-owned Product `pull_request` evidence because the workflow is absent from protected ConceptWeave `main`.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column collation, generation declaration mode, default/generated expressions, identity declaration mode, and first-class NOT NULL constraint evidence. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Frozen `ColumnObservationV3` remains unchanged.

## PostgreSQL 18 NOT NULL constraint identity

`ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores explicit table NOT NULL specifications as `pg_constraint.contype = 'n'` rows. The domain-separated NOT NULL family retains exact schema/relation kind/relation/column coordinates, constraint name, `convalidated`, enforced source state, `conislocal`, `coninhcount`, `connoinherit`, and a stable parent coordinate when `conparentid != 0`. Catalog OIDs are capture-time joins and never governed identity.

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
- NOT NULL enforcement domain: `5193694445 -> 02207332... -> a47d916c... -> 948282a3... -> e35b880f...`;
- parent-constraint graph acyclicity: `5193997785 -> a8ef4462... -> 2a064395... -> 1d529c02...`;
- direct partition-parent relation integrity: `5194258395 -> 51a8f268... -> 4cbb426d... -> d7c11072... -> 00a760bc...`.

### Direct parent relation repair

The previous graph repair guaranteed that all parent-constraint edges resolve and are acyclic, but an acyclic edge could still target the wrong partitioned table. PostgreSQL 18 `pg_constraint.conparentid` identifies the corresponding constraint on the parent partitioned table; PostgreSQL 18 `pg_inherits` separately records each direct parent-child relation through `inhrelid` and `inhparent`. Declarative partitioning permits a partition no parent other than the partitioned table it belongs to. Therefore parent-constraint resolution and direct relation-parent resolution are separate source facts and must agree.

`NotNullConstraintObservation` now carries a direct partition-parent relation witness intended to be populated from the adapter's independent `pg_inherits` join. For every nonzero parent constraint, canonicalization requires this witness and exact schema/relation agreement with the relation owning the resolved `conparentid` target. A witness without a parent constraint, a missing witness, or a mismatched witness fails as `not_null_constraint_partition_parent_relation` before governed hashing. The relation coordinate is not hashed twice because the stable parent relation is already part of `ParentNotNullConstraintCoordinate`; the integrity property is the mandatory agreement of independently captured source joins.

Earlier repairs remain unchanged: `conenforced=false` NOT NULL rows fail closed; parent rows must exist in the same family; parent and child constrained columns must correspond; partition-linked children require `conislocal=false`, `coninhcount=1`, `connoinherit=false`; and the resolved parent-constraint graph must be acyclic.

Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / ENFORCEMENT_SOURCE_REPAIRED / PARENT_RESOLUTION_SOURCE_REPAIRED / PARENT_COLUMN_SOURCE_REPAIRED / PARENT_NO_INHERIT_SOURCE_REPAIRED / PARENT_GRAPH_SOURCE_REPAIRED / PARTITION_PARENT_RELATION_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host does not provide a usable repository Rust toolchain, and protected/default ConceptWeave `main` still lacks the Product PR workflow. The executable contract and source repair are therefore source-level evidence only; no local Rust RED/GREEN or hosted Product acceptance is claimed. Draft/Ready cycling, synthetic status, copied central workflows, manual/no-op reruns, self-approval, review dismissal, force-push, destructive rebase, or gate weakening are prohibited.

## Central Product-CI and review owners

Central workflow ownership remains outside ConceptWeave; central evidence never transfers to a ConceptWeave leaf head.

Protected `.github/main` is `7f07029381a9ca770d0a68b7f3938dd652799d4d` at this snapshot.

- `.github#2106@44901e45636e655cf84cc609e5fe62789216cde9` is OPEN Ready / mergeable. Runtime Quality `34794865763`, Semgrep `34794865812`, and Python Security `34794865728` are terminal success; CodeQL `34794865762` is in progress and Security Scan `34794865771` is queued.
- `.github#2170@d493cc4c53d0b77d67ef4af13005dcda8fb33c7f` is OPEN Ready / mergeable. Runtime Quality `34794949744` and Semgrep `34794949745` are success; Security `34794949825`, Python Security `34794949775`, and CodeQL `34794949758` remain queued.
- `.github#2079@2d27e0c13f9b118ca844f5299b0fcdde420fa66b` is OPEN Ready / mergeable. Semgrep `34794916614` is success; CodeQL `34794916559` is in progress; Python Security `34794916564` and Security `34794916586` are queued.
- ConceptWeave #35 exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` remains the Product-workflow bootstrap. Its historical CodeQL PR run is terminal failure; Semgrep and Security succeeded. Fresh compatible unchanged-head acceptance remains required before normal landing.

Only after central owners settle and #35 lands normally can one unchanged #46 head obtain meaningful hosted Product acceptance. The complete #46 delta then flows ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull`, bounded `pg_constraint.contype = 'n'` rows, referenced parent constraints, owning `pg_class` rows, and the child relation's direct `pg_inherits` edge inside the same catalog snapshot. `conkey` resolves to exactly one bounded column. A nonzero `conparentid` requires a real parent-constraint join, parent `pg_class.relkind = 'p'`, `conislocal=false`, `coninhcount=1`, `connoinherit=false`, corresponding constrained columns, an acyclic parent-constraint graph, and an independent direct `pg_inherits` witness whose `inhrelid` is the child relation and `inhparent` is the same parent relation that owns the resolved parent constraint. The adapter must not manufacture the relation witness from `conparentid`, caller text, names, or column shape. Missing, unauthorized, contradictory, duplicate, cross-column, cyclic, `NO INHERIT`, unenforced, detach-ambiguous, or partially resolved evidence fails closed before immutable snapshot construction.

For column collation, generation, identity, expressions, temporal constraints and backing indexes, the existing source-authoritative families and exact optional-family ordering remain unchanged.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_inherits*. https://www.postgresql.org/docs/18/catalog-pg-inherits.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table Partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
