# PostgreSQL transform-converter set-return integrity

## Decision

ConceptWeave records raw same-generation `pg_proc.proretset` for every nonzero FROM-SQL/TO-SQL transform converter and admits only `false`.

This fact succeeds the converter routine-kind snapshot. It does not infer scalarity from the resolved return-type OID and it does not rewrite the converter definition, owner, ACL, configuration, security, planner, cost, or routine-kind digest domains.

## Why this is a separate source fact

PostgreSQL 18 stores `proretset` independently in `pg_proc`. The transform DDL implementation's `check_transform_function()` first verifies normal-function kind and then separately rejects set-returning functions. The checker subsequently validates argument count and the `internal` argument type. These are distinct catalog predicates, not consequences of a normalized converter coordinate.

ConceptWeave's base converter binding already records a scalar-looking return type for the FROM-SQL/TO-SQL direction. That normalized representation is insufficient evidence for the raw same-generation `proretset` bit: a mixed-generation join or extractor normalization defect could otherwise produce a converter-shaped successor from a row PostgreSQL itself would reject as set-returning.

## Contract

`IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot` succeeds the exact converter-kind snapshot and requires one observation for every predecessor `(constraint coordinate, key position, transform type, direction)` tuple. Each observation repeats the exact converter schema/function binding and raw `proretset`.

Admission fails closed when:

- `proretset=true`;
- a predecessor converter direction is missing or an extra coordinate is introduced;
- the same converter coordinate is duplicated;
- converter schema/function identity drifts from the predecessor;
- identifiers are blank or the key position is zero; or
- a provenance receipt is requested for an unknown coordinate.

The successor digest is domain-separated from converter kind and includes the raw set-return bit. Provenance continues to percent-encode transform schema and type-name components independently.

## Scope boundary

This decision does not infer or reconstruct raw `pg_proc.pronargs`. PostgreSQL's transform checker independently requires `pronargs == 1`; that raw same-row fact is handled by the next successor. The already existing base converter contract continues to require an `internal` argument type.

## Traceability

- finding review: `5251492933`;
- RED contract: `e92a8657a20878621da883a4326237f1d899bc89`;
- production implementation: `c6d11432cbac681a937449ea41e2c49e26f24409`;
- public composition: `40fc0e46b8a9e8c1018245540fc788f47a9132c0`;
- production source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_return_set.rs`;
- executable contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_return_set_contract.rs`.

Exact-head Rust 1.98 execution, owned production coverage, and the bounded PostgreSQL 18 live differential remain acceptance gates. No predecessor execution evidence transfers after source movement.

## Primary references

PostgreSQL Global Development Group. (2026). *pg_proc*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *CREATE TRANSFORM*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *functioncmds.c: check_transform_function*. PostgreSQL source code. https://doxygen.postgresql.org/functioncmds_8c.html
