# PostgreSQL transform-converter initial-privilege integrity

## Decision

ConceptWeave Source Observation preserves converter-function `pg_init_privs` independently from current `pg_proc.proacl`, extension membership/dependency, security labels, function definition, and `pg_transform` binding.

The admitted distinction is recovery semantics, not catalog enumeration. PostgreSQL records non-default initial privileges in `pg_init_privs`; `pg_dump` uses that baseline when reconstructing extension-object privilege state. The observed ACL is therefore source evidence rather than a name-only authorization summary.

PostgreSQL 18 `AclItem` stores `ai_grantee` and `ai_grantor` as OIDs. `pg_authid` stores role OID and `rolname` as separate attributes. A role can be dropped and recreated under the same name with a different OID, so a resolved role name alone is not lossless ACLITEM identity. ConceptWeave retains the same-generation resolved OID and role name together. Numeric OIDs remain private identity material; routine `Debug` and ordinary semantic accessors remain name-oriented and do not disclose them.

PostgreSQL also makes ACL array order recovery-significant. Client utilities such as `pg_dump` can need a grant-option provider before a dependent grant. `check_acl()` validates ACL array shape/nullability but does not impose uniqueness, and `aclnewowner()` contains duplicate-entry reconciliation for states that can arise during owner substitution. Source Observation therefore preserves exact array order and multiplicity instead of sorting, merging, or rejecting repeated ACLITEMs.

## Owner and bounded context

ConceptWeave owns observation of this PostgreSQL source fact because it changes immutable semantic evidence used by `observe -> discover -> propose -> align -> validate -> review -> publish`. It does not own role lifecycle, extension package files, enterprise authorization policy, PostgreSQL catalog repair, or restoration orchestration.

Current `pg_proc.proacl` and `pg_init_privs.initprivs` remain distinct facts. The former answers current object authorization; the latter is PostgreSQL's stored initial privilege baseline used for recovery/dump semantics.

## PostgreSQL source contract

For every exact FROM SQL / TO SQL converter function selected from one source generation, extraction uses the function object identity with `classoid = pg_proc`, the exact selected function `objoid`, and `objsubid = 0`.

Row absence remains distinct from present material. A present row preserves exact `privtype` and complete object-level EXECUTE ACL. Each ACL entry preserves:

- exact source-array position and multiplicity;
- PUBLIC versus role grantee shape;
- for a resolved grantee, the exact same-generation role OID plus readable role name;
- for a resolved grantor, the exact same-generation role OID plus readable role name;
- for a dangling nonzero role reference, the raw unresolved OID as recovery evidence;
- exact grant-option state.

OID zero is PUBLIC only in the grantee position and is rejected anywhere the domain requires a role identity. Resolved role OIDs are never synthesized from names. Unresolved OIDs are not stringified as names.

The material digest uses the versioned `.material.v2` domain. Resolved grantee/grantor identities commit both OID and role name, while PUBLIC and unresolved variants remain explicitly tagged. The outer snapshot digest continues to commit the versioned material digest. A drop/recreate event that preserves `rolname` but changes OID therefore changes material and snapshot identity.

Routine grant/material `Debug` formatting does not render numeric resolved or unresolved role OIDs. Readable role names, PUBLIC/unresolved shape, grant option, counts, and immutable digest remain available. Exact dangling OIDs are exposed only through receipt-bound recovery-validation evidence.

## Implementation traceability

The original `pg_init_privs` fact was admitted under review `5254405649`. Later repairs established dangling-role representability, non-forgeable receipt-bound validation, exact dangling-role remediation sets, routine Debug redaction, readable ACL semantics, source-order preservation, and source multiplicity.

Source-order finding review `5255912747` at pre-finding `4967f12cb90ac41ec42263bfed47ba6bcbb49735` led to structural RED `13e51561cac125f2a942e7ca0d0dd39433040426` and production repair `966e2136c15aa049f72acfebf2c1a478d09c66ef` removing ACL sorting.

ACL-multiplicity finding review `5256059436` at pre-finding `50fff94eb567fd9b1c2150d4570ca1422acc9a60` led to corrected structural RED `9747763c4b2370185bd18039b00e8ceaa9df206c` and production repair `55f828f6f6065fac89ae63f521d72d807f875c5b` removing source-level duplicate rejection while retaining canonical derived dangling-role sets.

