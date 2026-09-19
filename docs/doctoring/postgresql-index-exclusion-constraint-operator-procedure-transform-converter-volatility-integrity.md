# PostgreSQL transform-converter volatility integrity

## Decision

ConceptWeave records raw `pg_proc.provolatile` as an independent same-row Source Observation fact for every nonzero `pg_transform` converter direction. The observation layer admits the three catalog states `i` (immutable), `s` (stable), and `v` (volatile), and domain-separates them in the successor digest.

That does **not** mean PostgreSQL admits all three when a transform is created. PostgreSQL 18 `src/backend/commands/functioncmds.c::check_transform_function()` rejects `PROVOLATILE_VOLATILE` before `CREATE TRANSFORM` accepts either converter function. The same checker separately requires a normal function, a non-set return, exactly one argument, and an `internal` argument type.

The creation-time predicate cannot safely be turned into a raw-capture rejection. PostgreSQL 18 `ALTER FUNCTION` explicitly permits changing an existing function to `IMMUTABLE`, `STABLE`, or `VOLATILE`. The transform checker is used while creating the transform; it is not an `ALTER FUNCTION` revalidation hook. A converter referenced by an existing transform can therefore be observed later with raw `provolatile='v'`. Source Observation must retain that drift state rather than make the live catalog impossible to represent. A validation layer may classify such a state as not admissible for fresh `CREATE TRANSFORM`; it must not rewrite or discard the source fact.

This distinction is important for the `observe -> discover -> propose -> align -> validate -> review -> publish` boundary. Observation preserves what PostgreSQL currently stores. Validation decides whether that state satisfies a governed rule. Collapsing those concerns would hide exactly the post-creation drift that governance is expected to detect.

## Invariants

For one exact converter-strictness predecessor, the volatility successor covers the same `(constraint coordinate, key position, transform type, direction)` inventory. Missing, extra, or duplicate coordinates fail closed. Converter schema and function name must match the predecessor direction exactly. Only raw PostgreSQL states `i`, `s`, or `v` are admitted; any other discriminator fails closed. Changes among the three states change the successor digest.

A live `v` observation is preserved as evidence. It is not silently treated as creation-admissible. PostgreSQL 18 `check_transform_function()` remains the primary authority that a fresh transform definition must not use a volatile converter.

Canonical provenance locations preserve the quoted-identifier collision repair: transform schema and type-name components are percent-encoded independently before the literal separator. `("payload.domain", "json")` and `("payload", "domain.json")` therefore remain distinct. Receipt lookup is exact-coordinate, exact-position, exact-transform-type, and exact-direction bound.

No optimization claim is derived from this observation. PostgreSQL documents volatility as a promise used by the optimizer and as a determinant of snapshot visibility. Capturing the raw state makes that behavior and later drift auditable; it does not prove that the function is correctly labelled by its author.

## Review and repair lineage

- Review `5249791627` identified the missing `provolatile` fact at #46 exact `7ffc577908dd0e87ed950a3bc8c5c2b952788e04`.
- RED `2bf4001dc1cee62df3d826fe585e246528c450cb` added the executable raw-volatility contract before public volatility types existed.
- Production `85265a29730698c6847a1089357a0a18e70c77d8` added the raw volatility observation/snapshot/receipt implementation.
- Public composition `d875c26cdd47c008015e8dfd0d769fc16ba8b5d0` exported the successor through `index_partition.rs`.
- Review `5251665634` later noticed that `check_transform_function()` rejects `VOLATILE` and initially interpreted that as a Source Observation admission requirement.
- RED `a4a735d2bda47b5cbad7cdf6df6405c76f499fc6` and production `9de4b4ae2ddaa1339116a03d15ca13e5f4b19cef` temporarily encoded that interpretation.
- Primary-source follow-up found the missing lifecycle distinction: PostgreSQL 18 `ALTER FUNCTION` can change volatility independently after a transform exists. Correction review `5251695984` therefore withdrew the raw-capture rejection.
- Ordinary-forward `b4df9bef3c5eeb892a2ac47e208d06f9bb0f5bb0` restored `i|s|v` observation while documenting creation-time admission separately; `7ebe2a484330d882a8aa857fdb53333f524e09eb` pins volatile as an observable post-creation drift witness.

The temporary interpretation remains in history rather than being rewritten or force-pushed away. Its correction is part of the traceable engineering decision.

No Rust execution result is attached to this current lineage yet. Source-shaped RED/repair evidence is not exact-head GREEN.

## Traceability

| Concern | Owner evidence |
| --- | --- |
| Raw PostgreSQL catalog discriminator | PostgreSQL 18 `pg_proc.provolatile` (`i`, `s`, `v`) |
| Creation-time transform volatility predicate | PostgreSQL 18 `src/backend/commands/functioncmds.c::check_transform_function()` rejects `PROVOLATILE_VOLATILE` |
| Post-creation volatility mutability | PostgreSQL 18 `ALTER FUNCTION ... IMMUTABLE \| STABLE \| VOLATILE` |
| Raw converter `provolatile` observation, validation and digest | `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_volatility.rs` |
| Public composition | `crates/conceptweave-relation-partition/src/index_partition.rs` |
| Completeness, binding drift, invalid state, duplicate, receipt, digest, quoted-identifier and volatile-drift contracts | `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_volatility_contract.rs` |
| Exact predecessor converter inventory and binding | `IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot` |
| Validation boundary | A fresh-transform admissibility rule may reject observed `v`; Source Observation retains it |

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: functioncmds.c (`check_transform_function`)*. `src/backend/commands/functioncmds.c`.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
