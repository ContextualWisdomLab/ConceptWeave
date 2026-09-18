# PostgreSQL transform-converter volatility integrity

## Decision

ConceptWeave records raw `pg_proc.provolatile` as an independent same-row fact for every nonzero `pg_transform` converter direction. The fact is layered after converter strictness evidence rather than folded into transform identity, converter definition, or any earlier auxiliary function property. PostgreSQL's three catalog states — `i` (immutable), `s` (stable), and `v` (volatile) — are all representable and produce distinct domain-separated successor digests.

This is observational rather than normative. PostgreSQL 18 exposes `IMMUTABLE`, `STABLE`, and `VOLATILE` as function attributes that inform planner and MVCC behavior; `VOLATILE` is the default when no category is specified. `CREATE FUNCTION` lists volatility separately from leakproofness, strictness, security context, parallel safety, cost, support, configuration, and transform use. `CREATE OR REPLACE FUNCTION` can replace function properties without changing the function's input identity. The converter coordinate and the already-governed definition/owner/ACL/configuration/security-definer/leakproof/strictness facts therefore cannot stand in for `provolatile`.

PostgreSQL `CREATE TRANSFORM` binds a type/language transform to optional FROM SQL and TO SQL conversion functions and specifies their signatures. Its example happens to declare the converter functions `STRICT IMMUTABLE`, but immutability is a property of those function rows, not part of the transform's `(type, language, direction)` coordinate. ConceptWeave consequently captures the exact converter function's raw `provolatile` discriminator and does not infer an immutable-only admission rule from the example.

## Invariants

For one exact converter-strictness predecessor, the volatility successor covers the same `(constraint coordinate, key position, transform type, direction)` inventory. Missing, extra, or duplicate coordinates fail closed. Converter schema and function name must match the predecessor direction exactly. Only `i`, `s`, or `v` is admitted as raw PostgreSQL catalog state; changes among those states change the successor digest.

Canonical provenance locations preserve the quoted-identifier collision repair: transform schema and type-name components are percent-encoded independently before the literal separator. `("payload.domain", "json")` and `("payload", "domain.json")` therefore remain distinct. Receipt lookup is exact-coordinate, exact-position, exact-transform-type, and exact-direction bound.

No optimization claim is derived from this observation. PostgreSQL documents volatility as a promise used by the optimizer and as a determinant of snapshot visibility. Capturing the raw state makes that behavior auditable; it does not prove that the function was correctly labelled by its author.

## Review and repair lineage

- Review `5249791627` identified the missing `provolatile` fact at #46 exact `7ffc577908dd0e87ed950a3bc8c5c2b952788e04`.
- RED `2bf4001dc1cee62df3d826fe585e246528c450cb` added the executable contract before public volatility types existed.
- Production `85265a29730698c6847a1089357a0a18e70c77d8` added the raw volatility observation/snapshot/receipt implementation.
- Public composition `d875c26cdd47c008015e8dfd0d769fc16ba8b5d0` exports the successor through `index_partition.rs`.

No Rust execution result is attached to this lineage yet. This environment has no repository-pinned Rust toolchain, and no PR-triggered workflow run was present for `d875c26...` at the first exact-head check. Source-shaped RED/repair evidence is therefore not acceptance GREEN.

## Traceability

| Concern | Owner evidence |
| --- | --- |
| Raw converter `provolatile` observation, validation and digest | `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_volatility.rs` |
| Public composition | `crates/conceptweave-relation-partition/src/index_partition.rs` |
| Completeness, binding drift, invalid state, duplicate, receipt, digest and quoted-identifier contracts | `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_volatility_contract.rs` |
| Exact predecessor converter inventory and binding | `IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot` |
| Deferred independent converter attributes | parallel safety, planner support and cost remain separate review-gated facts |

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Function optimization information*. https://www.postgresql.org/docs/18/xfunc-optimization.html
