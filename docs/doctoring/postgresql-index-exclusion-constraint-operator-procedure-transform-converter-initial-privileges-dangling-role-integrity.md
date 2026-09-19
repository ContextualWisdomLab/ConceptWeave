# PostgreSQL transform-converter `pg_init_privs` dangling-role integrity

## Problem

The converter initial-privilege successor originally required every non-PUBLIC grantee and grantor OID in `pg_init_privs.initprivs` to resolve to a role name in the same source generation. That assumption is too strong for Source Observation. PostgreSQL can retain `pg_init_privs` ACL entries that reference a role OID after the role has been dropped. PostgreSQL 18 bug reports show this state can survive `pg_upgrade --check` and later make dump/restore fail when the numeric OID is emitted as if it were a role identity.

Source Observation therefore has to distinguish three ACL identities instead of treating role lookup as an admission gate: PUBLIC grantee, resolved role name, and unresolved nonzero raw role OID. Grantors distinguish resolved role names from unresolved nonzero raw role OIDs. A real role whose name is the decimal text `"16424"` is not the same fact as an unresolved role OID `16424`.

A second review found a separate validation gap in that repair. The raw OID was committed into the privacy-preserving digest, but the public material exposed only `privilege_type`, total `grant_count`, and `digest`. A later deterministic `validate` or recovery-diagnostic stage therefore could not tell from the immutable observation whether any unresolved grantee or grantor existed at all. Identity preservation alone was insufficient for the stated governance contract.

## Source semantics

`pg_init_privs` stores an `aclitem[]` initial privilege baseline for objects whose initial privileges are non-default. `privtype='i'` records an `initdb` baseline and `privtype='e'` records a baseline set during extension creation. ACL entries carry grantee and grantor identities. PUBLIC is the grantee OID zero; a grantor must identify a role.

The PostgreSQL 18 catalog declaration makes the object coordinate `(objoid, classoid, objsubid)` the unique key, forces `initprivs` non-NULL, and does not include `privtype` in the unique key. The ConceptWeave row-absence-versus-one-material representation is therefore retained rather than widened into an invented multi-row state.

BUG #19483, reported against PostgreSQL 18.3 on 2026-05-18, demonstrates extension-created `pg_init_privs` rows whose ACL grantee OIDs no longer resolve after role reassignment/drop. The reported failure appears during `pg_restore` inside `pg_upgrade`, after `pg_upgrade --check` had declared the clusters compatible. BUG #19513 reproduces the same class of failure on a PostgreSQL 14 to 18 upgrade. A follow-up pgsql-hackers patch thread proposed checking/filtering invalid `pg_init_privs` role references. Later revisions explicitly checked both grantor and grantee existence through `pg_roles` and retained PUBLIC rather than assuming all ACL OIDs resolve.

The PostgreSQL discussion also considered all-digit role names. That matters to the ConceptWeave contract because name text and unresolved raw OID are separate namespaces; converting an unresolved OID to decimal text would permit identity confusion if an actual role has that numeric-looking name.

## ConceptWeave decision

The initial-privilege observation remains the canonical owner of the exact external recovery fact. It does not silently discard dangling ACL entries and does not convert unresolved OIDs into synthetic role names.

`IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` therefore preserves:

- PUBLIC grantee with a resolved or unresolved grantor;
- resolved grantee role name with a resolved or unresolved grantor;
- unresolved nonzero grantee OID with a resolved or unresolved grantor;
- exact grant option.

OID zero is rejected for unresolved-role constructors because zero is PUBLIC only in the grantee namespace and is not a dangling role identity. Existing resolved-role constructors and their digest framing remain stable. Unresolved grantees use a separate digest tag. Unresolved grantors use a reserved framing sentinel before the raw OID so resolved role-name digests remain byte-for-byte compatible and cannot collide with the new raw-OID namespace.

The material now also carries `unresolved_grantee_count` and `unresolved_grantor_count`. These are deterministic projections of the already-observed ACL, not new source identity. They deliberately do not enter the digest: the existing digest already commits to each resolved or unresolved ACL identity and raw dangling OID, so adding derived counts to the digest would create needless identity churn. The counts expose only the minimum recovery-diagnostic signal required by later validation; raw dangling OID values remain privacy-preserving digest material rather than a new public data surface.

This is observation, not repair of PostgreSQL catalogs. Validation/publication can now flag dangling role references as a recovery or security defect from the immutable observation itself while observe continues to preserve the broken source state faithfully.

## RED → repair traceability

The first repair established representability of dangling ACL identities:

- Finding review: `5254640920` at #46 exact head `5daf2a5725ed52cd1b8d884200fc7786eaba54de`.
- Structural RED: `94e98da8ebb31f1429b44b20cce7b96ea16e1332`.
- Production causal repair: `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.

The second repair made the corruption state usable by downstream deterministic validation without exposing raw OIDs:

- Finding review: `5254787838` at #46 exact head `8348be96316f625525b3d89abb732e952c0240e5`.
- Structural RED: `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`; the contract called `unresolved_grantee_count()` and `unresolved_grantor_count()` before production exposed them.
- Production causal repair: `fde6d3a8776b75e3dd014713b671734b9226a0fe`.
- The diagnostic contract requires resolved-only `0/0`, dangling grantee `1/0`, dangling grantor `0/1`, both dangling `1/1`, and PUBLIC with dangling grantor `0/1`.
- The pre-diagnostic decision surface is archived as `docs/archive/CHANGELOG-through-8348be96.md` and `docs/archive/product-technical-gap-baseline-through-8348be96.md`.

Source repair is not exact-head GREEN. Repository-pinned Rust 1.98 fmt, strict Clippy, focused/retained/workspace/doc tests, release/rustdoc, owned production coverage, and the bounded PostgreSQL 18 live differential remain required after the final source/documentation head is known.

## Differential requirement

The PostgreSQL 18 differential must include at least one real or faithfully constructed catalog state in which `pg_init_privs.initprivs` contains a nonzero grantee or grantor OID with no row in `pg_roles`. Capture must preserve the raw OID and must not:

- drop the ACL entry;
- fail the entire source observation merely because role lookup fails;
- serialize the OID as a role-name string;
- resolve it by matching decimal text against an unrelated numeric-looking role name.

The resulting material must also report unresolved grantee/grantor counts consistent with the captured ACL while keeping raw OID values out of the public diagnostic projection. A resolved-only baseline must report zero unresolved references.

The same differential still has to bind the row to `classoid=pg_proc`, the exact converter function `objoid`, and `objsubid=0`, preserve `privtype`, the rest of the ACL, current function/ACL facts, converter extension lifecycle/security facts, immutable raw converter-root lineage, and transform-object extension membership from the same source generation.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h

PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org

PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org

Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com

Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com
