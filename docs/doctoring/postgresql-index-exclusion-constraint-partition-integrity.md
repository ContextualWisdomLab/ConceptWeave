# PostgreSQL 18 partitioned exclusion-constraint catalog integrity

## Decision

ConceptWeave must retain partitioned `EXCLUDE` constraints as explicit source catalog evidence, not merely as exclusion-index operator semantics. The source fact is the `pg_constraint.contype = 'x'` row together with its resolved `conindid`, `conparentid`, `conislocal`, and `coninhcount` values. This evidence is layered over the existing immutable `IndexPartitionSnapshot` under a new digest domain; issued v3, relation-partition, index-partition, key-constraint, operator-family, and exclusion-semantics digests are unchanged.

## Problem

The Source Observation stack already preserved `pg_index.indisexclusion` and exact exclusion operator/procedure/strategy evidence. It did not preserve the independent `pg_constraint` object for an ordinary exclusion constraint. A PostgreSQL 18 partitioned `EXCLUDE` hierarchy could therefore retain compatible parent/child indexes while losing the constraint identity, the `conindid` link to each backing index, the exact `conparentid` edge, or the `(conislocal, coninhcount)` inheritance state.

That gap is material because PostgreSQL treats the index and constraint relationships separately. In `ATExecAttachPartitionIdx()`, after index-definition compatibility is established, PostgreSQL obtains the parent constraint through `get_relation_idx_constraint_oid()`, requires a child constraint for the child index, attaches the index, and then calls `ConstraintSetParentConstraint()` for the constraint edge. `ConstraintSetParentConstraint()` sets a child constraint to `conislocal=false`, increments `coninhcount` from zero to one, and writes `conparentid`; detach reverses that state. `DefineIndex()` uses the same generic `createdConstraintId`/`parentConstraintId` recursion for `EXCLUDE` constraints as for other index-backed constraints.

Authoritative source: PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, especially `src/backend/commands/tablecmds.c`, `src/backend/commands/indexcmds.c`, and `src/backend/catalog/pg_constraint.c`.

## Constraint boundary

`pg_index.indisexclusion` alone cannot be promoted to “ordinary EXCLUDE constraint” identity. PostgreSQL 18 temporal PRIMARY KEY / UNIQUE `WITHOUT OVERLAPS` indexes also use exclusion behavior. Existing key-constraint observations remain canonical for those rows. The new successor therefore requires explicit exclusion-constraint observations with a resolved backing-index coordinate and uses `indisexclusion` only for completeness against non-key exclusion backing indexes.

Constraint names are not inferred from index names. Each exclusion observation carries its exact constraint coordinate separately from its exact backing-index coordinate, corresponding to source `pg_constraint.conindid`. Parentage is resolved by the attached backing-index topology and then matched to the exact parent constraint observation.

## Alternatives

Expanding the existing `TableConstraintObservation` enum was rejected for this repair because that representation already participates in issued v3 identity. Rewriting it would change a broad predecessor digest domain and force unrelated consumers to adopt a new base representation. Inferring exclusion constraints from `pg_get_indexdef`, rendered DDL, or `indisexclusion` alone was also rejected because it would collapse an independent catalog object into derived text or index state.

The selected design is a narrow domain-separated successor owned by `conceptweave-relation-partition`. It consumes the exact immutable `IndexPartitionSnapshot`, requires one-to-one `conindid` coverage for ordinary exclusion indexes, validates exact `conparentid` through the backing-index parent edge, and validates root `(true,0)` versus attached `(false,1)` inheritance state.

## Traceability

Finding review: `5217805982` on ConceptWeave PR #46 exact predecessor `63a1feeb238f4bf174fed31f34dbd89add5587c2`.

Source/compile RED contract: `e38db5f009ce154776bb8f3f2aa6b541b37f0ed4`, subsequently tightened by `7d2f14d7d5645f244c4b63335bb3638cfee32980` so constraint names and `conindid` backing-index identity cannot collapse.

Production successor: `b87b70130047eb890f375e57012c5be8afd95fdb`, tightened by `c53ef790d4a236cb44d8759621f906078b0fe362` to retain explicit `conindid` and resolve parent constraints through the index-partition edge. Public export: `58a9befb66910509861f87275233305975994de0`.

Focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_partition_contract.rs`.

Production module: `crates/conceptweave-relation-partition/src/index_exclusion_constraint.rs`.

## Acceptance

This lineage is not executable GREEN until one unchanged exact head passes the repository-pinned Rust 1.98 toolchain, strict workspace/all-target Clippy, the focused exclusion-constraint contract plus all retained Source Observation contracts, workspace/doc tests, release build, rustdoc/coverage, and applicable hosted Product/security gates.

The live PostgreSQL 18 differential must create a real partitioned `EXCLUDE` constraint, read `pg_constraint.contype`, `conindid`, `conparentid`, `conislocal`, and `coninhcount` together with the index `pg_inherits` edge, and show that ConceptWeave admits the exact source tuple while rejecting missing/wrong `conindid`, missing/wrong parentage, or local/zero state on an attached exclusion constraint. A temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` control must demonstrate that `indisexclusion=true` does not double-create an ordinary exclusion-constraint observation.