Resolved-role OID identity finding review `5256559272` at pre-finding `a82dced2a154e1ff105864309a74559255898bbc` found that resolved ACL entries were still reduced to role names. PostgreSQL `AclItem` is OID-based and `pg_authid` keeps OID and `rolname` separately, so equal names with different OIDs could alias in the material digest. Structural RED `b1c8fef6a526b22566b65d8f7962c3af0ebd2031` requires equal-name/different-OID grantee and grantor identities to produce different digests while keeping routine diagnostics OID-redacted. It is a source-level structural RED, not an executed failing CI result.

Production repair `52e1c12d5e592e624d8d55badbdc24c52b428a00` retains exact resolved OIDs privately alongside names, validates nonzero resolved role OIDs, introduces the `.material.v2` digest domain, and commits both OID and name for resolved grantee/grantor identities. Test adoption `2780f5edb6c086a3d13b0dc47d6ca4b22963eec9`, dangling-role adaptation `a7274b7de87f904b8aae7fbdf0208ed8b3066d36`, and recovery-fixture adaptation `8ba4e304fa4b24cb4e49ae31474985b400aeddc7` move retained contracts to the exact-OID constructors without exposing OIDs through routine diagnostics.

No predecessor Check or approval transfers across these head movements.

## Invariants

1. Every converter direction in the predecessor has exactly one initial-privilege observation, including explicit row absence.
2. Converter coordinate, transform type, direction, schema, and function name bind exactly to the predecessor.
3. `converter_snapshot_digest` retains immutable raw transform-converter lineage.
4. `privtype='i'` and `privtype='e'` remain distinct.
5. Source ACL order and multiplicity remain exact.
6. PUBLIC, resolved role, and unresolved role identities remain separate namespaces.
7. Resolved role identity is `(role_oid, role_name)` from the same source generation; neither component may substitute for the other.
8. Zero resolved/unresolved role OIDs fail closed wherever a role identity is required.
9. Routine diagnostics never render numeric role OIDs.
10. Derived dangling-role remediation sets may sort/deduplicate OIDs but never rewrite the source ACL.
11. Receipt-bound recovery validation remains the only public surface that exposes exact dangling OIDs for remediation.

## Alternatives considered

### Preserve resolved role names only

Rejected. PostgreSQL ACLITEM identity is OID-based. A role recreated under the same name is a distinct catalog identity, and name-only hashing aliases those source states.

### Preserve only role OIDs

Rejected. Source Observation also needs readable semantic identity and same-generation lookup evidence. The pair `(OID, name)` preserves both exact catalog identity and resolved meaning without forcing downstream callers to perform mutable name lookup.

### Expose resolved OIDs through ordinary accessors

Rejected. The OID is required for immutable identity, not routine product semantics. Keeping it private avoids accidental logging/API coupling while the digest proves exact identity.

### Sort or deduplicate ACL entries

Rejected. PostgreSQL documents ACL order relevance to client utilities, and its validator does not define uniqueness as an ACL invariant. Observation must not normalize source state.

### Reuse current `proacl`

Rejected. Current ACL and stored initial baseline are separate PostgreSQL facts and can diverge.

## Acceptance boundary

The current source repair is not accepted until the exact final head has repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and a bounded PostgreSQL 18 live differential.

The differential must prove at minimum:

- row absence versus present material and exact `privtype`;
- source-order and multiplicity round-trip;
- PUBLIC, resolved grantee/grantor names and grant option readability;
- equal role names with distinct same-generation OIDs yield distinct material/source identity;
- controlled drop/recreate or equivalent fixture demonstrates OID/name non-aliasing for grantee and grantor;
- dangling grantee/grantor identities remain actionable through recovery validation without routine Debug leakage;
- repeated dangling OIDs canonicalize only in the derived remediation sets;
- immutable raw converter-root and downstream transform-object lineage remain intact.

Synthetic Rust fixtures are unit-contract evidence only and do not substitute for the PostgreSQL 18 differential.

## Primary sources

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: ACL data structures (`src/include/utils/acl.h`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/utils/acl.h

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 source: role catalog (`src/include/catalog/pg_authid.h`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_authid.h

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 source: ACL validation, update, and owner-change behavior (`src/backend/utils/adt/acl.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/adt/acl.c

PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 source: pg_dump initial-privilege capture (`src/bin/pg_dump/pg_dump.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/bin/pg_dump/pg_dump.c

PostgreSQL Global Development Group. (2026f). *PostgreSQL 18 documentation: GRANT*. https://www.postgresql.org/docs/18/sql-grant.html
