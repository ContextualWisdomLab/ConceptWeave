# PostgreSQL transform-converter `pg_init_privs` dangling-role integrity

## Problem

The converter initial-privilege successor originally required every non-PUBLIC grantee and grantor OID in `pg_init_privs.initprivs` to resolve to a role name in the same source generation. That assumption is too strong for Source Observation. PostgreSQL can retain `pg_init_privs` ACL entries that reference a role OID after the role has been dropped. PostgreSQL 18 bug reports show this state can survive `pg_upgrade --check` and later make dump/restore fail when the numeric OID is emitted as if it were a role identity.

Source Observation therefore has to distinguish PUBLIC, resolved role names, and unresolved nonzero raw role OIDs. A real role whose name is the decimal text `"16424"` is not the same fact as unresolved role OID `16424`.

The first diagnostic repair retained only unresolved grantee/grantor counts outside the material digest. That was sufficient to block recovery readiness but not sufficient to identify which missing catalog roles required investigation. Equal counts can describe different damage. The admitted recovery boundary now retains exact canonical dangling-role sets for purpose-bound validation/remediation while keeping them out of routine `Debug` output.

A later log-safety review found one remaining escape hatch: the public ACL grant value itself still derived `Debug`, and its private unresolved-role enum variants also derived `Debug`. A caller could therefore log the raw dangling OID before the grant was reduced into the already-redacted material or recovery-validation types. The log boundary has to hold at every public routine `Debug` surface, not only after material construction.

## Source semantics

`pg_init_privs` stores an `aclitem[]` initial privilege baseline for objects whose initial privileges are non-default. `privtype='i'` records an `initdb` baseline and `privtype='e'` records a baseline set during extension creation. ACL entries carry grantee and grantor identities. PUBLIC is the grantee OID zero; a grantor must identify a role.

The PostgreSQL 18 catalog declaration makes `(objoid, classoid, objsubid)` the unique key, forces `initprivs` non-NULL, and does not include `privtype` in the unique key. ConceptWeave therefore retains row absence versus one exact material rather than inventing simultaneous per-`privtype` rows for one object coordinate.

BUG #19483, reported against PostgreSQL 18.3 on 2026-05-18, demonstrates extension-created `pg_init_privs` rows whose ACL grantee OIDs no longer resolve after role reassignment/drop. The failure appears during restore inside `pg_upgrade`, after `pg_upgrade --check` had declared the clusters compatible. BUG #19513 reproduces the same class of failure on a PostgreSQL 14 to 18 upgrade. A follow-up pgsql-hackers patch thread proposed checking invalid `pg_init_privs` role references and later revisions checked both grantor and grantee existence while preserving PUBLIC semantics.

The PostgreSQL discussion also considered all-digit role names. That matters because role-name text and unresolved raw OID are separate namespaces; converting an unresolved OID to decimal text permits identity confusion if an actual role has that numeric-looking name.

## ConceptWeave decision

The initial-privilege observation remains the canonical owner of the external recovery fact. It does not discard dangling ACL entries and does not convert unresolved OIDs into synthetic role names.

`IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` preserves PUBLIC or resolved/unresolved grantee identity, resolved/unresolved grantor identity, and exact grant option. OID zero is rejected for unresolved-role constructors because zero is PUBLIC only in the grantee namespace and is not a dangling role identity. Existing resolved-role digest framing remains stable. Unresolved grantees use a separate digest tag; unresolved grantors use a reserved framing sentinel before the raw OID so they cannot collide with resolved role-name framing.

`IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial` privately retains canonical sorted/deduplicated `unresolved_grantee_oids` and `unresolved_grantor_oids` in addition to the existing material digest. The material digest algorithm is unchanged and already commits every ACL identity, including raw dangling OIDs. The exact sets are therefore derived recovery evidence, not new digest identity. Aggregate count accessors are projections of those distinct canonical sets.

