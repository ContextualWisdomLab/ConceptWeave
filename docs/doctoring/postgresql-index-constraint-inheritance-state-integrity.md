# PostgreSQL index-constraint inheritance-state integrity

## Decision

ConceptWeave Source Observation must preserve `pg_constraint.conislocal` and `pg_constraint.coninhcount` for every observed primary-key or unique constraint after exact `conparentid` parentage has been established. The evidence is carried in a domain-separated successor above `IndexConstraintParentageSnapshot`; no frozen v3, relation-partition, index-partition, or parentage digest is rewritten.

## Problem

The parentage successor at predecessor `ccbe5b8b3c9d2cfa511652e6d74291ffe2fc86f9` distinguished an exact partition-constraint parent from `conparentid = 0`, but it did not retain the two catalog fields PostgreSQL mutates in the same parent-link operation. Consequently, a governed tuple could contain the correct resolved parent constraint while also claiming `conislocal = true` and `coninhcount = 0`. PostgreSQL 18 does not produce that stable attached state.

This is a source-integrity problem rather than a presentation concern. `conparentid`, `conislocal`, and `coninhcount` jointly describe whether a key constraint is local or inherited through the partition hierarchy. Collapsing those facts makes two materially different catalog states hash to the same ConceptWeave evidence.

## PostgreSQL 18 authority

Authority is PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` as observed on 2026-09-16.

`src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()` first attaches the child index with `IndexSetParentIndex()`. When the parent index is constraint-backed, it then calls `ConstraintSetParentConstraint(child_constraint_oid, parent_constraint_oid, child_table_oid)` before validating the partitioned index.

`src/backend/catalog/pg_constraint.c::ConstraintSetParentConstraint()` performs the catalog transition as one invariant:

- attaching asserts the child's prior `coninhcount` is zero;
- sets `conislocal = false`;
- increments `coninhcount` to one;
- sets `conparentid` to the parent constraint;
- creates partition dependencies preventing independent deletion;
- detaching decrements `coninhcount`, restores `conislocal = true`, clears `conparentid`, and asserts the resulting inheritance count is zero.

The existing parentage successor already owns exact parent identity. This repair adds only the two remaining scalar catalog facts needed to distinguish a stable parented key constraint from a local key constraint.

## Constraints

The repair must not project generic table-inheritance semantics into this bounded family. It applies only to primary-key and unique constraints already enumerated by `IndexConstraintParentageSnapshot`. It must preserve the valid PostgreSQL case in which a constraint-backed child index is attached below a non-constraint parent index: because there is no parent constraint, the child constraint remains local with `conparentid = 0`, `conislocal = true`, and `coninhcount = 0`.

Raw inheritance count is represented as signed `int16`, matching PostgreSQL catalog storage. Negative observations are rejected rather than normalized. No adapter-side inference from index shape is permitted.

## Alternatives considered

Extending `PrimaryKeyObservation` and `UniqueConstraintObservation` directly was rejected because those types participate in the frozen v3 representation; changing them would rewrite established identity rather than adding evidence ordinary-forward.

Mutating `IndexConstraintParentageObservation` and its existing digest domain was rejected for the same reason. Parentage evidence already has an issued successor identity in this Draft lineage and should remain stable for traceability.

Inferring local/inherited state from `conparentid` without observing the raw fields was rejected. PostgreSQL stores all three independently, and Source Observation must detect catalog inconsistency rather than normalize it away.

The selected design is `IndexConstraintInheritanceSnapshot`, complete over the exact parentage predecessor. A parented key constraint requires `(conislocal=false, coninhcount=1)`; an unparented key constraint requires `(true, 0)`. The successor binds the predecessor digest and the raw two-field observations under its own digest domain.

## Traceability

- Finding review: `5217650665` on #46 predecessor `ccbe5b8b3c9d2cfa511652e6d74291ffe2fc86f9`.
- Source/compile RED contract: `9f3125a28d2bd278a10c85a61557e5e00372f403`, `crates/conceptweave-relation-partition/tests/index_constraint_inheritance_state_contract.rs`.
- Production successor: `023adf1fc665c1aa1ec41e14bb126f510bd28ac4`, `crates/conceptweave-relation-partition/src/index_constraint_inheritance.rs`.
- Public export: `f3c46120f656afaac2358d5b6b2aea236b3ad5fb`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Receipt-coordinate correction: `dff165bb9bae049e28be6e649e72430e9f13fe14`; inheritance receipts and unknown-location diagnostics now use the distinct `/inheritance-state` evidence path rather than reusing the predecessor `/parentage` path.
- PostgreSQL authority: `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/catalog/pg_constraint.c::ConstraintSetParentConstraint()` and `src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()`.

## Acceptance and remaining risk

No executed Rust RED/GREEN is asserted from this editing environment because Rust 1.98 tooling is unavailable here. One unchanged exact #46 head still has to pass repository-pinned `fmt`, strict workspace/all-target Clippy, the new inheritance-state contract, retained Source Observation/relation-partition contracts, workspace/doc tests, release build, rustdoc/test/edge-case coverage, and hosted Product/security/dependency/review gates.

The PostgreSQL live differential must inspect all three fields after attachment and detach/standalone controls: exact parented key constraint identity, `conislocal=false`, `coninhcount=1`; and for the valid non-constraint-parent control, `conparentid=0`, `conislocal=true`, `coninhcount=0`. Any extractor that reports only parent OID identity remains incomplete.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `pg_constraint.c`* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). PostgreSQL Global Development Group.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `tablecmds.c`* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). PostgreSQL Global Development Group.
