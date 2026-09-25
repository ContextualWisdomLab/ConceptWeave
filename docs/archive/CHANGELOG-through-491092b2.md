# Changelog

All notable ConceptWeave changes through exact `4ed375df5ba0e8a1d248d67a1b9f66c22f954dc9` are preserved losslessly at `docs/archive/CHANGELOG-through-4ed375df.md`; earlier release-era and Source Observation history remains in `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds exact same-row `pg_proc.proconfig` identity after the exact function-owner predecessor. `IndexExclusionConstraintOperatorProcedureConfigurationMaterial` preserves nullable-array presence, ordered entry count, and exact entry bytes in domain-separated SHA-256 framing while withholding raw arbitrary/custom GUC values from receipts.
- `IndexExclusionConstraintOperatorProcedureConfigurationSnapshot` requires one exact configuration observation for every governed ordinary-EXCLUDE operator/function coordinate, rejects missing/duplicate coordinates and operator/function binding drift, and keeps `NULL`, an empty array, changed GUC assignments, and changed catalog-array order as distinct successor identities.

### Security

- Function-local configuration is no longer inferred from caller session state or reconstructed DDL. The bounded source differential must read exact same-row `pg_proc.proconfig`. This matters especially for `SECURITY DEFINER`, for which PostgreSQL documents a function-local safe `search_path` as protection against object-masking attacks.
- Raw `proconfig` entries are consumed only at the source boundary and immediately reduced to a digest, avoiding unnecessary propagation of deployment-specific or sensitive custom GUC values. This source-identity layer does not itself impose a `search_path` admission rule.

### Retained

- The complete pre-configuration decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-4ed375df.md` and `docs/archive/CHANGELOG-through-4ed375df.md`. Retained ordinary-EXCLUDE authority includes exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw binary `oprkind`; self-commutator; exact `oprcode -> pg_proc`; independent Boolean `oprresult`/`prorettype`; scalar `proretset=false`; raw `proisstrict`, `provolatile`, `proparallel`, `prokind='f'`, `prosecdef`, `proleakproof`; implementation-definition identity; exact `proowner` plus `pg_roles` resolution; backing-index namespace/lifecycle; and exact v3 source-content-generation binding.

### Acceptance

- The function-configuration repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential. That differential must independently read exact `pg_operator` facts and exact same-row `pg_proc.proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, and `proconfig`; resolve owner and language in the same generation; and keep all retained backing-index/operator-family controls in that same v3 source-content generation.
