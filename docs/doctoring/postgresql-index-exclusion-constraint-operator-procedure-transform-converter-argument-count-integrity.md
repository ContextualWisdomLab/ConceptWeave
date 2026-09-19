# PostgreSQL transform-converter argument-count integrity

## Decision

ConceptWeave records raw same-generation `pg_proc.pronargs` for every nonzero FROM-SQL/TO-SQL transform converter and admits only `1`.

This fact succeeds the converter set-return snapshot. It does not infer the catalog count from the normalized Rust value object that already carries one `internal` argument type, and it does not rewrite predecessor digest domains.

## Why this is a separate source fact

PostgreSQL 18 stores `pronargs` independently in `pg_proc`. In `check_transform_function()`, PostgreSQL checks `pronargs == 1` before reading `proargtypes.values[0]` and requiring `INTERNALOID`. A normalized one-argument signature can therefore describe the desired call shape without proving that the exact same-generation catalog row actually carried the required raw count.

Keeping raw argument count distinct protects the observation boundary against mixed-generation joins and extractor normalization defects. It also preserves the distinction between two source facts PostgreSQL checks separately: `pronargs == 1` and `proargtypes[0] == internal`.

## Contract

`IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot` succeeds the exact converter set-return snapshot and requires one observation for every predecessor `(constraint coordinate, key position, transform type, direction)` tuple. Each observation repeats the exact converter schema/function binding and raw `pronargs` value.

Admission fails closed when:

- `pronargs` is anything other than `1`;
- a predecessor converter direction is missing or an extra coordinate is introduced;
- the same converter coordinate is duplicated;
- converter schema/function identity drifts from the predecessor;
- identifiers are blank or the key position is zero; or
- a provenance receipt is requested for an unknown coordinate.

The successor digest is domain-separated from converter set-return state and includes the exact signed 16-bit argument-count bytes. Provenance continues to percent-encode transform schema and type-name components independently.

## Structural checker coverage

For the explicit structural predicates in PostgreSQL 18 `check_transform_function()`:

- normal routine kind is bound by the converter-kind successor (`prokind='f'`);
- set-return state is bound by the converter-return-set successor (`proretset=false`);
- raw argument count is bound by this successor (`pronargs=1`);
- the base converter contract requires the resolved single argument type to be `pg_catalog.internal`.

Volatility remains an independently observed mutable catalog fact in its earlier successor. This structural sequence does not assert that every possible future PostgreSQL/catalog semantic gap is exhausted; subsequent work must come from fresh source/catalog review.

## Traceability

- finding review: `5251508944`;
- RED contract: `4f81f92f54edb622ba933617ef6a598d7a4a1d16`;
- production implementation: `65dabb7df539c2a80455f8b4aec5c024117b8a58`;
- first public-composition attempt: `cb43e7fc8ba28516dc96a6e3d61a8df9a71c9a24`;
- public-composition repair: `c7419477a8538551a573482f78013ad226923899`.

The first composition attempt contained one duplicate argument-count re-export outside the intended module stanza. It was immediately repaired ordinary-forward. Net comparison from return-set public head `40fc0e46b8a9e8c1018245540fc788f47a9132c0` to `c7419477...` is ahead-only and changes the new production module, its contract, and four added lines in `index_partition.rs`; no unrelated source delta survives.

Production source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_argument_count.rs`.

Executable contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_argument_count_contract.rs`.

Exact-head Rust 1.98 execution, owned production coverage, and the bounded PostgreSQL 18 live differential remain acceptance gates. No predecessor execution evidence transfers after source movement.

## Primary references

PostgreSQL Global Development Group. (2026). *pg_proc*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *CREATE TRANSFORM*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *functioncmds.c: check_transform_function*. PostgreSQL source code. https://doxygen.postgresql.org/functioncmds_8c.html
