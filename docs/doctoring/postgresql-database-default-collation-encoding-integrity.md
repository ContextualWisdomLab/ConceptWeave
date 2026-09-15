# PostgreSQL 18 database-default built-in locale encoding integrity

## Problem

The database-default collation successor already preserved `pg_database.datlocprovider`, `datcollate`, `datctype`, `datlocale`, ICU rules, recorded provider version, and capture-time actual provider version. It also restricted the PostgreSQL 18 built-in provider to `C`, `C.UTF-8`, or `PG_UNICODE_FAST`.

That field-domain validation was still incomplete. PostgreSQL 18 makes `C.UTF-8` and `PG_UNICODE_FAST` available only when the database encoding is UTF8. `DatabaseDefaultCollationDefinitionObservation` did not have the source database encoding, while `IndexEffectiveCollationDefinitionSnapshot::new` already received the exact `IndexCollationDatabaseEncodingSnapshot` and did not compose the two invariants. A fabricated non-UTF8 database plus built-in `C.UTF-8` or `PG_UNICODE_FAST` definition could therefore be hashed into governed effective collation evidence.

Review `5205600414` records the P1 finding on #46 exact predecessor `d1a00d052c95fd52227067cbee3987a77a26b4fa`.

## Constraints

- Preserve the already-issued catalog-coordinate, database-encoding, material-definition, provider-version, and effective-definition digest domains.
- Do not infer encoding from locale spelling or normalize raw catalog text.
- Keep `C` valid across PostgreSQL 18 backend/database encodings.
- Require UTF8 for built-in `C.UTF-8` and `PG_UNICODE_FAST`.
- Keep the source database encoding authoritative through `IndexCollationDatabaseEncodingSnapshot`; do not duplicate it inside the database-default value object or transport layer.
- Concrete PostgreSQL I/O remains sequenced after representation acceptance.

## Alternatives considered

### Defer the rule to the future PostgreSQL adapter

Rejected. The representation composition already receives both sides of the invariant and claims to construct valid PostgreSQL 18 effective database-default collation evidence. Allowing a known-impossible tuple until transport would permit invalid semantic evidence to acquire an immutable digest.

### Add database encoding to `DatabaseDefaultCollationDefinitionObservation::new`

Rejected. The value object is responsible for the exact database-default locale definition, while source database encoding is already a separately governed predecessor. Duplicating encoding in the value object would create two authorities and widen existing constructor/digest semantics unnecessarily.

### Bind the two predecessors at effective-definition composition

Selected. The database-default definition exposes a version-specific compatibility validation against `PostgresDatabaseEncodingObservation`, and `IndexEffectiveCollationDefinitionSnapshot::new` invokes it using the exact database-encoding predecessor before hashing the effective successor.

## RED and causal repair

Source RED `c724cc825d1401e23ce5a126b20e0d1fce08ad7d` adds focused contracts that reject LATIN1 plus built-in `C.UTF-8`/`PG_UNICODE_FAST`, while retaining positive controls for `C` under LATIN1 and both UTF8-only built-ins under UTF8.

Production repair `3f25660929e250c0facd16aabe6aef211e3f272a` adds `DatabaseDefaultCollationDefinitionObservation::validate_database_encoding` and composes it inside `IndexEffectiveCollationDefinitionSnapshot::new`. PostgreSQL 18 UTF8 is bound as encoding ID `6`, consistent with the existing PostgreSQL 18 database-encoding contract on this branch. The repair changes admissibility only; it does not reinterpret predecessor digest domains.

No executed Rust RED/GREEN is claimed for these commits. The branch still requires one unchanged exact head to pass the repository-pinned Rust 1.98 native suite and hosted Product gates.

## Evidence and traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review finding: `5205600414`
- RED: `c724cc825d1401e23ce5a126b20e0d1fce08ad7d`
- Production repair: `3f25660929e250c0facd16aabe6aef211e3f272a`
- Production module: `crates/conceptweave-relation-partition/src/database_default_collation.rs`
- Focused contract: `crates/conceptweave-relation-partition/tests/database_default_collation_definition_contract.rs`
- Predecessor encoding module: `crates/conceptweave-relation-partition/src/collation_database_encoding.rs`

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Locale support*. https://www.postgresql.org/docs/18/locale.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE DATABASE*. https://www.postgresql.org/docs/18/sql-createdatabase.html

The PostgreSQL 18 documentation states that the built-in provider supports `C`, `C.UTF-8`, and `PG_UNICODE_FAST`, and that `C.UTF-8` and `PG_UNICODE_FAST` are available only for UTF8 database encoding. `C` remains defined in terms of the database encoding rather than being UTF8-only.

## Risk, effect, and follow-up

The repair closes an impossible-state admission path before immutable effective-definition identity is minted. It does not prove extractor correctness, actual catalog reachability, or live provider behavior.

The concrete PostgreSQL transport descendant must read `pg_database.encoding` and the database-default locale/provider/version fields in the same bounded observation and exercise live differentials for:

- UTF8 + built-in `C`;
- UTF8 + built-in `C.UTF-8`;
- UTF8 + built-in `PG_UNICODE_FAST`;
- a supported non-UTF8 database + built-in `C`;
- rejection or non-creation of non-UTF8 + built-in `C.UTF-8`;
- rejection or non-creation of non-UTF8 + built-in `PG_UNICODE_FAST`.

Transport work remains downstream of unchanged-head representation acceptance and must reacquire terminal GREEN after it moves the head.
