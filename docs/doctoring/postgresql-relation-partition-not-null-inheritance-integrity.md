# PostgreSQL 18 relation-partition / NOT NULL inheritance integrity

## Decision

ConceptWeave treats declarative-partition membership and PostgreSQL 18 first-class `NOT NULL` constraint rows as independent source observations that must agree in both directions before immutable relation-partition evidence is admitted.

A positive `pg_class.relispartition` observation with direct `pg_inherits` parent P requires every observed inheritable `NOT NULL` constraint on P to have the corresponding inherited child `NOT NULL` row linked to that exact parent constraint. Existing child `conparentid` evidence must also continue to agree with the independently observed direct relation parent. Extra child-local `NOT NULL` constraints remain valid and do not imply additional relation ancestry.

Generic table inheritance is outside this relation-partition family. Catalog OIDs remain capture-time join coordinates and are not governed identity.

## Problem

The initial relation-partition implementation repaired the loss of `pg_class.relispartition`, direct `pg_inherits` parentage, detach state, and graph topology. Its NOT NULL cross-family validation was one-way: a child constraint that already carried `conparentid` had to agree with relation membership, but a positive relation-membership edge did not require inherited parent NOT NULL rows to be represented on the child.

That allowed an impossible pair of observations to be promoted together: relation evidence could say `events_2026` is a partition of `events`, while the child `id NOT NULL` row remained local and unlinked even though the partitioned parent carried the corresponding inheritable `id NOT NULL` constraint.

## PostgreSQL authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. PostgreSQL. https://www.postgresql.org/docs/18/ddl-partitioning.html

The declarative-partitioning contract states that `CHECK` and `NOT NULL` constraints of a partitioned table are always inherited by all partitions. Partitions also have exactly the same columns as their parent.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. PostgreSQL. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

`pg_constraint.conparentid` identifies the corresponding constraint of the parent partitioned table when the constraint belongs to a partition. `conislocal`, `coninhcount`, and `connoinherit` preserve the associated inheritance state.

PostgreSQL Global Development Group. (2026). *PostgreSQL REL_18_STABLE source: pg_constraint.c*. PostgreSQL source tree. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c

The source catalog implementation retains first-class NOT NULL rows and parent-constraint relationships rather than reducing them to display DDL or `attnotnull` alone.

## Exact ConceptWeave evidence

- Owner review finding: `5198504917` on #46 exact `551862ff8647a673bcab5a96404a02eb39a28465`.
- Behavioral RED source: `7b00339e699ea83c77ed9ac45057d51d73fa83e4`, `crates/conceptweave-relation-partition/tests/not_null_inheritance_contract.rs`.
- Minimal causal production repair: `c708d45429f9b942d8b5ee7628f6876595a2a856`, `crates/conceptweave-relation-partition/src/lib.rs`.
- Error boundary: `relation_partition_not_null_inheritance`.
- Retained prior mismatch boundary: `relation_partition_not_null_parent`.

The new regression contains both sides of the contract: an unlinked local child row must fail when relation membership proves a direct declarative-partition edge, while an inherited child row with `conislocal=false`, `coninhcount=1`, exact parent constraint coordinate, and matching direct `pg_inherits` witness is admissible.

## Adapter obligation

A PostgreSQL adapter must capture one bounded catalog state and resolve:

- `pg_class.relispartition` for every bounded relation;
- the direct `pg_inherits.inhparent` edge and `inhdetachpending` state for each declarative partition;
- every bounded PostgreSQL 18 `pg_constraint.contype='n'` row, including its column, `conparentid`, validation/enforcement/locality/inheritance flags; and
- parent constraint OIDs to exact immutable schema/relation/constraint coordinates before crossing the ACL.

It must not infer partition membership from relation names, `conparentid`, rendered DDL, or generic inheritance. It must not synthesize inherited NOT NULL linkage merely because the relation topology suggests it; missing or contradictory source evidence fails closed.

## Risk and follow-up

This repair is source-contract evidence, not execution acceptance. The exact successor still requires repository-pinned Rust 1.98 formatting, strict all-target/workspace Clippy, focused and retained contract tests, rustdoc/doc tests, release build, owned coverage, and applicable hosted Product/security/dependency/review evidence on one unchanged head. No predecessor GREEN transfers after head movement.
