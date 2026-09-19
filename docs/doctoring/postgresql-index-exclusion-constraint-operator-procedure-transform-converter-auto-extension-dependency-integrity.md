# PostgreSQL transform-converter auto-extension dependency integrity

## Decision

ConceptWeave Source Observation must preserve converter-function PostgreSQL extension membership and converter-function auto-extension dependency as different lifecycle facts.

For an exact FROM SQL or TO SQL converter function:

- `pg_depend.deptype = 'e'` is extension **membership**. The object is owned by the extension lifecycle and cannot be dropped independently from that extension relationship.
- `pg_depend.deptype = 'x'` is an **auto-extension dependency**, not membership. The function remains independently droppable, but dropping any extension on which it explicitly depends also drops the function. PostgreSQL dump semantics differ from extension-owned membership.
- `ALTER FUNCTION ... DEPENDS ON EXTENSION extension_name` and `ALTER FUNCTION ... NO DEPENDS ON EXTENSION extension_name` mutate `x` edges without changing the converter function's schema/name identity or the `pg_transform` binding. A function can depend on more than one extension.

Therefore `IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation { extension_name: Option<String> }` cannot encode the complete lifecycle state. Absence of an `e` edge does not imply absence of `x` edges.

## Contract shape

`IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation` records one converter coordinate and a complete canonical sorted set of exact resolved `pg_extension.extname` values reached through same-generation `pg_depend.deptype='x'` edges.

The set is explicit even when empty. Multiple names are allowed because PostgreSQL permits multiple explicit extension dependencies. Blank or duplicate names fail closed. Ordering is not semantic, so constructor normalization makes catalog row order irrelevant while the digest remains sensitive to set membership.

The snapshot descends from converter-function `deptype='e'` membership, preserves the immutable raw `converter_snapshot_digest`, and requires complete coordinate plus exact schema/function binding. The later transform-object extension-membership successor descends from this auto-extension snapshot so `x` lifecycle facts cannot disappear from the final successor digest.

## Extraction boundary

For each exact converter function OID selected from the same immutable source generation:

1. preserve any `deptype='e'` membership using the existing membership surface;
2. independently collect `pg_depend` rows with dependent `classid = pg_proc`, exact converter `objid`, `objsubid = 0`, referenced `refclassid = pg_extension`, `refobjsubid = 0`, and `deptype='x'`;
3. resolve every referenced extension OID to exact `pg_extension.extname` in that same snapshot;
4. provide the complete set to the auto-extension dependency constructor.

Naming conventions, package inventory, `extversion`, control files, update scripts, application configuration, and the transform object's own extension membership are not substitutes. Extension-owned metadata stays outside this Source Observation fact.

## Why this is material

This successor is justified by lifecycle and recovery behavior, not by catalog enumeration. Two otherwise identical converter bindings differ operationally when one has an `x` dependency: `DROP EXTENSION` can remove the dependent routine, while the routine is not an extension member and is handled differently by dump/restore tooling. Collapsing the states would make governed semantic evidence unable to explain destructive extension operations or reconstruct lifecycle intent.

## Test traceability

- Finding review: `5254049432` on #46.
- Structural RED: `3347a4c8fe2dab338c138cb4acd0ca0d7fc0e498` introduces a contract against the not-yet-existing auto-extension dependency API.
- Production observation/snapshot: `f52c34dbd385b8d2e50b54a67b125c1a35a777fa`.
- Public composition: `55b0baf3bbb34d0ef24534d29c12a3814fa97176`.
- Transform-object successor contract restack: `458bd76f193906999a81c583da30dff152d0f9f8`.
- Transform-object production restack: `fcb0e2cdd8700ed56ee4d2a0b96bdf89c5bd4532`.
- Retained transform direction/function/raw-root lineage contracts restacked on the new converter lifecycle predecessor: `60d90c3ed0b0793378707a39cd27397dba6ed3bb`.

The exact head after documentation movement still requires independent Rust 1.98, coverage, and PostgreSQL 18 same-generation differential evidence before GREEN.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: DROP EXTENSION*. https://www.postgresql.org/docs/18/sql-dropextension.html
