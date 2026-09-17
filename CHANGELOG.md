# Changelog

All notable ConceptWeave changes through exact `69a674329969977c25e9266d615bec36a2538bf6` are preserved losslessly at `docs/archive/CHANGELOG-through-69a67432.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-69a67432.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds exact same-row `pg_proc.procost` planner-cost state after the planner-support predecessor.
- `IndexExclusionConstraintOperatorProcedureCostSnapshot` requires complete unique `(constraint, key_position)` coverage, exact operator/target-function binding, and a positive finite PostgreSQL `float4` cost. The validated raw `f32` bits participate directly in the domain-separated successor digest.

### Correctness

- Planner cost is no longer inferred from function language, implementation definition, volatility, parallel safety, planner support, operator family, or any product performance heuristic. PostgreSQL allows `ALTER FUNCTION ... COST` without changing the function signature, so omitted `procost` could collapse planner-distinct catalog states.
- Decimal-string normalization is not used for `procost`; the successor preserves the exact validated float4 bit pattern. No ConceptWeave minimum, maximum, or preferred cost is invented beyond PostgreSQL's positive-cost contract.
- `pg_proc.prorows` is deliberately not added to this scalar-function successor because the retained predecessor requires `proretset=false` and PostgreSQL documents `prorows=0` in that case.

### Retained

- The complete pre-procost decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-69a67432.md` and `docs/archive/CHANGELOG-through-69a67432.md`. Retained ordinary-EXCLUDE authority includes exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw `oprkind`; independently resolved commutator; exact `oprcode -> pg_proc`; Boolean result identity; scalar/set/strictness/volatility/parallel/kind/security/leakproof facts; implementation definition; exact owner; nullable `proconfig`; exact `proacl` and ACL semantics; optional `prosupport`; backing-index namespace/lifecycle; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- The planner-cost repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential.
- The live differential must read exact same-row target `pg_proc.procost` together with retained target-function facts, including `prosupport`. All operator/backing-index controls remain bound to that same v3 source-content generation.
