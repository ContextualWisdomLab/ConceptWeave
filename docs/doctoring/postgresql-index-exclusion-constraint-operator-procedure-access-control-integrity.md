# PostgreSQL ordinary-EXCLUDE implementation-function access-control integrity

## Decision

ConceptWeave must preserve the exact same-row `pg_proc.proacl` encoding state and the effective function `EXECUTE` grant set for every `pg_operator.oprcode -> pg_proc` implementation function used by an ordinary PostgreSQL `EXCLUDE` constraint. The source adapter materializes effective grants with PostgreSQL ACL semantics, resolves grantee/grantor role identities in the same source generation, and immediately reduces the role-bearing grant set to a domain-separated SHA-256 digest. Downstream receipts expose only whether `proacl` was NULL, the effective grant count, and the digest.

This is an observational source-integrity contract. It does not require PUBLIC execution, prohibit PUBLIC execution, or define a new role-admission policy. It preserves the authorization fact so later governed validation can make a policy decision without reconstructing access control from unrelated state.

## Problem

The retained ordinary-EXCLUDE chain binds exact operator identity, `oprcode` function identity, scalar Boolean behavior, strictness, volatility, parallel safety, routine kind, security-definer mode, leakproofness, implementation definition, exact owner, and function-local configuration. It still omitted `pg_proc.proacl`.

PostgreSQL 18 stores routine access privileges in `pg_proc.proacl`. PostgreSQL documents `EXECUTE` as the only privilege applicable to functions and procedures and states that it permits function calls, including use of operators implemented on top of the function. `GRANT`/`REVOKE EXECUTE` can therefore change who may invoke an implementation function/operator without changing the function input signature or the catalog facts already captured by the predecessor chain.

A `NULL` `proacl` is also distinct catalog state: PostgreSQL applies default function privileges, including PUBLIC `EXECUTE` unless defaults were changed for object creation. Effective privileges and exact catalog encoding therefore both matter for provenance.

## Constraints

- `pg_proc.proacl` must come from the exact same `pg_proc` row reached through the governed `pg_operator.oprcode`, in the same v3 source-content generation.
- The adapter must preserve whether the raw `proacl` value was NULL.
- Effective function privileges must be expanded with PostgreSQL ACL semantics. For a NULL ACL, use the function default ACL for the exact owner rather than treating NULL as “no grants”.
- Only effective `EXECUTE` grants enter this function-specific material. The source adapter resolves grantee/grantor OIDs in the same generation; grantee OID zero is represented as PUBLIC.
- ACL array order is not authorization semantics. Effective grants are canonicalized as a set; duplicate effective grant evidence fails closed.
- Grantee role, grantor role, and grant option all affect the material digest.
- Role-bearing ACL plaintext is consumed only at the source boundary and is not propagated through receipts.
- `proowner`, `prosecdef`, `proconfig`, implementation definition, and function ACL remain independent successor facts.
- No predecessor digest domain is rewritten.

## Alternatives

### Ignore `proacl` because operator creation already required EXECUTE

Rejected. Creation-time authorization does not freeze later function privileges. PostgreSQL exposes function ACLs as mutable object privileges, and use of an operator implemented by the function is covered by `EXECUTE`.

### Hash the textual `aclitem[]` array in source order

Rejected. ACL array ordering is not privilege meaning and would manufacture semantic drift from set reordering. The governed material instead canonicalizes effective EXECUTE grants after same-generation role resolution while separately preserving NULL-versus-explicit catalog state.

### Persist role names and ACL entries in receipts

Rejected. Downstream provenance needs authorization identity, not a copied role directory or deploy-specific ACL listing. Exact role-bearing material is reduced immediately to a digest.

### Treat NULL `proacl` as an empty grant set

Rejected. PostgreSQL grants default privileges to newly created functions, including PUBLIC `EXECUTE` under the normal defaults. NULL must be interpreted with PostgreSQL function-default ACL semantics and separately preserved as catalog encoding state.

## Implementation and invariants

`IndexExclusionConstraintOperatorProcedureExecuteGrant` represents one effective `EXECUTE` grant to PUBLIC or a resolved role, with the exact resolved grantor role and grant-option bit.

`IndexExclusionConstraintOperatorProcedureAccessControlMaterial::new` preserves `proacl_was_null`, canonicalizes the effective grant set, rejects duplicate grants, and hashes grantee kind/name, grantor identity, and grant option under a dedicated domain. Raw role names do not survive into the material object.

`IndexExclusionConstraintOperatorProcedureAccessControlObservation` binds one exact ordinary-EXCLUDE coordinate/key position to the repeated stable operator and exact `oprcode` function plus privacy-preserving ACL material.

`IndexExclusionConstraintOperatorProcedureAccessControlSnapshot` derives the complete coordinate inventory from `IndexExclusionConstraintOperatorProcedureConfigurationSnapshot`, rejects missing/duplicate coordinates and operator/function binding drift, and frames the configuration-predecessor digest plus exact access-control material digests under a new successor domain.

The focused contract covers default-ACL provenance, NULL-versus-explicit-equivalent distinction, order-invariant canonicalization, grantee/grantor/grant-option distinguishability, duplicate-grant rejection, blank role rejection, operator/function drift, missing/duplicate observation evidence, zero positions, unknown receipt coordinates, and public composition.

## Traceability

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5231368167`
- Structural source/compile RED: `52a533a6a2979e1ae884830dc5084b2a8dfd963c`
- Production successor: `edac52866c4a8f8055eb018b4c6782a4b6661feb`
- Public composition: `f50040d6fd7e50856a2b3ff032dc0fe7f03d9e59`
- Production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_access_control.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_access_control_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureConfigurationSnapshot`
- Exact catalog fact: `pg_proc.proacl`

## References

National Institute of Standards and Technology. (2020). *Security and privacy controls for information systems and organizations (NIST SP 800-53 Rev. 5), AC-3 Access Enforcement and AC-6 Least Privilege.* https://doi.org/10.6028/NIST.SP.800-53r5

PostgreSQL Global Development Group. (2026). *Privileges (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/ddl-priv.html

PostgreSQL Global Development Group. (2026). *ALTER DEFAULT PRIVILEGES (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/sql-alterdefaultprivileges.html

PostgreSQL Global Development Group. (2026). *pg_proc (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/catalog-pg-proc.html
