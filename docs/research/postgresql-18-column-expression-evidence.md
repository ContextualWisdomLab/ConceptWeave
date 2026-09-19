# PostgreSQL 18 column-expression evidence

Status: Proposed Source Observation contract. This note supports PR #46 and does not claim native or hosted acceptance.

## Problem

The representation-v3 successor now preserves `pg_attribute.attgenerated` independently, but generation mode is not the expression itself. PostgreSQL stores explicit column default expressions and generated-column expressions in `pg_attrdef`; `pg_attribute.atthasdef` only says that such a row exists, while `attgenerated` distinguishes an ordinary default from a generated column. A snapshot that records type, nullability, generation mode, identity mode, and collation while omitting the `pg_attrdef` expression can therefore collapse materially different source schemas into one governed identity.

This is observable behavior, not DDL decoration. `ALTER TABLE ... ALTER COLUMN ... SET DEFAULT` changes the value used by later writes that omit the column. `ALTER TABLE ... ALTER COLUMN ... SET EXPRESSION AS` replaces the expression of a generated column; for a stored generated column PostgreSQL rewrites existing stored data and uses the new expression for future changes. PostgreSQL inheritance and partition rules also require generation status to agree while permitting generation expressions to differ, so `attgenerated` cannot stand in for expression identity.

## Source authority

The bounded PostgreSQL adapter should read `pg_attribute.atthasdef` and `attgenerated` together with the matching `pg_attrdef` row in the same `REPEATABLE READ READ ONLY` catalog snapshot. `pg_attrdef.adrelid` and `adnum` are the capture-time joins back to the exact relation and attribute. `adbin` is PostgreSQL's internal `pg_node_tree` representation; PostgreSQL documents `pg_get_expr(adbin, adrelid)` as the SQL-expression rendering. Catalog OIDs and the internal node-tree serialization are not governed semantic coordinates.

When the column-expression family is claimed, every bounded column must have one explicit state:

- no expression (`atthasdef = false`);
- ordinary default expression (`atthasdef = true`, `attgenerated = ''`, exact server-rendered expression retained);
- generation expression (`atthasdef = true`, `attgenerated in {'s','v'}`, exact server-rendered expression retained).

Unexpected catalog combinations fail closed. In particular, a generated column with no expression, an ordinary column labelled with a generation expression, or a generated column labelled with an ordinary default expression is contradictory evidence. The already-owned column-generation family remains declaration authority for ordinary/stored/virtual mode; this expression family must not infer generation mode from expression text.

## Identity and canonicalization

Do not alter frozen `ColumnObservationV3`. Add an optional complete observed family after column-generation evidence so legacy snapshots with an unobserved expression family preserve their existing digest meaning.

The family needs exact schema/relation/relation-kind/column coordinates, explicit expression state, and exact server-rendered expression text for present expressions. Canonicalize by exact column coordinate, reject duplicates and incomplete bounded coverage, and domain-separate the digest from the preceding snapshot digest. Input order must not create a second identity.

The expression text is source evidence rather than parsed ConceptWeave semantic truth. Dependency extraction, function volatility/leakproof/security properties, generated-column privilege analysis, expression equivalence/canonical algebra, and application-domain meaning are separate later contracts. Do not promote a parser-derived interpretation into authority merely because the server-rendered text was captured.

## Alternatives considered

### Leave expressions outside governed identity

Rejected. Distinct defaults change future insert behavior, and distinct generation expressions change computed values while all already-modeled column attributes may remain equal.

### Put the expression into `ColumnObservationV3`

Rejected. v3 is already a frozen compatibility boundary. Retrofitting a field would silently change historical digest meaning and make unobserved versus explicitly absent expression state ambiguous.

### Hash `pg_attrdef.adbin`

Rejected as governed identity. PostgreSQL documents `adbin` as an internal `pg_node_tree` representation. It is useful inside one capture transaction but is not the stable semantic coordinate ConceptWeave should expose to consumers.

### Infer default/generated kind from expression shape

Rejected. `pg_attribute.attgenerated` is the direct catalog declaration and must remain authoritative. Expression text does not safely reconstruct catalog state.

## RED traceability

Owner finding: PR #46 review `5189444945` on exact predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386`.

Behavioral/source RED: `afc9509f021681e6f6a2d0e6d0c386cce46fad12`, `crates/conceptweave-observation/tests/column_expression_contract.rs`.

The RED requires:

- different ordinary defaults produce different governed digests;
- different generation expressions produce different governed digests even when `attgenerated` is unchanged;
- explicit no-expression evidence differs from an unobserved expression family;
- observed expression evidence is complete for every bounded column;
- duplicate coordinates fail closed;
- input order is identity-neutral;
- expression kind and the already-observed column-generation mode agree.

The production API intentionally does not exist at this RED checkpoint. No GREEN claim is made until the source is implemented and repository-pinned Rust 1.98 plus applicable hosted gates pass on one unchanged exact head.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attrdef*. https://www.postgresql.org/docs/18/catalog-pg-attrdef.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Generated columns*. https://www.postgresql.org/docs/18/ddl-generated-columns.html
