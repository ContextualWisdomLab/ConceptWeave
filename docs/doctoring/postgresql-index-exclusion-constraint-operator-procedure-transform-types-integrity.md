# PostgreSQL ordinary EXCLUDE operator function transform-type integrity

## Decision status

Proposed Source Observation successor for ConceptWeave PR #46. This decision is observational and does not create a PostgreSQL admission rule.

## Problem

The ordinary-EXCLUDE chain already binds each exact `pg_constraint.conexclop` position through `pg_operator.oprcode` to its target `pg_proc` function and preserves execution, security, implementation-definition, owner, configuration, ACL, planner-support, and planner-cost facts. It did not preserve `pg_proc.protrftypes`.

PostgreSQL 18 stores `protrftypes` independently as the nullable array of argument/result type OIDs for which the function call applies transforms selected by its `TRANSFORM FOR TYPE` clause. `CREATE FUNCTION` states that these transforms convert between SQL types and language-specific data types. The same target function signature, language, source definition, ACL, planner support, and planner cost therefore do not by themselves prove identical transform selection.

This is material to semantic source identity because `CREATE OR REPLACE FUNCTION` preserves function identity when name and input argument types remain the same while reassigning other specified or implied properties. A governed snapshot that omits transform selection can collapse runtime-conversion-distinct catalog states.

## Constraints

- Preserve the frozen meaning of every predecessor digest; add one domain-separated successor.
- Read `protrftypes` from the same exact target `pg_proc` row and same v3 source-content generation as the retained facts.
- Preserve PostgreSQL's documented NULL state when no transform types are selected. Do not normalize NULL to an invented explicit empty array.
- Resolve each non-null type OID to an exact qualified type in the same source generation before domain construction. Unresolved OIDs are capture failures outside this value object, not absent transforms.
- Treat the declared transform types as a set: SQL syntax has no transform-order semantics. Canonicalize qualified types for deterministic hashing while rejecting duplicate input rather than silently collapsing it.
- Do not infer transform selection from `prolang`, function source, argument/result types, or the existence of `pg_transform` rows.
- Do not claim that selected transform types authenticate the actual `pg_transform` converter functions. `pg_transform` separately identifies `(trftype, trflang)` and optional `trffromsql`/`trftosql` functions; exact converter identity requires a separately reviewed successor.

## Alternatives considered

### Omit `protrftypes` because target signature and language are already governed

Rejected. PostgreSQL stores `protrftypes` independently and `TRANSFORM FOR TYPE` changes conversion behavior without changing the call signature.

### Derive transform selection from all `pg_transform` rows for the implementation language

Rejected. `pg_transform` describes transforms available for a type/language pair, while `protrftypes` records which transforms this routine call selected. Availability is not selection.

### Bind selected type plus the complete `pg_transform` converter definition in this successor

Rejected for this increment. That would cross from one same-row `pg_proc` fact into another catalog family with its own converter procedures, implementation definitions, ownership, ACL, and lifecycle. Mixing those concerns would enlarge the causal fix and overstate what this source evidence proves. The residual converter-binding gap remains explicit.

### Normalize NULL and an explicit empty array to the same state

Rejected. PostgreSQL documents NULL when no transforms apply. Source Observation preserves the raw documented absence state rather than silently collapsing a distinguishable catalog representation.

## Decision

Add `IndexExclusionConstraintOperatorProcedureTransformTypesObservation`, `IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt`, and `IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot` as a successor to `IndexExclusionConstraintOperatorProcedureCostSnapshot`.

Each observation is bound to the exact `(constraint coordinate, key_position)`, operator signature, and target function signature already admitted by the planner-cost predecessor. The transform value is either NULL or a nonempty same-generation resolved set of `QualifiedTypeName`. Non-null input is sorted by `(schema_name, type_name)` for deterministic set identity; duplicate types and an explicit empty array fail closed.

The successor digest includes the predecessor digest, exact coordinate and position, operator and target-function signatures, an explicit NULL/non-NULL discriminator, and every canonical qualified transform type. No predecessor digest is rewritten.

## RED, repair, and traceability

- Finding review: `5232628816` on exact predecessor `781f8d57c83c722596fd5f3759f54dc6a001a803`.
- Structural source/compile RED: `6c2aa9f3470c00a4fdd34ed6025a35640455ca58`; the referenced public transform-type types did not yet exist. No executed compiler failure is claimed.
- RED contract refinement: `92d8141e102c33a18b89c05a586ee8c2054efbcd`; preserves NULL explicitly and rejects explicit empty-array input.
- Production successor: `e12b961179a9b22ca9429eeaf1844d8fe8563323`.
- Public composition: `ba11bd4333003b824a4128d32782a19a7a02a0ca`.
- Pre-transform-type active decision surface: `docs/archive/product-technical-gap-baseline-through-781f8d57.md`.
- Pre-transform-type changelog: `docs/archive/CHANGELOG-through-781f8d57.md`.
- Production module: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_types.rs`.
- Focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_types_contract.rs`.

Focused tests require exact transform provenance, NULL-versus-selection digest separation, set-order canonicalization, explicit-empty and duplicate rejection, operator/function binding integrity, complete unique coordinate coverage, one-based positions, exact receipt coordinates, and public composition.

## Risks and limits

This successor proves declared routine transform selection, not the identity or immutability of converter implementations. A selected type can resolve to a mutable `pg_transform` row whose `trffromsql` or `trftosql` target functions change independently. A later material-gap review must decide whether governed ordinary-EXCLUDE execution requires exact same-generation `(type, language) -> pg_transform -> converter pg_proc` binding and, if so, extend the chain without retroactively changing this digest.

The current execution environment has not established repository-pinned Rust 1.98 compilation or tests for this lineage. Structural RED and source-shaped repair remain distinct from native or hosted GREEN.

## Expected effect

Two otherwise identical ordinary-EXCLUDE source snapshots can no longer collapse solely because their target function has different declared transform-type selection. The source representation remains deterministic, bounded, privacy-neutral, and faithful to PostgreSQL catalog ownership without claiming foreign runtime authority.

## Follow-up

The bounded PostgreSQL 18 live differential must read exact same-row `pg_proc.protrftypes`, preserve NULL, resolve every non-null OID against the same-generation `pg_type`/namespace identity, and feed that resolved evidence to this successor. Existing operator-family/strategy, backing-index, and retained target-function facts remain in the same v3 source-content generation.

After exact-head native and hosted acceptance plus this live differential, continue bounded material-catalog review. The next separate candidate gap is exact `pg_transform` converter identity for selected `(type, language)` pairs; it must not be silently folded into the transform-selection contract.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html
