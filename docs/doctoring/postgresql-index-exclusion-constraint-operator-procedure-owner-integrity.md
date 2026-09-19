# PostgreSQL ordinary-EXCLUDE implementation-function owner integrity

## Decision

ConceptWeave must preserve the exact owner of every `pg_operator.oprcode -> pg_proc` implementation function used by an ordinary PostgreSQL `EXCLUDE` constraint. The governed successor records raw nonzero `pg_proc.proowner` together with the exact role name independently resolved through `pg_roles.oid -> rolname`, while retaining the already-governed operator/function binding and implementation-definition predecessor.

This is an observational source-integrity contract. It does not require `SECURITY DEFINER`, does not require a particular role, and does not infer ownership from the containing schema, session user, function name, or definition text.

## Problem

The predecessor chain binds the exact ordinary-EXCLUDE operator, the exact `oprcode` function, Boolean result/cardinality, strictness, volatility, parallel safety, routine kind, security-definer mode, leakproofness, and executable definition material. It still omits `pg_proc.proowner`.

PostgreSQL stores function ownership independently in `pg_proc.proowner`, referencing `pg_authid.oid`. `ALTER FUNCTION ... OWNER TO` can change that owner without changing the function's input-argument identity. PostgreSQL explicitly states that when the function is `SECURITY DEFINER`, it subsequently executes as the new owner. Two otherwise identical governed function definitions can therefore have different execution principals while collapsing to one current evidence identity if owner state is omitted.

The publicly readable `pg_roles` view exposes both `oid` and `rolname`, so the bounded source differential can resolve the exact `proowner` OID without reading password material from `pg_authid`. Binding both the raw OID and resolved name prevents a role rename from being silently ignored and prevents a dropped/recreated same-name role from collapsing to the same source-generation owner identity.

## Constraints

- `pg_proc.proowner` must be read from the exact same `pg_proc` row already joined from the governed `pg_operator.oprcode`.
- The observed owner OID must be nonzero and must resolve exactly once through `pg_roles.oid` to a nonblank `rolname` in the same bounded source generation.
- Raw owner OID and resolved role name are both evidence. Neither may be synthesized from schema ownership, routine namespace, session identity, or caller configuration.
- Owner evidence is collected for both SECURITY INVOKER and SECURITY DEFINER functions. `prosecdef` remains an independent predecessor fact; this successor does not invent a new admission rule.
- Role privilege closure, membership, and role-local configuration remain separate catalog concerns. This increment binds the exact implementation-function owner only and does not claim that owner identity alone fully describes the role's effective privileges.
- No predecessor digest domain is rewritten.

## Alternatives

### Preserve only `rolname`

Rejected. PostgreSQL role names can be renamed, and a role can be dropped and a new role created with the same name. The raw OID is the exact source-generation principal coordinate referenced by `pg_proc.proowner`; preserving it alongside the human-readable role name avoids collapsing those distinct catalog states.

### Preserve only raw `proowner` OID

Rejected. OIDs are source-local identifiers and are not meaningful to downstream reviewers without resolution. The public `pg_roles` view exists specifically to expose role identity, including the underlying OID needed for catalog joins, without exposing passwords.

### Require SECURITY INVOKER or a fixed owner

Rejected. That would impose a product policy not established by PostgreSQL ordinary-EXCLUDE semantics. ConceptWeave should preserve the independent fact and leave policy evaluation to the appropriate governance layer.

## Implementation and invariants

`IndexExclusionConstraintOperatorProcedureOwnerObservation` binds one exact ordinary-EXCLUDE coordinate/key position to the repeated stable operator and exact `oprcode` function plus raw `owner_oid` and resolved `owner_role_name`. Zero OID, blank role name, and zero key position fail closed.

`IndexExclusionConstraintOperatorProcedureOwnerSnapshot` derives its complete coordinate inventory from `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`, rejects missing or duplicate coordinates and operator/function binding drift, and frames the predecessor digest plus exact owner observations under a new domain-separated SHA-256 successor identity. Changes to either raw owner OID or resolved role name change the successor digest.

The focused contract covers provenance, raw-OID and role-name digest separation, zero/blank owner rejection, operator/function binding drift, missing/duplicate evidence, zero positions, unknown receipt coordinates, and public composition. These are source-level contracts until one unchanged exact head receives repository-pinned Rust 1.98 native and hosted GREEN evidence.

## Traceability

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5231157230`
- Structural source/compile RED: `0ebd61cb44957d5d5278c9d70c7bcf63ec7bd94c`
- Production successor: `39382ad29a35b428c8b431a46b4071aa202603ec`
- Public composition: `51d8d43af1deb230fa475f211c87ccd890bdbe68`
- Production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_owner.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_owner_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`
- Catalog facts: `pg_proc.proowner`, resolved `pg_roles.oid`, `pg_roles.rolname`

## References

PostgreSQL Global Development Group. (2026). *ALTER FUNCTION (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *CREATE FUNCTION (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *pg_proc (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *pg_roles (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/view-pg-roles.html
