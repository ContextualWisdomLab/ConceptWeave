# Source Observation temporal period-value type traceability

## Problem

PostgreSQL 18 applies `WITHOUT OVERLAPS` and `PERIOD` semantics to the final constraint column. That value must resolve to a range or multirange type. ConceptWeave already preserves explicit `pg_constraint.conperiod` and cross-checks temporal key/index shape, but those facts do not prove the type semantics of the final temporal column.

Exact predecessor review `5184917000` records the P1 finding on `5051f8d2262851ebfd9119518ceb633d92bbef40`. Behavioral RED `ef86be21c06477d210a9725874f10a969f4f37a9` adds `constraint_period_type_contract.rs`, where an otherwise coherent `WITHOUT OVERLAPS` primary-key fixture uses scalar `pg_catalog.text` as the final column. PostgreSQL cannot create that state, so governed observation must reject it rather than treating GiST/exclusion evidence as a type proof.

A second exact-head review, `5185079749`, records a compatibility constraint that materially changes the repair: PostgreSQL 18.4 fixed `WITHOUT OVERLAPS` so a domain over a range or multirange is valid. Commit `558fb3c880b03dcac6456d168b460c4619dbac86` therefore preserves a domain-over-`tstzrange` positive control beside the scalar RED. A direct-only `pg_type.typtype IN ('r', 'm')` check on the column type would be incorrect for supported 18.4+ servers.

## Authoritative semantics

PostgreSQL 18 `CREATE TABLE` specifies all of the following:

- `WITHOUT OVERLAPS` applies to the last `UNIQUE` or `PRIMARY KEY` column.
- The `WITHOUT OVERLAPS` value must resolve to range or multirange semantics.
- `PERIOD` applies to the last foreign-key column, after at least one ordinary equality-key column.
- The `PERIOD` value must resolve to range or multirange semantics.
- The referenced key must use `WITHOUT OVERLAPS` on its corresponding final column.

PostgreSQL 18.4 explicitly corrected `WITHOUT OVERLAPS` to allow domains over range/multirange types. Current 18.x catalog semantics expose the direct type class in `pg_type.typtype` (`d` domain, `r` range, `m` multirange) and the domain base coordinate through `pg_type.typbasetype`; `pg_range` independently records exact range and associated multirange OID relationships. These OIDs are capture-time join coordinates only. Governed identity remains the exact qualified type coordinate resolved through `pg_namespace` + `pg_type`.

`pg_constraint.conperiod` records whether temporal constraint syntax was declared. It is authoritative for the presence of temporal constraint semantics, but it does not encode the period column's type semantics. `pg_constraint.conexclop` and the supporting GiST index likewise do not substitute for that type fact.

## Representation decision

Do not infer range/multirange identity from index access method, constraint name, display text, an underscore convention, or a hard-coded list of built-in range names. PostgreSQL supports user-defined range and multirange types, and current PostgreSQL 18 also permits a domain layered over those types.

The successor therefore needs a versioned observed type-kind family derived from exact PostgreSQL catalog facts. At minimum it must preserve each observed qualified `pg_type` coordinate, its direct `typtype`, and—when the direct type is a domain—the exact qualified `typbasetype` coordinate needed to resolve the domain chain. `pg_range` can provide reciprocal range/multirange evidence without making raw OIDs part of semantic identity.

Temporal constraint admission can then require:

1. the final local constraint column resolves to an observed qualified type coordinate;
2. that coordinate resolves directly, or through an explicitly observed domain chain, to range or multirange semantics;
3. a same-snapshot referenced temporal key resolves its final referenced column through the same rule;
4. missing type-kind/domain-base evidence fails closed rather than guessing from names or PostgreSQL defaults;
5. `conperiod` remains the only authority for whether the constraint itself has temporal semantics.

The family must distinguish unobserved type-kind evidence from explicitly observed category evidence and participate in the governed successor digest. It must also retain direct user-defined range/multirange coordinates; a compatibility layer that can only classify built-ins would leave the v3 representation incomplete.

## Rejected alternatives

Hard-coding PostgreSQL built-in names would reject valid user-defined range types. Checking only the direct final-column `typtype` for `r`/`m` would reject the domain-over-range behavior fixed in PostgreSQL 18.4. Treating any non-`pg_catalog` type as potentially temporal would fail open for domains, enums, composites, and ordinary user-defined types. Inferring from GiST/exclusion flags confuses enforcement mechanics with type identity. Parsing `format_type()` or reconstructed DDL as identity would make rendered text authoritative over catalog coordinates.

## Acceptance

The current scalar RED is intentionally not native/Product GREEN evidence. Production repair is complete only when exact-head Rust 1.98 tests prove all of the following together: scalar final columns fail closed; built-in and user-defined range/multirange coordinates are admitted from explicit catalog evidence; a domain over range/multirange remains admissible; a domain over a scalar fails closed; missing or cyclic domain/type-kind evidence fails closed; input ordering is digest-stable; unobserved versus explicitly observed type-kind evidence remains distinct; and historical non-temporal/v2 identity remains unchanged.

## References

PostgreSQL Global Development Group. (2026). *CREATE TABLE — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *pg_type — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-type.html

PostgreSQL Global Development Group. (2026). *pg_range — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-range.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes*. https://www.postgresql.org/docs/18/release-18-4.html

PostgreSQL Global Development Group. (2026). *pg_constraint — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
