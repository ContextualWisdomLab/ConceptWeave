# PostgreSQL transform-converter `pg_init_privs` dangling-role integrity

## Problem

The converter initial-privilege successor originally required every non-PUBLIC grantee and grantor OID in `pg_init_privs.initprivs` to resolve to a role name in the same source generation. That assumption is too strong for Source Observation. PostgreSQL can retain `pg_init_privs` ACL entries that reference a role OID after the role has been dropped. PostgreSQL 18 bug reports show this state can survive `pg_upgrade --check` and later make dump/restore fail when the numeric OID is emitted as if it were a role identity.

Source Observation therefore has to distinguish three ACL identities instead of treating role lookup as an admission gate: PUBLIC grantee, resolved role name, and unresolved nonzero raw role OID. Grantors distinguish resolved role names from unresolved nonzero raw role OIDs. A real role whose name is the decimal text `"16424"` is not the same fact as an unresolved role OID `16424`.

## Source semantics

`pg_init_privs` stores an `aclitem[]` initial privilege baseline for objects whose initial privileges are non-default. `privtype='i'` records an `initdb` baseline and `privtype='e'` records a baseline set during extension creation. ACL entries carry grantee and grantor identities. PUBLIC is the grantee OID zero; a grantor must identify a role.

BUG #19483, reported against PostgreSQL 18.3 on 2026-05-18, demonstrates extension-created `pg_init_privs` rows whose ACL grantee OIDs no longer resolve after role reassignment/drop. The reported failure appears during `pg_restore` inside `pg_upgrade`, after `pg_upgrade --check` had declared the clusters compatible. A follow-up pgsql-hackers patch thread proposed checking/filtering invalid `pg_init_privs` role references. Later revisions explicitly checked both grantor and grantee existence through `pg_roles` and retained PUBLIC rather than assuming all ACL OIDs resolve.

The PostgreSQL discussion also considered all-digit role names. That matters to the ConceptWeave contract because name text and unresolved raw OID are separate namespaces; converting an unresolved OID to decimal text would permit identity confusion if an actual role has that numeric-looking name.

## ConceptWeave decision

The initial-privilege observation remains the canonical owner of the exact external recovery fact. It does not silently discard dangling ACL entries and does not convert unresolved OIDs into synthetic role names.

`IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` therefore preserves:

- PUBLIC grantee with a resolved or unresolved grantor;
- resolved grantee role name with a resolved or unresolved grantor;
- unresolved nonzero grantee OID with a resolved or unresolved grantor;
- exact grant option.

OID zero is rejected for unresolved-role constructors because zero is PUBLIC only in the grantee namespace and is not a dangling role identity. Existing resolved-role constructors and their digest framing remain stable. Unresolved grantees use a separate digest tag. Unresolved grantors use a reserved framing sentinel before the raw OID so resolved role-name digests remain byte-for-byte compatible and cannot collide with the new raw-OID namespace.

This is observation, not repair of PostgreSQL catalogs. Validation/publication may flag dangling role references as a recovery or security defect, but observe must first preserve them faithfully.

## RED → repair traceability

- Finding review: `5254640920` at #46 exact head `5daf2a5725ed52cd1b8d884200fc7786eaba54de`.
- Structural RED: `94e98da8ebb31f1429b44b20cce7b96ea16e1332`. The new contract referenced dangling-role constructors before production exposed them.
- Production causal repair: `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- The new contract requires distinct digests for a resolved numeric-looking role name and the same numeric value as an unresolved raw OID, supports dangling grantee/grantor/both, and rejects unresolved OID zero.

Source repair is not exact-head GREEN. Repository-pinned Rust 1.98 fmt, strict Clippy, focused/retained/workspace/doc tests, release/rustdoc, owned production coverage, and the bounded PostgreSQL 18 live differential remain required after the final source/documentation head is known.

## Differential requirement

The PostgreSQL 18 differential must include at least one real or faithfully constructed catalog state in which `pg_init_privs.initprivs` contains a nonzero grantee or grantor OID with no row in `pg_roles`. Capture must preserve the raw OID and must not:

- drop the ACL entry;
- fail the entire source observation merely because role lookup fails;
- serialize the OID as a role-name string;
- resolve it by matching decimal text against an unrelated numeric-looking role name.

The same differential still has to bind the row to `classoid=pg_proc`, the exact converter function `objoid`, and `objsubid=0`, preserve `privtype`, the rest of the ACL, current function/ACL facts, converter extension lifecycle/security facts, immutable raw converter-root lineage, and transform-object extension membership from the same source generation.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org

Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com

Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com
