# PostgreSQL 18 NOT NULL partition-parent relation integrity

## Decision

A nonzero `pg_constraint.conparentid` is not sufficient by itself to establish the owning relation's declarative-partition parent. ConceptWeave therefore treats the parent-constraint join and the direct relation-parent join as separate source facts that must agree before a PostgreSQL 18 NOT NULL observation can enter governed identity.

PostgreSQL 18 defines `conparentid` as the corresponding constraint of the parent partitioned table when the constraint belongs to a partition. PostgreSQL 18 `pg_inherits` separately records one row for every direct parent-child table or index relationship: `inhrelid` is the child and `inhparent` is the parent. Declarative partitioning further restricts a partition to the partitioned table it belongs to; it cannot acquire some other inheritance parent. These facts make the `pg_inherits` edge an independent source witness for the relation owning a parent-linked constraint.

## Defect

Exact predecessor `e41749226aa00ce08fd5dc7bf3796ab977edaf02` resolved a nonzero `conparentid` to a real same-snapshot NOT NULL row, required that row to belong to a `PartitionedTable`, required the same constrained column, enforced the child locality/direct-ancestor/`NO INHERIT` tuple, and rejected cycles. Those checks still allowed a child constraint to name an otherwise valid constraint on the wrong partitioned table. If two observed partitioned tables exposed compatible NOT NULL rows, the false edge could satisfy every local invariant because the representation had no independently captured child-to-parent relation fact.

Review `5194258395` records this as a source-integrity finding. Executable contract `51a8f2680943f2526b1f03712ebf92f5cad30c10` requires a parent-linked child without a direct relation witness to fail as `not_null_constraint_partition_parent_relation`.

## Repair

Production repair `4cbb426d045f290a061820593fee0a870c7cbd94` adds a direct partition-parent relation witness to `NotNullConstraintObservation`. The witness is attached from the adapter's independent `pg_inherits` join. Canonicalization now requires the witness whenever `parent_constraint` is present and requires its exact schema/relation coordinate to equal the relation owning the resolved `conparentid` target. A witness without `conparentid` is also contradictory and fails closed.

Contract alignment `d7c11072ab1c03d928129b547be9b0d608fc8039` updates retained valid fixtures to carry the direct-parent witness and adds a second negative case in which the constraint points to `public.metric` while the relation witness points to `public.metric_archive`.

The witness is validation evidence rather than a second semantic coordinate. The resolved parent relation is already part of `ParentNotNullConstraintCoordinate` and therefore already participates in the NOT NULL digest. Hashing the same stable relation coordinate a second time would not create new semantic information; the security property comes from requiring two independently captured catalog joins to agree before hashing.

## Adapter obligation

The future PostgreSQL adapter must capture these facts inside one bounded `REPEATABLE READ READ ONLY` catalog snapshot:

- the child `pg_constraint` row and nonzero `conparentid`;
- the referenced parent `pg_constraint` row and its owning `pg_class` row;
- the child relation's direct `pg_inherits` row with `inhrelid = child` and `inhparent = parent`;
- the parent and child constrained attributes resolved from `conkey`;
- the existing `conislocal=false`, `coninhcount=1`, `connoinherit=false`, enforcement, completeness, and acyclicity invariants.

The adapter must not manufacture the relation witness from the already-resolved `conparentid` target. Missing, multiple, unauthorized, detach-ambiguous, or mismatched direct-parent evidence must fail closed or be represented by a later explicitly versioned capture policy; it must not silently collapse to a normal attached partition. Catalog OIDs remain capture-time join keys only and never become governed identity.

## Alternatives rejected

Inferring the parent relation from the parent constraint alone was rejected because it merely restates the original assertion. Matching by constraint name or column shape was rejected because neither proves the direct declarative-partition edge. Hashing raw OIDs was rejected because OIDs are database-local catalog identifiers, not stable governed coordinates. Treating acyclicity as sufficient was rejected because an acyclic graph can still point to the wrong partitioned table.

## Acceptance

The source and executable contracts are present, but no Rust RED/GREEN execution or hosted exact-head acceptance is claimed for this repair. One unchanged #46 head still requires repository-pinned Rust 1.98 formatting, strict Clippy, contract/workspace/doc tests, release build, owned coverage, and applicable Product/security/dependency/review evidence after the Product workflow bootstrap lands on protected ConceptWeave `main`.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_inherits*. https://www.postgresql.org/docs/18/catalog-pg-inherits.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
