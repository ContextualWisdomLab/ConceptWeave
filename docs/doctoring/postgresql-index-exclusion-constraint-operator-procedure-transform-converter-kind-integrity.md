# PostgreSQL transform-converter function-kind integrity

## Decision

ConceptWeave records raw same-generation `pg_proc.prokind` for every nonzero FROM-SQL/TO-SQL transform converter and admits only PostgreSQL normal-function kind `f`.

The fact is a successor over the already governed converter-cost snapshot. It does not replace the converter coordinate, signature, implementation digest, owner, ACL, configuration, security, planner-support, or cost facts, and it does not infer routine kind from any of them.

## Why this is a separate source fact

PostgreSQL stores routine kind in `pg_proc.prokind`; PostgreSQL 18 documents `f` as a normal function, `p` as a procedure, `a` as an aggregate, and `w` as a window function. The transform DDL implementation performs an explicit check that the selected converter has `prokind == PROKIND_FUNCTION`. The same function also separately checks volatility, set-returning state, argument count, and the required `internal` input type. Those independent checks are evidence that normalized function identity alone is not the authoritative source for converter routine kind.

ConceptWeave already governs raw `prokind='f'` for the ordinary exclusion operator implementation routine. The converter chain previously preserved its exact function coordinate and many `pg_proc` properties without binding this structural field. That allowed an extractor defect or mixed-generation join to present a converter-shaped coordinate without proving the same row is a normal function.

## Contract

`IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot` succeeds the exact converter-cost snapshot and requires one observation for every predecessor `(constraint coordinate, key position, transform type, direction)` tuple. Each observation repeats the exact converter schema/function binding and raw `prokind`.

Admission fails closed when:

- `prokind` is anything other than `f`;
- a predecessor direction is missing or an extra coordinate is introduced;
- the same converter coordinate is duplicated;
- converter schema/function identity drifts from the predecessor;
- identifiers are blank or the key position is zero; or
- a provenance receipt is requested for an unknown coordinate.

The successor digest is domain-separated from converter cost and includes the exact routine-kind byte. Provenance continues to percent-encode transform schema and type-name components separately, so quoted identifiers containing `.` do not collide.

## Scope boundary

This change does not fold `pg_proc.proretset` into routine kind. PostgreSQL's transform checker independently rejects set-returning converters, so `proretset=false` remains a separate structural fact for the next review. Likewise, this decision does not reinterpret mutable auxiliary fields such as volatility: it only binds the structural normal-function kind required by PostgreSQL's transform contract.

## Traceability

- finding review: `5251421078`;
- RED contract: `f60aa85fbf36ffb8ab6354afc4826d347ba8ea44`;
- production implementation: `b75b3f008c0baeeae1b5e5eb3fd421b14e0d1205`;
- public composition: `75028d1c8c18a490f793f8fcde5061ec39c52158`;
- production source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_kind.rs`;
- executable contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_kind_contract.rs`.

Exact-head Rust 1.98 execution, owned production coverage, and bounded PostgreSQL 18 live differential remain acceptance gates. No predecessor execution evidence transfers after source movement.

## Primary references

PostgreSQL Global Development Group. (2026). *pg_proc*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *CREATE TRANSFORM*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *functioncmds.c: check_transform_function*. PostgreSQL source code. https://doxygen.postgresql.org/functioncmds_8c.html
