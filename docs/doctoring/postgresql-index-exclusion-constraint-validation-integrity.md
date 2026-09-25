# PostgreSQL 18 EXCLUDE constraint validation-state integrity

## Decision

ConceptWeave must retain `pg_constraint.convalidated` as explicit governed source evidence for every ordinary index-backed `EXCLUDE` constraint represented by the Source Observation successor chain. A `contype = 'x'` observation with `convalidated = false` fails closed for the PostgreSQL 18 creation path modeled here.

This is a domain-separated successor. It does not change any issued v3, relation-partition, index-partition, EXCLUDE identity, EXCLUDE timing, or EXCLUDE enforcement digest.

## Problem

The predecessor at `39a5e57b2da10c1326b57822829f3fa80abc01f9` preserves ordinary EXCLUDE constraint identity, exact `conindid`, `conparentid`, `conislocal`, `coninhcount`, `condeferrable`, `condeferred`, and `conenforced`. It does not preserve `convalidated`.

PostgreSQL exposes `convalidated` independently in `pg_constraint`. More importantly, the pinned PostgreSQL 18 source `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` calls `CreateConstraintEntry()` from `index_constraint_create()` with `true` for both `Is Enforced` and `isValidated`. That function handles index-backed constraints and explicitly distinguishes `CONSTRAINT_EXCLUSION` from primary/unique constraints where expression handling differs.

Without an explicit validation observation, contradictory source evidence such as an ordinary EXCLUDE row with `convalidated = false` can receive the same governed identity as the supported PostgreSQL 18 state.

## Constraints

- Preserve the existing immutable digest chain; do not revise an issued predecessor domain.
- Treat raw catalog state as source evidence. Do not silently normalize `false` to the PostgreSQL creation default.
- Require exactly one validation observation for every exact EXCLUDE constraint in the enforcement predecessor.
- Keep temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` in the existing key-constraint family rather than double-counting `pg_index.indisexclusion` as ordinary `contype = 'x'` evidence.
- Do not claim that this source contract proves a live PostgreSQL extractor, transport, or hosted acceptance. Those remain later evidence gates.

## Alternatives considered

### Ignore `convalidated`

Rejected. `pg_constraint` exposes the field independently, and the pinned creation path supplies an explicit validated state. Dropping it makes incomplete or contradictory catalog evidence indistinguishable after governance.

### Add `convalidated` to the existing EXCLUDE constraint digest

Rejected. The predecessor digest is already issued in this Draft lineage. Mutating its framing would invalidate reproducibility and violate the immutable-successor rule.

### Normalize missing or false validation state to `true`

Rejected. Normalization would convert extraction omission or contradictory source data into apparently valid authoritative evidence.

### Domain-separated validation successor

Selected. The successor binds the exact enforcement-predecessor digest, deterministic EXCLUDE constraint coordinate, and raw `convalidated` bit. It requires complete one-to-one coverage and rejects `false`.

## Traceability

- Finding review: `5218582084` on PR #46 at predecessor `39a5e57b2da10c1326b57822829f3fa80abc01f9`.
- Source/compile RED contract: `064e3085a56aebfb35bea8cefd143ea4552191d1`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_validation_contract.rs`.
- Production successor: `100f66078fc4568480f7f6d7bdd6060b5d25bd96`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_validation.rs`.
- Public composition: `5cc60c6d2ddc9f6c4cfcbee1aeea0da16d445850`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- PostgreSQL source authority: `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/catalog/index.c`, `index_constraint_create()`.
- PostgreSQL catalog authority: PostgreSQL 18 `pg_constraint`, including `contype`, `conenforced`, `convalidated`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, and `connoinherit`.

The RED is an executable source/compile contract, not a claim that it was executed in this environment. Exact-head Rust 1.98 and hosted acceptance must be obtained after all source/docs movement stops.

## Risk and effect

The repair prevents unsupported or incomplete EXCLUDE validation state from being silently promoted into governed immutable evidence. It also makes later extractor completeness measurable: transport must explicitly read `convalidated`, not infer it from constraint type or DDL defaults.

This does not yet close the complete `pg_constraint` state surface. In particular, `connoinherit` remains a separately observable catalog field and must be evaluated against PostgreSQL 18 root/partition index-constraint semantics before it can be claimed complete.

## Live differential requirement

The PostgreSQL 18 differential must read ordinary EXCLUDE `contype = 'x'`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `condeferrable`, `condeferred`, `conenforced`, and `convalidated` in one bounded observation. It must show that supported ordinary EXCLUDE constraints are created validated and that ConceptWeave rejects missing, duplicate, or contradictory validation evidence. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains a control proving exclusion behavior is not double-counted into this family.

## References

PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: 52.13. pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: src/backend/catalog/index.c, REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1* [Source code]. GitHub. https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/catalog/index.c
