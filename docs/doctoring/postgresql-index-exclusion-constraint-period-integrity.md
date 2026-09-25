# PostgreSQL ordinary EXCLUDE `conperiod` integrity

## Problem

The ordinary partitioned `EXCLUDE` successor already preserved the independent `pg_constraint` row, backing-index identity, partition parentage/inheritance, deferrability, default timing, enforcement, validation, and raw `connoinherit`, but it did not retain `pg_constraint.conperiod`.

PostgreSQL 18 defines `conperiod=true` only for `WITHOUT OVERLAPS` primary-key/unique constraints and `PERIOD` foreign keys. Ordinary `contype='x'` exclusion constraints therefore require an explicit `conperiod=false` observation. Dropping the field allows a contradictory `contype='x', conperiod=true` tuple to collapse into the same governed ordinary-EXCLUDE identity.

Temporal primary/unique/foreign-key semantics remain owned by `conceptweave-observation::ConstraintPeriodObservation`. This repair does not reinterpret or duplicate that family.

## Authority and decision

Authoritative catalog reference: PostgreSQL 18 `pg_constraint`, which defines `contype='x'` separately and describes `conperiod` as `WITHOUT OVERLAPS` for primary/unique constraints or `PERIOD` for foreign keys. PostgreSQL 18 release notes likewise scope temporal constraints to primary/unique keys and foreign keys.

Selected design:

- retain an explicit ordinary-EXCLUDE `conperiod` observation for every exact `IndexExclusionConstraintCoordinate`;
- admit only `false` for this `contype='x'` family and fail closed on `true`;
- require complete, duplicate-free one-to-one observation over the existing ordinary-EXCLUDE identity predecessor;
- bind the predecessor digest, exact constraint coordinate, and raw bit in a new domain-separated digest;
- issue exact `/period` provenance without changing any predecessor digest;
- leave temporal p/u/f `conperiod` truth with `ConstraintPeriodObservation`.

Rejected alternatives:

- omitting the bit because it is normally false: loses source integrity and admits impossible mixed catalog tuples;
- deriving temporal semantics from `pg_index.indisexclusion`: PostgreSQL 18 temporal key behavior can use exclusion-style enforcement while remaining `contype='p'`/`'u'`, so index shape is not constraint-type authority;
- moving p/u/f temporal semantics into the relation-partition crate: violates the existing owner boundary and would duplicate canonical observation truth.

## Traceability

- Review finding: `5218939490` on #46 predecessor `41a90acf08f3398e55f22298940745775ec9efc8`.
- Source/compile RED contract: `47b543b5618ceb30d1df285f7880fcbb1d313118`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_period_contract.rs`.
- Minimum production successor: `7840356c3a2ffd60b690e6b2a7b1b14535e6f985`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_period.rs`.
- Public composition: `13846fa2ac40a144ba454fa6c56fd924045662bc`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

The executable contract requires explicit false evidence, rejects true, rejects incomplete inventory, and verifies exact provenance. This runtime has not executed the Rust contract; source/compile RED and source repair are not exact-head GREEN until the repository-pinned Rust and hosted gates run on one unchanged head.

## Live differential still required

A PostgreSQL 18 differential must query the ordinary EXCLUDE row and temporal controls in the same bounded capture. It must verify:

- ordinary `contype='x'` rows carry `conperiod=false`;
- `WITHOUT OVERLAPS` primary/unique controls carry their temporal state in the existing p/u family rather than being double-counted as ordinary EXCLUDE;
- `PERIOD` foreign-key controls remain in the existing foreign-key temporal family;
- the ordinary EXCLUDE extractor rejects missing, duplicate, or contradictory `conperiod` evidence.

The same differential should continue to read `conindid`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, and `convalidated` so the catalog tuple is validated as one bounded source observation.