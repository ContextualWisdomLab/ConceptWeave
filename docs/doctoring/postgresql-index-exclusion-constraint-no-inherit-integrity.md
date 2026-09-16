# PostgreSQL 18 EXCLUDE constraint no-inherit integrity

## Decision

ConceptWeave must retain `pg_constraint.connoinherit` as explicit governed source evidence for each ordinary index-backed `EXCLUDE` constraint, but it must not derive that bit from the constraint's current parentage.

PostgreSQL 18 has more than one source-reachable lifecycle path for a partition-child constraint. A child cloned while a parent constraint is known is created with `connoinherit = false`; a compatible standalone child constraint can be created first with `connoinherit = true` and later attached. `ConstraintSetParentConstraint()` changes `conparentid`, `conislocal`, and `coninhcount` but does not rewrite `connoinherit`. Detach likewise changes parentage/locality without changing that bit.

The correct contract is therefore raw-state preservation plus complete inventory, not parentage-derived normalization. This remains a domain-separated sibling successor over the exact EXCLUDE identity snapshot and does not rewrite issued EXCLUDE identity, timing, enforcement, or validation digest domains.

## Problem

The EXCLUDE identity predecessor already retains exact `conindid`, `conparentid`, `conislocal`, and `coninhcount`, but drops `connoinherit`. PostgreSQL exposes `connoinherit` independently in `pg_constraint`, so omitting it loses source history/state that PostgreSQL itself retains.

The pinned PostgreSQL 18 source `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` establishes two relevant paths:

- `index_constraint_create()` initializes a constraint created with a valid `parentConstraintId` as `islocal=false`, `inhcount=1`, `noinherit=false`; without a parent it initializes `islocal=true`, `inhcount=0`, `noinherit=true`.
- `ATExecAttachPartition()` can reuse an existing valid child index/constraint and calls `ConstraintSetParentConstraint()`.
- `ConstraintSetParentConstraint()` updates `conislocal`, `coninhcount`, and `conparentid`, but does not update `connoinherit` in either the attach or detach direction.

Consequently, current parentage alone does not determine `connoinherit`: an attached preexisting child can validly retain `true`, while a cloned child can have `false`.

## Constraints

- Preserve every issued predecessor digest domain.
- Keep the raw catalog bit explicit and hash it into governed evidence.
- Require exactly one `connoinherit` observation per predecessor ordinary EXCLUDE constraint; reject duplicate or incomplete inventory.
- Do not normalize or reject a raw bit merely because it disagrees with current parentage.
- Keep temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` in its existing key-constraint family.
- Do not claim live PostgreSQL extraction or exact-head acceptance from source-only changes.

## Alternatives considered

### Derive `connoinherit` from current parentage

Rejected after source review. The initial implementation attempted this and was repaired ordinary-forward. `ConstraintSetParentConstraint()` proves that parentage can change independently of `connoinherit`, so a parentage-derived invariant rejects a valid attach path.

### Ignore the field

Rejected. Different source-reachable states would collapse onto the same immutable governed identity even though PostgreSQL preserves the raw bit.

### Add the bit to `IndexExclusionConstraintSnapshot`

Rejected. That would rewrite an issued predecessor digest and violate immutable-successor reproducibility.

### Domain-separated raw-state successor

Selected. `IndexExclusionConstraintNoInheritSnapshot` binds the exact EXCLUDE predecessor digest, exact constraint coordinate, and raw `connoinherit` bit. It requires complete one-to-one inventory and issues exact provenance, while deliberately avoiding lifecycle inference.

## Traceability

- Original field-omission finding: review `5218606154` on PR #46 at predecessor `10c6c8c4827c585a744576700fa04478e20b14aa`.
- Initial source/compile contract: `3552cd383ba54b8f6aae8a8796b492a86a5d4738`.
- Initial successor: `a605dd29afc48f7b112d6aa8d73bc45e901bb86c`.
- Public composition: `cd25d0aaf633ae84caeb7144477027da657ab346`.
- Repair finding for the over-strong parentage invariant: review `5218646331` at `25508f53ed4ca8f4bb17b637dc82801ffcdbd7fc`.
- Corrected source/compile contract: `e8aa445679a56ba950e193fb4f9825614522d1ab`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_no_inherit_contract.rs`.
- Corrected production successor: `ed7362c110992e0d9914e971a2e63d2bf86ea589`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_no_inherit.rs`.
- PostgreSQL source authority: `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/catalog/index.c` (`index_constraint_create()`), `src/backend/commands/tablecmds.c` (`ATExecAttachPartition()`), and `src/backend/catalog/pg_constraint.c` (`ConstraintSetParentConstraint()`).
- PostgreSQL catalog authority: PostgreSQL 18 `pg_constraint`.

The tests are executable source/compile contracts, not a claim that they were executed in this environment. Exact-head Rust 1.98 and hosted acceptance remain required after source/docs movement stops.

## Risk and effect

The corrected repair preserves source distinctions without manufacturing a stronger invariant than PostgreSQL enforces. In particular, it prevents an attached preexisting constraint with `connoinherit=true` from being falsely rejected while still ensuring missing or duplicate observations fail closed and `true` versus `false` produce different governed digests.

## Live differential requirement

The PostgreSQL 18 differential must capture ordinary EXCLUDE `conparentid`, `conislocal`, `coninhcount`, and `connoinherit` together across at least two lifecycle paths:

- a child constraint cloned/created with its parent, demonstrating the creation-path state where `connoinherit=false`;
- a compatible standalone child constraint attached later, demonstrating that parentage/locality become child state while the preexisting `connoinherit=true` can remain unchanged.

A detach control should confirm that clearing parentage/locality does not silently normalize `connoinherit`. ConceptWeave must preserve the observed raw bit, reject missing/duplicate evidence, and issue distinct digests for distinct raw states.

The same bounded differential continues to require `contype`, `conindid`, `condeferrable`, `condeferred`, `conenforced`, and `convalidated`, plus a temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` control.

## References

PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: 52.13. pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1* [Source code]. GitHub. https://github.com/postgres/postgres/tree/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1
