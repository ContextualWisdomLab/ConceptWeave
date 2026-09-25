# PostgreSQL 18 collation provider-shape integrity

## Decision

ConceptWeave must not hash an impossible PostgreSQL 18 `pg_collation` provider/field combination into governed material-definition evidence. `CollationDefinitionObservation` therefore validates both provider-specific catalog shape and the unique provider-`d` bootstrap coordinate before construction while preserving the existing material-definition digest domains and the later database-default effective-definition successor.

This is a representation invariant, not transport normalization. The concrete PostgreSQL adapter must still extract raw catalog values and prove them against a live PostgreSQL 18 differential after the representation lane reaches exact-head acceptance.

## Problem

The material-definition successor introduced at review `5204669655` preserved `collprovider`, `collisdeterministic`, libc/provider locale fields, ICU rules, stored `collversion`, and capture-time actual version, but its constructor only rejected embedded NUL bytes. A caller could therefore construct and hash states PostgreSQL 18 itself does not create coherently, including:

- libc provider with `colllocale` present or one of `collcollate`/`collctype` absent;
- built-in or ICU provider with libc `collcollate`/`collctype` fields present;
- nondeterministic built-in/libc definitions;
- ICU rules attached to a non-ICU provider;
- an arbitrary built-in locale outside PostgreSQL 18's built-in locale set;
- a malformed database-default provider row carrying ordinary locale fields;
- an arbitrary catalog coordinate using provider `d`, which the effective-definition successor would otherwise misinterpret as database-default delegation.

That is an authority defect because the governed digest would distinguish fabricated combinations without first proving that they belong to the PostgreSQL 18 source vocabulary.

## PostgreSQL 18 authority

`REL_18_STABLE` `CollationCreate()` asserts the ordinary created-row shape: libc requires both `collcollate` and `collctype` and forbids `colllocale`; non-libc created rows forbid the two libc fields and require `colllocale`. `CREATE COLLATION` restricts providers exposed to users to builtin/ICU/libc, restricts nondeterministic collations and `RULES` to ICU, and restricts the built-in provider locale to `C`, `C.UTF-8`, or `PG_UNICODE_FAST`.

The bootstrap catalog has one deliberate exception to the ordinary non-libc constructor shape: `pg_catalog.default` uses provider `d`, `collencoding = -1`, default deterministic mode, and no locale/rules fields. Its effective behavior is represented separately by `IndexEffectiveCollationDefinitionSnapshot` over `pg_database`. Because provider `d` is not a user-creatable provider, a governed provider-`d` observation must bind exactly to `(pg_catalog, default, -1)`.

Primary sources:

- PostgreSQL Global Development Group. (2025). *pg_collation.dat* (`REL_18_STABLE`, `src/include/catalog/pg_collation.dat`).
- PostgreSQL Global Development Group. (2025). *pg_collation.c* (`REL_18_STABLE`, `src/backend/catalog/pg_collation.c`).
- PostgreSQL Global Development Group. (2026). *CREATE COLLATION — PostgreSQL 18 documentation*.

## RED → repair

Initial provider/field repair:

- Review: `5204966516` on predecessor `bec344824d1810cbf2629bbc606fde29940dede7`.
- Source RED: `9601b4d11a87bbd5f8339524f9b30aca632b9ed1`, `tests/index_collation_provider_shape_contract.rs`.
- Minimal causal repair: `e26f94b33aa9a89fdc6576c180dba29e6b65d4a9`, `src/collation_definition.rs`.

Provider-`d` coordinate follow-up:

- Review: `5205011408` on predecessor `3deb099c67aa7febe0c440cb961f458206267500`.
- Source RED: `0c1ed9930054d444b388cb54c157eb90f5fc6f9c`, extending the same focused contract with wrong-schema, wrong-name, and wrong-encoding provider-`d` cases.
- Minimal causal repair: `f4dc329433443706e11526bce75e6bcb4892afff`, binding provider `d` to `pg_catalog.default` with raw `collencoding = -1` before construction.

The focused contract pins invalid libc/non-libc field mixtures, non-ICU nondeterminism/rules, the PostgreSQL 18 built-in locale set, the valid ICU nondeterministic/rules case, and the exact bootstrap database-default shape and coordinate. Production rejects violations with `index_collation_definition_provider_shape` before the observation can be hashed.

## Invariants and non-goals

- Existing catalog-coordinate, database-encoding, material-definition, and effective database-default digest domains are not rewritten.
- Stored/current collation version mismatch remains evidence; this repair does not interpret a refresh as index rebuild proof.
- `collowner` and catalog OID remain outside governed semantic identity.
- Provider-specific version-string semantics are not invented beyond the values PostgreSQL exposes.
- No claim of native Rust 1.98 or hosted GREEN is made until one unchanged exact head executes the required gates.

## Follow-up

After representation acceptance, the concrete adapter must obtain the exact used `pg_collation` rows and actual provider versions in the same bounded read-only catalog observation and run live differential cases for libc, built-in, ICU, and the exact `pg_catalog.default` delegation path. The adjacent `pg_database` default-definition constructor is governed by its own provider-shape contract rather than inferred from this `pg_collation` repair.
