# PostgreSQL 18 NOT NULL inheritance-origin integrity

## Decision

ConceptWeave rejects a relation-scoped `pg_constraint.contype = 'n'` observation when `conislocal = false` and `coninhcount = 0`.

This is a source-integrity rule, not a normalization rule. The two catalog fields remain governed evidence, but the combination above cannot represent a PostgreSQL 18 NOT NULL constraint origin: it says the constraint is not locally defined while also reporting no direct inheritance ancestor.

## PostgreSQL 18 authority

`REL_18_STABLE` `AddRelationNewConstraints()` derives a new NOT NULL constraint's inheritance count as `is_local ? 0 : 1` before calling `StoreRelNotNull()`. Therefore a newly created inherited-only NOT NULL row begins with `conislocal = false` and `coninhcount = 1`, while a purely local row begins with `conislocal = true` and `coninhcount = 0`.

`AdjustNotNullInheritance()` preserves the same model for an existing NOT NULL constraint: a local addition sets `conislocal` true, while an inherited addition increments `coninhcount`. `ConstraintSetParentConstraint()` applies the declarative-partition case by setting `conislocal = false` and incrementing `coninhcount` from zero to one before storing `conparentid`; removing that parent reverses the count and restores `conislocal = true`.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: heap.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/heap.c
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

## Alternatives considered

Allowing every nonnegative `coninhcount` independently from `conislocal` was rejected because public callers could construct an origin-less tuple and receive the same governed treatment as catalog-observable evidence.

Requiring `conislocal == (coninhcount == 0)` was also rejected. PostgreSQL legitimately permits a constraint to be locally defined and inherited simultaneously, so `conislocal = true` with `coninhcount > 0` must remain representable.

The selected invariant is therefore deliberately one-way: only `!conislocal && coninhcount == 0` fails closed.

## Executable traceability

- Finding: PR #46 review `5195128595`, exact predecessor `5764aff19466b621cdb442646c0b8e48d84d6d46`.
- RED contract: `crates/conceptweave-observation/tests/not_null_constraint_inheritance_origin_contract.rs`, commit `2940852b202eb280685eef1042d8025b3cf9d2e2`.
- Production repair: `crates/conceptweave-observation/src/not_null_constraint.rs`, commit `8a7dfae20be17ec910924e56e88950166a626c8f`.

The regression also preserves three source-representable states: purely local `(conislocal=true, coninhcount=0)`, inherited-only `(false, >=1)`, and local-plus-inherited `(true, >=1)`.

## Adapter obligation

A PostgreSQL transport must capture `conislocal` and signed `int2` `coninhcount` from the same bounded catalog snapshot. It must not infer one from the other, substitute defaults, or coerce the invalid `(false, 0)` combination into a local row. Contradictory evidence fails before immutable snapshot construction.

## Acceptance status

The RED contract and causal source repair are source-level evidence. They are not an executed Rust GREEN until one unchanged exact PR head passes the repository-pinned Rust 1.98 native suite and applicable hosted acceptance gates.
