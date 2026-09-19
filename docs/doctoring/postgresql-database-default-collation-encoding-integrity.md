# PostgreSQL 18 database-default locale/provider encoding integrity

## Problem

The database-default collation successor preserves `pg_database.datlocprovider`, `datcollate`, `datctype`, `datlocale`, ICU rules, recorded provider version, and capture-time actual provider version. It also restricts the PostgreSQL 18 built-in provider to `C`, `C.UTF-8`, or `PG_UNICODE_FAST`.

Two representation-level encoding invariants are required before this state can become governed effective-collation evidence.

First, PostgreSQL 18 makes built-in `C.UTF-8` and `PG_UNICODE_FAST` available only when the database encoding is UTF8. The original representation validated the locale vocabulary but did not compose it with the separately governed source database encoding. Review `5205600414` recorded that P1 on #46 predecessor `d1a00d052c95fd52227067cbee3987a77a26b4fa`.

Second, the first repair still left the ICU database provider unconstrained. PostgreSQL 18 does not support ICU for every valid backend encoding. `src/common/encnames.c` defines the authoritative `pg_enc2icu_tbl` and `is_encoding_supported_by_icu()`: SQL_ASCII (`0`), EUC_JIS_2004 (`5`), MULE_INTERNAL (`7`), LATIN10 (`17`), and WIN874 (`21`) are valid backend encodings but have no ICU mapping. `CREATE DATABASE` calls that predicate and rejects an ICU database when the selected encoding is absent. Before the follow-up repair, `DatabaseDefaultCollationDefinitionObservation::validate_database_encoding()` checked only the built-in UTF8-only locales, so impossible ICU database-default tuples could acquire an immutable digest. Review `5206872888` records the P1 finding on exact predecessor `7eda21d6873e9780cabdf60409584ec4b0ae1e53`.

## Constraints

- Preserve the already-issued catalog-coordinate, database-encoding, material-definition, provider-version, and effective-definition digest domains.
- Do not infer encoding from locale spelling or normalize raw catalog text.
- Keep built-in `C` valid across PostgreSQL 18 backend/database encodings.
- Require UTF8 for built-in `C.UTF-8` and `PG_UNICODE_FAST`.
- For ICU, mirror PostgreSQL 18 `is_encoding_supported_by_icu()` over the already-validated backend encoding ID rather than using a heuristic such as “all non-ASCII encodings”.
- Keep the source database encoding authoritative through `IndexCollationDatabaseEncodingSnapshot`; do not duplicate it inside the database-default value object or transport layer.
- Concrete PostgreSQL I/O remains sequenced after representation acceptance.

## Alternatives considered

### Defer the rules to the future PostgreSQL adapter

Rejected. The representation composition already receives both sides of these invariants and claims to construct valid PostgreSQL 18 effective database-default collation evidence. Allowing a known-impossible tuple until transport would permit invalid semantic evidence to acquire an immutable digest.

### Add database encoding to `DatabaseDefaultCollationDefinitionObservation::new`

Rejected. The value object is responsible for the exact database-default locale definition, while source database encoding is already a separately governed predecessor. Duplicating encoding in the value object would create two authorities and widen existing constructor/digest semantics unnecessarily.

### Treat every PostgreSQL backend encoding as ICU-capable

Rejected. PostgreSQL 18 deliberately separates `PG_VALID_BE_ENCODING` from `is_encoding_supported_by_icu()`. Five valid backend encodings have NULL entries in `pg_enc2icu_tbl`, and `CREATE DATABASE` fails them when `datlocprovider='i'`.

### Bind provider-specific encoding rules at effective-definition composition

Selected. The database-default definition exposes a version-specific compatibility validation against `PostgresDatabaseEncodingObservation`, and `IndexEffectiveCollationDefinitionSnapshot::new` invokes it using the exact database-encoding predecessor before hashing the effective successor.

## RED and causal repair

Source RED `c724cc825d1401e23ce5a126b20e0d1fce08ad7d` added focused contracts that reject LATIN1 plus built-in `C.UTF-8`/`PG_UNICODE_FAST`, while retaining positive controls for `C` under LATIN1 and both UTF8-only built-ins under UTF8. Production repair `3f25660929e250c0facd16aabe6aef211e3f272a` added the database-encoding composition seam.

Follow-up source RED `e6108ee01764d47823ce608f47522deb6343ca02` pins the complete PostgreSQL 18 ICU-support boundary: it rejects backend encoding IDs `0`, `5`, `7`, `17`, and `21`, and exercises every ICU-capable backend ID from the PostgreSQL 18 table as a positive control. Production repair `3f6173125e7c817f49174ce582290df112c3e01c` mirrors `is_encoding_supported_by_icu()` at that same composition seam. The repair changes admissibility only; it does not reinterpret predecessor digest domains or derive raw catalog values.

No executed Rust RED/GREEN is claimed for these commits. The branch still requires one unchanged exact head to pass the repository-pinned Rust 1.98 native suite and hosted Product gates.

## Evidence and traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Built-in review finding: `5205600414`
- Built-in RED: `c724cc825d1401e23ce5a126b20e0d1fce08ad7d`
- Built-in production repair: `3f25660929e250c0facd16aabe6aef211e3f272a`
- ICU review finding: `5206872888`
- ICU RED: `e6108ee01764d47823ce608f47522deb6343ca02`
- ICU production repair: `3f6173125e7c817f49174ce582290df112c3e01c`
- Production module: `crates/conceptweave-relation-partition/src/database_default_collation.rs`
- Focused contract: `crates/conceptweave-relation-partition/tests/database_default_collation_definition_contract.rs`
- Predecessor encoding module: `crates/conceptweave-relation-partition/src/collation_database_encoding.rs`
- PostgreSQL 18 authority: `src/common/encnames.c::pg_enc2icu_tbl`, `is_encoding_supported_by_icu()` and `src/backend/commands/dbcommands.c` on `REL_18_STABLE`.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Locale support*. https://www.postgresql.org/docs/18/locale.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE DATABASE*. https://www.postgresql.org/docs/18/sql-createdatabase.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: Encoding names and ICU support (`src/common/encnames.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/common/encnames.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: Database creation validation (`src/backend/commands/dbcommands.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/dbcommands.c

## Risk, effect, and follow-up

The repairs close known impossible-state admission paths before immutable effective-definition identity is minted. They do not prove extractor correctness, actual catalog reachability, operating-system libc compatibility, or live provider behavior.

The concrete PostgreSQL transport descendant must read `pg_database.encoding` and the database-default locale/provider/version fields in the same bounded observation and exercise live differentials for:

- UTF8 + built-in `C`, `C.UTF-8`, and `PG_UNICODE_FAST`;
- a supported non-UTF8 database + built-in `C`;
- rejection or non-creation of non-UTF8 + built-in `C.UTF-8`/`PG_UNICODE_FAST`;
- ICU on representative supported encodings including UTF8 and LATIN1;
- rejection or non-creation of ICU on SQL_ASCII, EUC_JIS_2004, MULE_INTERNAL, LATIN10, and WIN874;
- libc cases separately, because locale/encoding compatibility there depends on the operating-system locale rather than PostgreSQL's static ICU encoding table.

Transport work remains downstream of unchanged-head representation acceptance and must reacquire terminal GREEN after it moves the head.
