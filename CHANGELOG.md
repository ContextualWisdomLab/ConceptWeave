# Changelog

All notable ConceptWeave changes through exact `e8a60dc4a85c8850eacecb3e88ff6b60e6a48700` are preserved losslessly at `docs/archive/CHANGELOG-through-e8a60dc4.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-e8a60dc4.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` implementation-function evidence now binds selected `pg_proc.protrftypes` entries to exact same-generation `pg_transform` rows after the transform-type predecessor.
- `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot` preserves exact target language plus optional FROM-SQL/TO-SQL converter directions for every selected transform type.
- Every nonzero transform-converter OID is resolved to an exact function coordinate, one `pg_catalog.internal` argument, direction-specific return type, converter implementation language, and a content digest over exact `prosrc`/optional `probin`/optional `prosqlbody` material. Plaintext converter bodies and binary paths are not retained.

### Correctness

- Equal target function signature, implementation, ACL, planner facts and `protrftypes` no longer imply equal transform runtime behavior. Mutable `(trftype, trflang) -> trffromsql/trftosql` identity is now a separate governed successor.
- NULL `protrftypes` cannot acquire unrelated transform rows; selected transform types require exact matching rows. Duplicate/missing rows, target-language drift, operator/target-function drift, invalid direction signatures, zero positions and unknown receipt coordinates fail closed.
- FROM-SQL and TO-SQL absence/presence remain distinct. A transform may provide only one direction; the source contract does not invent a policy requiring both.
- The initial converter constructor was refactored to explicit `TransformConverterFunctionDefinition` material rather than suppressing strict-Clippy `too_many_arguments` risk.

### Retained

- The complete pre-transform-converter decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-e8a60dc4.md` and `docs/archive/CHANGELOG-through-e8a60dc4.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity; ordered `conkey`/`conexclop`; operator kind/commutator/result; exact target `oprcode -> pg_proc`; target function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts; backing-index namespace/lifecycle/access-method/catalog controls; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- The transform-converter repair is source-shaped, not an executed GREEN claim. The current runtime does not provide repository-pinned Rust 1.98, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc and owned coverage remain unexecuted for the current exact head.
- The bounded PostgreSQL 18 live differential must resolve every selected transform type with the target function language to the exact same-generation `pg_transform` row, preserve zero/nonzero converter directions, and content-bind every nonzero converter function together with all retained target-function/operator/backing-index facts. Synthetic converter fixtures remain unit-test distinguishability controls only.
- Complete auxiliary converter-function `pg_proc` runtime/security identity remains a separately reviewed residual gap; it is not inferred from converter definition material.
