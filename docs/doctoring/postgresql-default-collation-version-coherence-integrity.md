# PostgreSQL 18 default-collation version coherence integrity

## Problem

PostgreSQL 18 represents the current database default through two catalog layers that must agree inside one bounded source observation. The bootstrap `pg_catalog.default` row has `collprovider = 'd'`, `collencoding = -1`, and no stored `pg_collation.collversion`. For that exact default OID, `pg_collation_actual_version(DEFAULT_COLLATION_OID)` does not read provider fields from the bootstrap row; it resolves the current database's `pg_database.datlocprovider` and `datcollate` or `datlocale`. `pg_database_collation_actual_version(MyDatabaseId)` resolves the same current database provider/locale through the database catalog.

Before this repair, ConceptWeave allowed two contradictions. First, `CollationDefinitionObservation` could assign a stored `collversion` to the provider-`d` bootstrap row even though PostgreSQL 18 does not store one there. Second, `IndexEffectiveCollationDefinitionSnapshot` required database-default evidence when provider `d` was used but did not prove that the material default row's capture-time actual version equaled the database-default observation's capture-time actual version. One bounded observation could therefore hash two different current versions for the same effective database locale.

A follow-up review found a second-order false-positive. Once the default row correctly retained `version = NULL` and a database-derived `actual_version`, the generic material `has_version_mismatch()` predicate would report every versioned provider-`d` database as dirty. PostgreSQL does not refresh that bootstrap row with `ALTER COLLATION ... REFRESH VERSION`; it explicitly directs the default collation to `ALTER DATABASE ... REFRESH COLLATION VERSION`. Recorded-versus-actual drift for provider `d` therefore belongs to the database-default observation, not the material bootstrap row.

## Constraints

The repair must preserve issued digest domains, exact raw optional version evidence, catalog-coordinate predecessors, database-encoding predecessors, and source provenance. It must not synthesize a stored version for `pg_catalog.default`, copy `pg_database.datcollversion` into `pg_collation.collversion`, normalize an unavailable actual version, or infer that a successful version refresh proves dependent indexes have been rebuilt.

The equality between the two actual-version observations applies to the same bounded current database: ConceptWeave's provider-`d` material row and its `pg_database` default evidence are captured from one source observation. Cross-database comparisons are outside this invariant.

## Decision

Review `5206721529` records the primary P1 at predecessor `7fe0f250b6acf379a6d667abc8cc33ca41e245e2`.

Source/compile RED `5efa50beed136f497ebe0769ac310ad97ca0df68` adds `database_default_collation_version_coherence_contract.rs`. It requires the exact provider-`d` bootstrap row to reject a stored `pg_collation.collversion`, preserves capture-time actual-version availability, and requires the material default actual version to equal the database-default actual version, including `None/None` as a coherent unavailable state.

Production `6f659172ae8a676e01b53d346c024d82e2a3e0b0` rejects a non-NULL stored version on `PostgresCollationProvider::DatabaseDefault` before governed hashing. Production `0f05f80b52edb6c8b4d1df9dda301ebdd1a4a033` adds `DatabaseDefaultCollationDefinitionObservation::validate_material_default_collation` and invokes it from `IndexEffectiveCollationDefinitionSnapshot`, binding the exact provider-`d` material actual version to the database-default actual version.

Follow-up review `5206762510` records the provider-`d` mismatch-delegation finding. RED `8e2ed18a08ad443776a9d5d72c5d4c84018efd87` requires material provider-`d` evidence not to report a false ordinary-collation mismatch solely because its stored `collversion` is structurally absent, while the database-default observation continues to report `datcollversion` versus actual-version drift. Production `ec02e73b45bce749d208737ecdf552f3c1746b27` delegates that mismatch responsibility to the database-default layer and preserves ordinary builtin/libc/ICU mismatch semantics.

## Alternatives rejected

Copying `pg_database.datcollversion` into the provider-`d` material observation was rejected because it would fabricate a `pg_collation` field that is absent from the bootstrap row and collapse two catalog authorities. Ignoring the two actual-version observations was rejected because both PostgreSQL functions resolve the same current database locale for the default OID, so disagreement is contradictory source evidence rather than legitimate semantic variance. Treating `None/Some` on the bootstrap row as a material mismatch was rejected because the missing stored value is structural, not provider-version availability loss; the database-default layer owns the recorded baseline.

## Traceability and acceptance

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Primary review: `5206721529`
- Primary RED: `5efa50beed136f497ebe0769ac310ad97ca0df68`
- Stored-version repair: `6f659172ae8a676e01b53d346c024d82e2a3e0b0`
- Actual-version coherence repair: `0f05f80b52edb6c8b4d1df9dda301ebdd1a4a033`
- Follow-up review: `5206762510`
- Follow-up RED: `8e2ed18a08ad443776a9d5d72c5d4c84018efd87`
- Mismatch-delegation repair: `ec02e73b45bce749d208737ecdf552f3c1746b27`
- Contract: `crates/conceptweave-relation-partition/tests/database_default_collation_version_coherence_contract.rs`
- Production: `crates/conceptweave-relation-partition/src/collation_definition.rs`, `crates/conceptweave-relation-partition/src/database_default_collation.rs`

This remains source-level RED → repair evidence until one unchanged exact head passes repository-pinned Rust 1.98 formatting, strict Clippy, focused/workspace/doc tests, rustdoc/release/coverage and applicable hosted Product/security/review gates. Concrete PostgreSQL transport must capture both actual-version functions inside the same bounded observation and prove equal results for the exact default OID/current database pair; a disagreement must fail the observation rather than be normalized.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: bootstrap collation catalog (`REL_18_STABLE`)* [Source code]. GitHub. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_collation.dat

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: collation commands (`REL_18_STABLE`)* [Source code]. GitHub. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: database commands (`REL_18_STABLE`)* [Source code]. GitHub. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/dbcommands.c
