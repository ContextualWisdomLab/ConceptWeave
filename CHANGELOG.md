# Changelog

All notable ConceptWeave changes through exact `939172595c56a6a4bbd3ca7e9c63589a76930b3b` are preserved losslessly at `docs/archive/CHANGELOG-through-93917259.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-93917259.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds exact same-row `pg_proc.prosupport` planner-support state after the access-control predecessor. `prosupport = 0` is preserved as explicit absence; a nonzero support OID must be independently resolved in the same bounded source generation to a stable support-function signature.
- `IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot` requires complete unique `(constraint, key_position)` coverage and exact operator/target-function binding. Absence and distinct resolved support-function identities produce distinct domain-separated successor digests without rewriting predecessor domains.

### Correctness

- Planner support is no longer inferred from target-function language, volatility, cost, implementation definition, operator family, or other retained metadata. PostgreSQL planner support can simplify target calls and operator expressions and can provide selectivity, cost, rows, and index-condition planning information, so omitted `prosupport` could collapse planner-distinct catalog states.
- The focused fixture represents the documented planner support input signature `supportfn(internal)` instead of reusing the target operator function's two-`int4` signature. Synthetic identities remain digest/binding controls only and do not assert production existence or EXCLUDE admission.

### Retained

- The complete pre-planner-support decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-93917259.md` and `docs/archive/CHANGELOG-through-93917259.md`. Retained ordinary-EXCLUDE authority includes exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw `oprkind`; independently resolved commutator; exact `oprcode -> pg_proc`; Boolean result identity; scalar/set/strictness/volatility/parallel/kind/security/leakproof facts; implementation definition; exact owner; nullable `proconfig`; exact `proacl` and ACL semantics; backing-index namespace/lifecycle; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- The planner-support repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential.
- The live differential must read exact same-row target `pg_proc.prosupport` together with retained target-function facts. A nonzero support OID must resolve to the exact support `pg_proc` row in the same source generation; unresolved nonzero support is a capture failure rather than absence. All operator/backing-index controls remain bound to that same v3 source-content generation.
