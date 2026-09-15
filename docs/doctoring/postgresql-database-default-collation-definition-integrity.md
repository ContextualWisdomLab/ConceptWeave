# PostgreSQL 18 database-default collation definition integrity

## Problem

The material `pg_collation` successor closes catalog-coordinate, provider, locale/rules, stored-version, and current-provider-version gaps for ordinary collation objects. One PostgreSQL 18 provider token has an additional delegation boundary: `pg_collation.collprovider = 'd'` means the collation uses the current database default. The behaviorally material default definition lives in `pg_database`, not solely in that `pg_collation` row.

PostgreSQL 18 `pg_database` records the database encoding and default locale definition: `datlocprovider` (`b`, `c`, `i`), `datcollate`, `datctype`, `datlocale`, `daticurules`, and `datcollversion`. The system administration function `pg_database_collation_actual_version(oid)` returns the current provider version for the database collation. PostgreSQL also provides `ALTER DATABASE ... REFRESH COLLATION VERSION`; this is the database-default analogue of `ALTER COLLATION ... REFRESH VERSION`.

Consequently, two bounded source observations can use the same `pg_catalog.default` catalog coordinate and the same material `pg_collation` row while having different effective database-default provider/locale/rules/version state. Treating provider `d` as fully described by the `pg_collation` row would allow those source states to collapse into one governed semantic identity.

## PostgreSQL 18 authority

PostgreSQL Global Development Group. (2026). *pg_database (PostgreSQL 18 system catalog)*. https://www.postgresql.org/docs/18/catalog-pg-database.html

PostgreSQL Global Development Group. (2026). *System administration functions: collation management (PostgreSQL 18)*. https://www.postgresql.org/docs/18/functions-admin.html

PostgreSQL Global Development Group. (2026). *ALTER DATABASE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-alterdatabase.html

PostgreSQL Global Development Group. (2026). *ALTER COLLATION (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-altercollation.html

The catalog authority states that `datlocprovider` admits built-in, libc, or ICU and that provider locale/rules plus recorded `datcollversion` are database-level fields. The function authority defines `pg_database_collation_actual_version(oid)` as the current operating-system/provider version and warns that mismatch from `datcollversion` can require rebuilding dependent objects. `ALTER COLLATION` explicitly points to `ALTER DATABASE ... REFRESH COLLATION VERSION` for the database default.

## Repair lineage

Review `5204715861` on exact #46 head `4f6d1e4683fda3a5ca7d059b6fe29bd061af781b` identified the remaining provider-`d` delegation gap.

Source RED `73464e2e8228a62bdae00555a1c24dd26ab66bd1` introduces a compile-time contract for database-default definition evidence. It requires:

- the same recorded database version with a changed actual provider version to alter governed identity;
- a changed database-default locale to alter governed identity; and
- PostgreSQL 18 `datlocprovider` to admit only `b`, `c`, and `i`, rejecting recursive `d` and unknown provider tokens.

Production `bf44d3dcdd83ffddb41314786becfacb9a007a71` adds `DatabaseDefaultCollationDefinitionObservation`, `PostgresDatabaseLocaleProvider`, provenance receipt support, and `IndexEffectiveCollationDefinitionSnapshot`. Export `649c791301cb28137744cb290a2ee0a308144a98` exposes the successor through the relation/index-partition contract.

## Chosen contract

`DatabaseDefaultCollationDefinitionObservation` preserves raw database-level provider state without guessing defaults:

- validated `pg_database.datlocprovider` (`b`, `c`, `i`);
- nullable `datcollate` and `datctype`;
- nullable `datlocale`;
- nullable `daticurules`;
- recorded nullable `datcollversion`; and
- capture-time nullable `pg_database_collation_actual_version(database_oid)`.

Stored and current versions remain separate. A mismatch is evidence that dependent default-collation objects may need rebuilding; ConceptWeave does not silently refresh it or infer that `REINDEX` occurred.

`IndexEffectiveCollationDefinitionSnapshot` is a domain-separated successor above `IndexCollationDefinitionSnapshot`. It reconstructs the material-definition predecessor from the exact database-encoding, per-key coordinate, and expression/relation-`Var` predecessors and requires the same source connection, policy binding, extractor revision, and observation time. It then inspects the bounded used material definitions:

- if any used collation has `collprovider = 'd'`, exactly one database-default definition must be supplied;
- if no used collation delegates to provider `d`, supplying unrelated database-default definition evidence fails closed.

The effective successor digest binds the exact material-definition predecessor digest plus the complete database-default definition only when required. Issued coordinate, database-encoding, and material-definition digest meanings remain unchanged.

## Rejected alternatives

Folding `pg_database` fields into `CollationCatalogIdentity` was rejected because the catalog coordinate predecessor remains correct for its narrower purpose and rewriting it would invalidate issued evidence semantics.

Folding database-default fields into every `CollationDefinitionObservation` was rejected because only provider `d` delegates to database-level locale state; doing so for ordinary built-in/libc/ICU collation objects would mix unrelated source facts into their identity.

Using only `datcollversion` was rejected because the provider can change before the stored version is refreshed. Using only `pg_database_collation_actual_version()` was rejected because the recorded/current mismatch is operationally significant evidence.

Treating `ALTER DATABASE ... REFRESH COLLATION VERSION` as proof that affected indexes were rebuilt was rejected. PostgreSQL version refresh records current provider state; it does not prove that all dependent stored objects were repaired.

## Remaining boundary

This is representation source evidence, not executed Rust acceptance or concrete PostgreSQL transport. The future PostgreSQL 18 extractor must capture the current database row and `pg_database_collation_actual_version(database_oid)` in the same bounded source observation as used collation rows, database encoding, expressions, and raw `pg_attribute.atttypmod`. Live differential coverage must include a provider-`d` case where the used collation coordinate remains unchanged while database locale/provider version changes. Exact-head Rust 1.98 and hosted Product acceptance remain prerequisites before transport evidence is promoted.
