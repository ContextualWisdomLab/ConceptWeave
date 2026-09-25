# PostgreSQL transform-converter strictness integrity

## Decision

ConceptWeave records `pg_proc.proisstrict` as an independent same-row fact for every nonzero `pg_transform` converter direction. The fact is layered after converter leakproof evidence rather than folded into converter identity or definition evidence. Both strict and non-strict states are representable; a change in the raw Boolean changes the successor digest.

This is intentionally observational. PostgreSQL `CREATE FUNCTION` defines `STRICT` / `RETURNS NULL ON NULL INPUT` as function execution semantics: PostgreSQL does not call the function when an input argument is null and assumes a null result. The same command exposes volatility, leakproof, security context, parallel safety, cost, support, and configuration as separate function attributes. `CREATE OR REPLACE FUNCTION` can replace function properties without changing the function's input identity. Therefore converter coordinate, implementation digest, owner, ACL, configuration, security-definer state, or leakproof state cannot be used as a proxy for strictness.

PostgreSQL's `CREATE TRANSFORM` contract constrains converter signatures: FROM SQL accepts `internal` and returns `internal`; TO SQL accepts `internal` and returns the transformed SQL type. The PostgreSQL 18 example declares both converter functions `STRICT IMMUTABLE`, but `STRICT` is a function attribute rather than part of the `(trftype, trflang)` transform-row identity. ConceptWeave consequently captures the exact converter function's raw `proisstrict` value instead of inferring it from the transform declaration, example, signature, or target function.

## Invariants

For one predecessor converter-leakproof snapshot, the strictness successor must cover exactly the same `(constraint coordinate, key position, transform type, direction)` inventory. Missing, extra, or duplicate coordinates fail closed. The converter schema and function name must match the predecessor direction exactly. Strict and non-strict observations use distinct domain-separated digests. Provenance lookup is exact-coordinate and exact-position bound.

Canonical provenance locations percent-encode the transform schema and type-name components independently before joining them with `.`. This preserves the collision repair established for quoted PostgreSQL identifiers: `("payload.domain", "json")` and `("payload", "domain.json")` cannot collapse to the same location.

## Traceability

| Concern | Owner evidence |
| --- | --- |
| Raw converter `proisstrict` observation and digest | `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_strictness.rs` |
| Public composition | `crates/conceptweave-relation-partition/src/index_partition.rs` |
| Completeness, binding drift, duplicate, receipt, digest and quoted-identifier contracts | `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_strictness_contract.rs` |
| Exact predecessor converter inventory | `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot` |
| Deferred independent converter attributes | volatility, parallel safety, planner support and cost remain separate follow-up facts |

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
