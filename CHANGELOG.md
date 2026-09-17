# Changelog

All notable ConceptWeave changes through exact `781f8d57c83c722596fd5f3759f54dc6a001a803` are preserved losslessly at `docs/archive/CHANGELOG-through-781f8d57.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-781f8d57.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds exact same-row nullable `pg_proc.protrftypes` transform selection after the planner-cost predecessor.
- `IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot` requires complete unique `(constraint, key_position)` coverage, exact operator/target-function binding, and either PostgreSQL's documented NULL state or a nonempty same-generation resolved qualified transform-type set.

### Correctness

- Transform selection is no longer inferred from target signature, `prolang`, implementation definition, ACL, planner support, planner cost, or the existence of `pg_transform` rows. PostgreSQL records routine transform selection independently in `protrftypes`.
- Non-null transform types are canonicalized as a set by qualified type identity; ordering does not affect the successor digest, while duplicate input and an explicit empty array fail closed. NULL remains distinct from selected transform types.
- This successor does not claim exact transform-converter identity. `pg_transform.trffromsql` and `pg_transform.trftosql` remain a separate material catalog gap for selected `(type, language)` pairs.

### Retained

- The complete pre-transform-type decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-781f8d57.md` and `docs/archive/CHANGELOG-through-781f8d57.md`. Retained ordinary-EXCLUDE authority includes exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw `oprkind`; independently resolved commutator; exact `oprcode -> pg_proc`; Boolean result identity; scalar/set/strictness/volatility/parallel/kind/security/leakproof facts; implementation definition; exact owner; nullable `proconfig`; exact `proacl` and ACL semantics; optional `prosupport`; exact positive-finite `procost`; backing-index namespace/lifecycle; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- The transform-type repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential.
- The live differential must read exact same-row target `pg_proc.protrftypes`, preserve NULL, and resolve each selected type OID to the same-generation qualified type identity together with all retained target-function facts. Exact `pg_transform` converter binding is not implied and remains a separately reviewed next gap.
