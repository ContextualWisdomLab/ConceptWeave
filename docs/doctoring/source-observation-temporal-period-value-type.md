# Source Observation temporal period-value type traceability

## Problem

PostgreSQL 18 applies `WITHOUT OVERLAPS` and `PERIOD` semantics to the final constraint column. That column must have a range or multirange type. ConceptWeave already preserves explicit `pg_constraint.conperiod` and cross-checks temporal key/index shape, but those facts do not prove the type category of the final temporal column.

Exact predecessor review `5184917000` records the P1 finding on `5051f8d2262851ebfd9119518ceb633d92bbef40`. Behavioral RED `ef86be21c06477d210a9725874f10a969f4f37a9` adds `constraint_period_type_contract.rs`, where an otherwise coherent `WITHOUT OVERLAPS` primary-key fixture uses scalar `pg_catalog.text` as the final column. PostgreSQL cannot create that state, so governed observation must reject it rather than treating GiST/exclusion evidence as a type proof.

## Authoritative semantics

PostgreSQL 18 `CREATE TABLE` specifies all of the following:

- `WITHOUT OVERLAPS` applies to the last `UNIQUE` or `PRIMARY KEY` column.
- The `WITHOUT OVERLAPS` column must have a range or multirange type.
- `PERIOD` applies to the last foreign-key column, after at least one ordinary equality-key column.
- The `PERIOD` column must have a range or multirange type.
- The referenced key must use `WITHOUT OVERLAPS` on its corresponding final column.

`pg_constraint.conperiod` records whether these temporal forms were declared. It is authoritative for the presence of temporal constraint semantics, but it does not encode the period column's PostgreSQL type category. `pg_constraint.conexclop` and the supporting GiST index likewise do not substitute for that type fact.

## Representation decision

Do not infer range/multirange identity from index access method, constraint name, display text, an underscore convention, or a hard-coded list of built-in range names. PostgreSQL supports user-defined range and multirange types, so a correct repair needs exact source-authoritative type-category evidence tied to the qualified type coordinate used by the final constrained column.

The preferred successor is a versioned observed type-kind family derived from PostgreSQL catalog facts (`pg_type` and, where required, `pg_range`). Temporal constraint admission can then require:

1. the final local constraint column resolves to an observed qualified type coordinate;
2. that coordinate is explicitly observed as range or multirange;
3. a same-snapshot referenced temporal key resolves its final referenced column through the same rule;
4. `conperiod` remains the only authority for whether the constraint itself has temporal semantics.

The family must distinguish unobserved type-kind evidence from explicitly observed category evidence and must participate in the governed successor digest. OIDs may join one captured catalog transaction but are not semantic identity.

## Rejected alternatives

Hard-coding PostgreSQL built-in names would reject valid user-defined range types. Treating any non-`pg_catalog` type as potentially temporal would fail open for domains, enums, composites, and ordinary user-defined types. Inferring from GiST/exclusion flags confuses enforcement mechanics with type identity. Parsing `format_type()` or reconstructed DDL as identity would make rendered text authoritative over catalog coordinates.

## Acceptance

The current RED is intentionally not native/Product GREEN evidence. Production repair is complete only when exact-head Rust 1.98 tests prove that scalar final columns fail closed, built-in and user-defined range/multirange coordinates are admitted when explicitly observed, input ordering is digest-stable, missing type-kind inventory fails closed for temporal constraints, and historical non-temporal/v2 identity remains unchanged.

## References

PostgreSQL Global Development Group. (2026). *CREATE TABLE — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *pg_constraint — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
