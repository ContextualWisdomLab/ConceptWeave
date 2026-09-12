# Source Observation temporal period-value type traceability

## Problem

PostgreSQL 18 applies `WITHOUT OVERLAPS` and `PERIOD` semantics to the final constraint column. That value must resolve to a range or multirange type. ConceptWeave already preserves explicit `pg_constraint.conperiod` and cross-checks temporal key/index shape, but those facts do not prove the type semantics of the final temporal column.

Exact predecessor review `5184917000` records the P1 finding on `5051f8d2262851ebfd9119518ceb633d92bbef40`. Behavioral RED `ef86be21c06477d210a9725874f10a969f4f37a9` added `constraint_period_type_contract.rs`, where an otherwise coherent `WITHOUT OVERLAPS` primary-key fixture uses scalar `pg_catalog.text` as the final column. PostgreSQL cannot create that state, so governed observation must reject it rather than treating GiST/exclusion evidence as a type proof.

Review `5185079749` added a compatibility constraint that materially changes the repair: PostgreSQL 18.4 fixed `WITHOUT OVERLAPS` so a domain over a range or multirange is valid. Commit `558fb3c880b03dcac6456d168b460c4619dbac86` therefore preserves a domain-over-`tstzrange` positive control beside the scalar RED. A direct-only `pg_type.typtype IN ('r', 'm')` check on the column type would be incorrect for supported 18.4+ servers.

Review `5185263151` then identified a second representation boundary on exact `1506e26cd5e608e47877d4093b4ebaee1ee3a916`: the private v3 compatibility representation resolves non-`pg_catalog` column types only when they are already represented as a domain, enum, or relation-backed row type. A direct user-defined range or multirange therefore could not reach temporal admission at all. The repair must preserve the exact public source coordinate and use any legacy compatibility projection only inside the private validator.

## Authoritative semantics

PostgreSQL 18 `CREATE TABLE` specifies all of the following:

- `WITHOUT OVERLAPS` applies to the last `UNIQUE` or `PRIMARY KEY` column.
- The `WITHOUT OVERLAPS` value must resolve to range or multirange semantics.
- `PERIOD` applies to the last foreign-key column, after at least one ordinary equality-key column.
- The `PERIOD` value must resolve to range or multirange semantics.
- The referenced key must use `WITHOUT OVERLAPS` on its corresponding final column.

PostgreSQL 18.4 explicitly corrected `WITHOUT OVERLAPS` to allow domains over range/multirange types. Current 18.x catalog semantics expose the direct type class in `pg_type.typtype` (`d` domain, `r` range, `m` multirange) and the domain base coordinate through `pg_type.typbasetype`; `pg_range` independently records the exact range type and associated multirange type through `rngtypid` and `rngmultitypid`. These OIDs are capture-time join coordinates only. Governed identity remains the exact qualified type coordinate resolved through `pg_namespace` and `pg_type`.

`pg_constraint.conperiod` records whether temporal constraint syntax was declared. It is authoritative for the presence of temporal constraint semantics, but it does not encode the period column's type semantics. `pg_constraint.conexclop` and the supporting GiST index likewise do not substitute for that type fact.

## Implemented representation

The Source Observation successor now has a first-class `TypeKindObservation` family and `PostgresTypeKind` vocabulary. The family preserves exact qualified `pg_type` coordinates and direct catalog type kind. Domain observations carry the exact qualified `typbasetype`; range and multirange observations carry reciprocal exact coordinates from the same `pg_range` relationship. Raw catalog OIDs are not part of governed identity.

Production lineage:

- `375242bb02cc165a410e72e318f9abec712764bf` introduces `type_kind.rs` and the source-authoritative value objects;
- `7bfe5eb46a38e3a4bcfe91aac2688b956526b8b7` integrates the family into `PostgresSchemaSnapshotV3`, adds the domain-separated `conceptweave.postgres_schema_snapshot.v3.type_kinds.v1` digest, preserves unobserved versus explicitly observed state, validates reciprocal range/multirange evidence and domain cycles, and requires every `conperiod=true` final local column to resolve through explicit type-kind/domain-base evidence to range or multirange;
- the same integration adds `new_with_type_kinds(...)` so direct user-defined range/multirange bindings remain exact in the public aggregate while the legacy private validator receives only a bounded compatibility projection; the outer successor digest binds the original qualified source coordinate, not the projection;
- `15e8b1949834ace8eb3cd99834d3c7157316579d` expands `constraint_period_type_contract.rs` with missing-evidence and scalar rejection, domain-over-range acceptance, domain-over-scalar rejection, direct user-defined range and multirange positive controls, and reciprocal `pg_range` failure;
- retained temporal contracts are ordinary-forward repaired at `23cde04c62dcb8c3b967913f3e3147275a51466e`, `da916f06fd39679800cc66701dbc5c3cba61731d`, and `67318b4edf1555297db1e8cecca2371e9eb5e92c` so their positive temporal fixtures provide explicit type-kind evidence rather than relying on type-name inference.

Temporal admission deliberately does not make the type-kind family authoritative for whether temporal syntax was declared. `conperiod` remains that authority. Type-kind evidence only proves that a declared temporal constraint has a PostgreSQL-valid final value type.

The family is attached before timing/period digest layers. Adding it after those families fails closed, avoiding two source-content identities created solely by optional-family application order. Direct user-defined range/multirange columns use the explicit type-kind-aware constructor because the frozen private v3 validator cannot represent those coordinates before projection.

## Rejected alternatives

Hard-coding PostgreSQL built-in names would reject valid user-defined range types. Checking only the direct final-column `typtype` for `r`/`m` would reject the domain-over-range behavior fixed in PostgreSQL 18.4. Treating any non-`pg_catalog` type as potentially temporal would fail open for domains, enums, composites, and ordinary user-defined types. Inferring from GiST/exclusion flags confuses enforcement mechanics with type identity. Parsing `format_type()` or reconstructed DDL as identity would make rendered text authoritative over catalog coordinates.

The compatibility projection used by `new_with_type_kinds(...)` is also not semantic evidence. It exists only to pass exact source facts through the frozen private validator; the public aggregate and outer digest retain the original qualified type binding and exact catalog type-kind evidence.

## Acceptance

The source repair is implemented but is not yet native/Product GREEN evidence. Exact-head Rust 1.98 acceptance must still prove the full retained workspace plus the following temporal type contract:

- missing type-kind evidence fails closed;
- scalar final columns fail closed;
- built-in range/multirange coordinates are admitted only from explicit catalog evidence;
- direct user-defined range and multirange coordinates remain exact and are admitted;
- domain-over-range/multirange remains admissible, including PostgreSQL 18.4+ behavior;
- domain-over-scalar and cyclic/missing domain evidence fail closed;
- range/multirange evidence is reciprocal;
- type-kind input order is digest-stable and unobserved versus observed evidence remains distinct;
- historical non-temporal and frozen-v2 identity remains unchanged.

Until one unchanged exact head produces repository-pinned Rust, Product, security, dependency, and review evidence, this lineage remains source-repaired / acceptance-pending and must stay Draft.

## References

PostgreSQL Global Development Group. (2026). *CREATE TABLE — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *pg_type — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-type.html

PostgreSQL Global Development Group. (2026). *pg_range — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-range.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes*. https://www.postgresql.org/docs/18/release-18-4.html

PostgreSQL Global Development Group. (2026). *pg_constraint — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