Grant, material, and recovery-validation routine `Debug` surfaces now all redact unresolved raw role OIDs. Grant-level formatting retains the grantee/grantor kind, resolved role names where present, and `grant_option`, but renders an unresolved role only as `UnresolvedRoleOid(<redacted>)`. Material and validation formatting retain only aggregate counts and existing digest/provenance. Exact raw OIDs remain available through explicit typed recovery accessors on non-forgeable validation evidence. PostgreSQL role OIDs are catalog identifiers rather than credentials, but this boundary avoids turning ordinary logs into a raw catalog dump.

ConceptWeave still does not repair PostgreSQL catalogs or own role lifecycle. Observation preserves the damaged source faithfully; validate/review can block publication and identify the exact missing role identities that require external remediation.

## RED → repair traceability

The first repair established representability of dangling ACL identities:

- finding review `5254640920` at #46 exact head `5daf2a5725ed52cd1b8d884200fc7786eaba54de`;
- structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332`;
- production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.

The second repair added deterministic aggregate diagnostics:

- finding review `5254787838` at exact head `8348be96316f625525b3d89abb732e952c0240e5`;
- structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`;
- production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe`.

Fresh review then found that counts plus an opaque digest were not actionable recovery evidence:

- finding review `5255447654` at exact pre-finding head `440d5e3f1cd12011047025c499aec76792c95c83`;
- structural RED `f23aefdcb8416c9828d2a46c12fcf3b6190fcd47`, requiring exact dangling-role OID access and equal-count/different-identity separation before production exposed it;
- material repair `b7b9e3d1b17a840436fbde7420f88a9c9539335f`, retaining sorted/deduplicated exact OID sets without changing digest framing;
- recovery-validation repair `57b5b209e9ade1f84c1c036a2bca81e7dacbac73`, carrying exact sets into non-forgeable receipt-bound evidence;
- log/rustdoc follow-up `a49243baddbc3142286de4c38c29bd5dd06cbdcb`;
- edge contract `dd8d5adf65638ae5c5e2f61686b37d14df17062c`, proving canonical order/deduplication and distinct equal-count identities.

Grant-level log-safety review then found that the public grant value still bypassed the intended redaction boundary:

- finding review `5255638361` at exact pre-finding head `c159955939ddcfa7b69419816414a67d7424e69e`;
- structural RED `3626a658d45b59802f632845708bf8ec378296cb`, proving `Debug` must not contain `16424` or `16425` while preserving unresolved-role kind and grant-option diagnostics;
- production causal repair `cfe4eba0e68eca6d4f4cf6b50f18478e61d14c23`, replacing unresolved-role derived `Debug` with redacted formatters while leaving equality, ordering, constructors, source identity, and digest framing unchanged.

The immediate pre-repair decision surface is archived as `docs/archive/CHANGELOG-through-c1599559.md` and `docs/archive/product-technical-gap-baseline-through-c1599559.md`.

Source repair is not exact-head GREEN. Repository-pinned Rust 1.98 fmt, strict Clippy, focused/retained/workspace/doc tests, release/rustdoc, owned production coverage, and the bounded PostgreSQL 18 live differential remain required after the final source/documentation head is known.

## Differential requirement

The PostgreSQL 18 differential must include at least one real or faithfully constructed catalog state in which `pg_init_privs.initprivs` contains a nonzero grantee or grantor OID with no row in `pg_roles`. Capture must preserve the raw OID and must not drop the ACL entry, fail source observation merely because role lookup fails, serialize the OID as role-name text, or resolve it by decimal-name coincidence.

The resulting recovery evidence must expose the exact canonical missing grantee/grantor OID sets through typed accessors, keep those values out of routine grant/material/validation `Debug`, and derive counts consistently from the distinct sets. It must include direct formatting of unresolved public-grantor, grantee-only, grantor-only, and both-unresolved grant values; none may render the numeric raw OID. It must also include two damaged baselines with equal counts but different OID identities and a case where repeated OIDs canonicalize deterministically. A resolved-only baseline must report empty sets and zero counts.

The same differential still has to bind the row to `classoid=pg_proc`, the exact converter function `objoid`, and `objsubid=0`, preserve `privtype`, the complete initial ACL, current function/ACL facts, converter extension lifecycle/security facts, immutable raw converter-root lineage, and transform-object extension membership from the same source generation.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h

PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org

PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org

Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com

Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com
