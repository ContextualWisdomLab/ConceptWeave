# PostgreSQL ordinary-EXCLUDE implementation-function local configuration integrity

## Decision

ConceptWeave must preserve the exact same-row `pg_proc.proconfig` identity for every `pg_operator.oprcode -> pg_proc` implementation function used by an ordinary PostgreSQL `EXCLUDE` constraint. The governed successor consumes the raw nullable `text[]` value in source order, distinguishes `NULL` from an empty array, and immediately reduces the array to a domain-separated SHA-256 digest. Raw run-time configuration values are not copied into downstream receipts.

This is an observational source-integrity contract. It does not impose a particular `search_path`, require `SECURITY DEFINER`, or infer function-local configuration from a caller session, reconstructed DDL, or current GUC values.

## Problem

The retained chain now binds the exact ordinary-EXCLUDE operator, exact `oprcode` function, scalar Boolean behavior, strictness, volatility, parallel safety, routine kind, security-definer mode, leakproofness, executable definition material, and exact function owner. It still omitted `pg_proc.proconfig`.

PostgreSQL 18 stores `proconfig` independently as a function's local run-time configuration settings. `ALTER FUNCTION ... SET`, `SET FROM CURRENT`, `RESET`, and `RESET ALL` can change those settings without changing the function's input identity. PostgreSQL's own `SECURITY DEFINER` guidance treats `search_path` as security material: writable schemas must be excluded and `pg_temp` should be placed last to avoid object-masking attacks. Two otherwise identical governed operator functions can therefore execute under materially different name-resolution or run-time settings while collapsing to one current evidence identity if `proconfig` is omitted.

## Constraints

- `pg_proc.proconfig` must be read from the exact same `pg_proc` row already joined from the governed `pg_operator.oprcode`, in the same v3 source-content generation.
- `NULL` and an empty array remain distinct source states.
- Array entry order and exact entry bytes are preserved in digest framing. This layer does not parse, sort, deduplicate, or normalize GUC assignments.
- Raw settings are reduced immediately to a dedicated digest and are not exposed through receipts. PostgreSQL permits custom configuration parameters, so treating every value as safe-to-publish configuration would be an unnecessary disclosure risk.
- Function-local configuration is observed for both SECURITY INVOKER and SECURITY DEFINER functions. `prosecdef` and `proowner` remain independent predecessor facts.
- A later governance rule may inspect a securely bounded source representation for requirements such as a safe SECURITY DEFINER `search_path`; this source-identity layer does not invent that policy.
- No predecessor digest domain is rewritten.

## Alternatives

### Ignore `proconfig` because the executable body is already hashed

Rejected. Function-local GUC state changes execution context independently from function body and stable signature. PostgreSQL allows `ALTER FUNCTION ... SET/RESET` without changing either.

### Store only a normalized map of configuration assignments

Rejected for this increment. PostgreSQL exposes an ordered nullable `text[]` catalog fact. Normalizing it would silently assert equivalence rules that are not required to close the source-identity gap and could erase exact-source differences.

### Copy raw GUC values into receipts

Rejected. Custom GUCs can contain deployment-specific or sensitive material. Downstream provenance needs a stable source identity, not arbitrary plaintext configuration. The raw array is consumed only at the source boundary and reduced to a digest.

### Require a fixed secure `search_path`

Rejected at this layer. PostgreSQL strongly recommends a safe path for SECURITY DEFINER functions, but the source observer should first preserve the independent catalog fact. Admission/policy evaluation belongs in a separate governed validation decision with the exact security context available.

## Implementation and invariants

`IndexExclusionConstraintOperatorProcedureConfigurationMaterial::from_proconfig` consumes `Option<Vec<String>>`, frames nullable presence, ordered entry count, and exact entry bytes, and exposes only `is_configured`, `entry_count`, and a domain-separated SHA-256 digest.

`IndexExclusionConstraintOperatorProcedureConfigurationObservation` binds one exact ordinary-EXCLUDE coordinate/key position to the repeated stable operator and exact `oprcode` function plus the privacy-preserving configuration material.

`IndexExclusionConstraintOperatorProcedureConfigurationSnapshot` derives the complete coordinate inventory from `IndexExclusionConstraintOperatorProcedureOwnerSnapshot`, rejects missing or duplicate coordinates and operator/function binding drift, and frames the owner-predecessor digest plus exact configuration-material digests under a new successor domain. A change from NULL to an empty array, a change to any entry, or a source-order change produces a different successor identity.

The focused contract covers provenance, NULL-versus-empty distinction, `search_path` change distinguishability, array-order distinguishability, operator/function drift, missing/duplicate evidence, zero positions, unknown receipt coordinates, and public composition. These remain source-level contracts until one unchanged exact head receives repository-pinned Rust 1.98 native and hosted GREEN evidence.

## Traceability

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5231254550`
- Structural source/compile RED: `f5baf631e9add518322cbab8db471acdf4fe1d4f`
- Production successor: `4c7117fa7587f6c85e0588f00a76c1a4e1dbb117`
- Public composition: `0354ecf1c003a021b1401932cbc82f74bd5cbdf4`
- Lint-neutral production refinement: `f3995fd2b0484d2c06e2871caab0c930d3109426`
- Production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_configuration.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_configuration_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureOwnerSnapshot`
- Exact catalog fact: `pg_proc.proconfig`

## References

PostgreSQL Global Development Group. (2026). *ALTER FUNCTION (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *CREATE FUNCTION (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *pg_proc (PostgreSQL 18 documentation).* https://www.postgresql.org/docs/18/catalog-pg-proc.html
