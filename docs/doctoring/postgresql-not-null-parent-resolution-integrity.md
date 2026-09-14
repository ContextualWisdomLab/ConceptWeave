# PostgreSQL 18 NOT NULL parent-resolution integrity

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-14

## Problem

`pg_constraint.conparentid` is not descriptive text. PostgreSQL 18 defines it as a reference to the corresponding constraint row on the parent partitioned table. ConceptWeave had already replaced the database-local OID with a stable `ParentNotNullConstraintCoordinate` and validated the partition-specific `conislocal=false`, `coninhcount=1`, and parent relation-kind invariants. However, aggregate canonicalization did not prove that the supplied stable coordinate actually resolved to an observed parent `contype='n'` row in the same immutable NOT NULL family.

That gap allowed caller-constructed schema/relation/constraint strings to enter governed identity as if the `conparentid` OID had been resolved from source. The previous parent-contract fixture exposed the defect directly: a child constraint carrying a parent coordinate could produce a valid snapshot even when neither the parent relation nor the referenced parent NOT NULL constraint was present in the family.

## Primary-source basis

PostgreSQL 18 documents `pg_constraint.conparentid` as referencing `pg_constraint.oid` and identifying the corresponding constraint of the parent partitioned table when the row belongs to a partition. PostgreSQL 18 table-partitioning documentation also states that NOT NULL constraints of a partitioned table are inherited by all partitions. In `REL_18_STABLE`, `ConstraintSetParentConstraint()` records a partition dependency on the exact `parentConstrId`; the linkage is therefore to an existing parent constraint row, not to an arbitrary coordinate assembled by a consumer.

## Decision

A nonzero source `conparentid` may enter governed identity only after all of the following hold in one immutable observation family:

- the parent coordinate names a `RelationKind::PartitionedTable`;
- the child row has `conislocal=false` and `coninhcount=1`;
- a NOT NULL observation with the exact parent schema, relation kind, relation name, and constraint name exists in the same canonicalized family;
- normal relation/column completeness validation independently proves that the referenced parent constraint belongs to an observed relation and constrained column.

The model continues to exclude raw OIDs from governed identity. The stable coordinate remains the identity representation, but it is no longer trusted solely because a caller constructed it.

## Repair lineage

Review `5193008264` identified the unresolved-coordinate authority gap on exact predecessor `445acfff1516a69c9180a06f930ead952f0d9f57`.

RED `b20a54c3da3d2890b7e786fce7535fdd13bf6615` changes `not_null_constraint_parent_contract.rs` so valid partition-parent fixtures include the parent relation and parent NOT NULL row, and adds `unresolved_partition_parent_constraint_is_rejected`. The new regression requires `not_null_constraint_parent_coordinate` when a child claims a parent coordinate absent from the immutable NOT NULL family.

Production repair `7d62df1792ebcf4a015741496535dafebbc22148` adds the minimum aggregate check in `canonicalize_not_null_constraints()`: every attached parent coordinate must resolve to an exact observation in the same family before the snapshot can be hashed. Existing partition-kind, locality/count, signed-`int2`, `NO INHERIT`, nullability, duplicate-column, and completeness gates remain unchanged.

The repair is source-complete but not execution acceptance. Rust 1.98 fmt, strict Clippy, contract/workspace/doc tests, release build, owned coverage, hosted security/Product/review evidence, and the unchanged-head requirement remain separate gates.

## Adapter obligation

The future PostgreSQL transport must resolve `conparentid` while holding the same bounded read-only catalog snapshot used for `pg_constraint`/`pg_attribute` capture. It must not emit a parent coordinate unless the referenced parent `pg_constraint` row and parent `pg_class` row were actually joined and validated. A missing, out-of-scope without authorized expansion, contradictory, or partially resolved parent row fails closed before immutable snapshot construction.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
