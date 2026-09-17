# Changelog

All notable ConceptWeave changes through exact `491092b20a2ccca4ae3bb8ce6bffcf07eceae838` are preserved losslessly at `docs/archive/CHANGELOG-through-491092b2.md`; earlier release-era and Source Observation history remains in `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds exact same-row `pg_proc.proacl` access-control state after the function-local configuration predecessor. The source boundary preserves whether `proacl` was `NULL`, materializes the function-default ACL when needed, canonicalizes exact function `EXECUTE` ACL entries, and resolves PUBLIC/role grantees and grantors in the same source generation.
- `IndexExclusionConstraintOperatorProcedureAccessControlMaterial` reduces role-bearing ACL material immediately to a domain-separated SHA-256 digest while retaining NULL-vs-explicit state and grant count. `IndexExclusionConstraintOperatorProcedureAccessControlSnapshot` requires complete unique coordinate coverage and exact operator/function binding.

### Security

- Function invocation authority is no longer inferred from owner, `SECURITY DEFINER`, implementation definition, local GUC configuration, or current-session privilege checks. PostgreSQL function `EXECUTE` privilege also governs use of operators implemented by that function, so `GRANT`/`REVOKE` changes now produce a distinct governed source identity.
- ACL array order is treated as non-semantic: exact ACL entries are canonicalized as a set before hashing, while grantee, grantor and grant-option changes remain distinct. Role-bearing ACL plaintext is not propagated through downstream receipts. This source layer records ACL identity and does not invent a PUBLIC/role admission policy or role-membership closure.

### Retained

- The complete pre-access-control decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-491092b2.md` and `docs/archive/CHANGELOG-through-491092b2.md`. Retained ordinary-EXCLUDE authority includes exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw `oprkind`; independently resolved commutator; exact `oprcode -> pg_proc`; Boolean result identity; scalar/set/strictness/volatility/parallel/kind/security/leakproof facts; implementation definition; exact owner; exact nullable `proconfig`; backing-index namespace/lifecycle; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- The function-access-control repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential.
- The live differential must read exact same-row `pg_proc.proacl` together with retained `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, and `proconfig`; preserve NULL-default ACL semantics; resolve ACL grantee/grantor role identities in the same generation; and retain all operator/backing-index controls in that same v3 source-content generation.
