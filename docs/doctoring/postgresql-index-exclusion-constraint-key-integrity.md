# PostgreSQL ordinary EXCLUDE `conkey` integrity

## Decision

ConceptWeave must preserve `pg_constraint.conkey` as explicit governed evidence for every ordinary PostgreSQL 18 `contype = 'x'` exclusion constraint. The vector is not reconstructed and silently substituted for source metadata. A source adapter supplies the raw ordered `int2[]`; the relation-partition successor verifies that it agrees with the already-governed backing-index key layout and then binds the raw vector into a new domain-separated digest and provenance receipt.

This decision does not change any issued v3, relation-partition, index-partition, ordinary EXCLUDE identity, timing, enforcement, validation, no-inherit, or period digest domain.

## Source authority

PostgreSQL 18 documents `pg_constraint.conkey` as the constrained-column attribute numbers. For exclusion constraints, simple column references carry their `pg_attribute.attnum`; expression elements carry zero, and the catalog documentation states that the resulting array has the same contents as the supporting index's `pg_index.indkey` for those constraint key elements.

The pinned PostgreSQL source used by this lane is `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

`src/backend/catalog/index.c::index_constraint_create()` passes `indexInfo->ii_IndexAttrNumbers`, `indexInfo->ii_NumIndexKeyAttrs`, and `indexInfo->ii_NumIndexAttrs` to `CreateConstraintEntry()`. `src/backend/catalog/pg_constraint.c::CreateConstraintEntry()` constructs `conkey` from exactly the first `constraintNKeys` values. The total-key count is used separately for dependency registration, which is why INCLUDE payload attributes are not members of `conkey`.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). `src/backend/catalog/index.c`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`. https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/catalog/index.c
- PostgreSQL Global Development Group. (2026). `src/backend/catalog/pg_constraint.c`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`. https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/catalog/pg_constraint.c

## Finding

Review `5219346562` on exact predecessor `658b46604edf597cac700175fc47b91200eaeed4` found that `IndexExclusionConstraintSnapshot` retained the independent constraint coordinate, exact `conindid` backing-index coordinate, partition parentage, and inheritance state, while later successors retained timing, enforcement, validation, raw no-inherit, and period state. None retained the independently stored `conkey` array.

Without an explicit `conkey` observation, contradictory catalog evidence can collapse into the same governed identity: the backing index can say that the first exclusion element is relation attribute 1 while the constraint row carries another attribute number. Expression positions are also material because PostgreSQL records zero there rather than an inferred column coordinate.

## Repair

The source/compile contract is commit `edc700bf98bfb3fce9848ab9693624c18c1d97ce`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_key_contract.rs`.

The production successor is commit `5a4d55a2e652694f259a50632012c0e3e563f6b7`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_key.rs`, exported by `e856d8f148fb2826503a7928d3fa1da1d38bdea4`.

`IndexExclusionConstraintKeySnapshot`:

- rebound-validates the exact `IndexExclusionConstraintPeriodSnapshot` against the supplied ordinary EXCLUDE identity predecessor;
- requires exactly one raw `conkey` observation for every ordinary EXCLUDE coordinate;
- resolves the exact `conindid` backing index already owned by `IndexExclusionConstraintSnapshot`;
- maps each governed backing-index key element to PostgreSQL constraint-key form: simple relation column -> exact observed attribute number, expression -> `0`;
- excludes INCLUDE payload positions because PostgreSQL persists only the first `ii_NumIndexKeyAttrs` values into `conkey`;
- rejects contradictory vectors with `index_exclusion_constraint_key_state`;
- binds the predecessor digest, exact constraint coordinate, ordered signed `int2` vector, and `/key-attributes` provenance into a new digest domain.

The raw source vector remains the evidence being governed. The derived vector is only a consistency check against already-governed index/relation evidence.

## Validation boundary

No runtime GREEN is inferred from source commits. The execution environment for this lane currently exposes no Rust toolchain, and the Draft PR has no repository-owned exact-head pull-request workflow execution. Before acceptance, one unchanged head must pass repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the new contract and all retained relation-partition contracts, workspace/doc tests, release build, rustdoc/coverage, and applicable hosted Product/security/dependency/review gates.

The concrete PostgreSQL 18 live differential must extract `pg_constraint.conkey` and `pg_index.indkey` in the same bounded read. It must include at least: a simple-column exclusion element, an expression element proving the zero position, an INCLUDE payload proving it is absent from `conkey`, and a partitioned EXCLUDE parent/child pair whose mapped attribute numbers remain correct for each owning relation.
