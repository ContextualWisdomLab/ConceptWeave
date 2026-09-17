# Changelog

All notable ConceptWeave changes through exact `40e137925e46c5f4c758fc671ba699b59c78515e` are preserved losslessly at `docs/archive/CHANGELOG-through-40e13792.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-40e13792.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence now preserves every nonzero converter function's independently observed `pg_proc.proowner` after exact `pg_transform` converter identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity` groups raw nonzero owner OID and same-generation resolved role name as one validated value object.
- `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot` binds each exact `(constraint, key_position, transform_type, direction)` to the predecessor converter coordinate and validated owner identity.
- Converter owner provenance has an exact source receipt and a new domain-separated successor digest; the transform-converter predecessor remains immutable.

### Correctness

- Equal converter function coordinate, signature, implementation language and source body no longer imply equal controlling-principal identity. `ALTER FUNCTION ... OWNER TO` can change owner without redefining those predecessor facts.
- Missing/extra owner directions, duplicate coordinates, zero owner OIDs, blank role/function identifiers, converter-function binding drift, zero positions, and unknown receipt coordinates fail closed.
- Owner is not inferred from schema ownership, transform creation, target-function ownership, session identity, ACL, or `SECURITY DEFINER` mode.
- The receipt location is canonicalized without a duplicate path separator, and the unknown-coordinate fixture now actually exercises a failing receipt lookup.
- Review `5233923341` rejected the initial eight-argument owner-observation constructor instead of suppressing strict Clippy. The repair uses the owner value object and a private typed coordinate key rather than an eight-element tuple; no lint waiver was added.

### Retained

- The complete pre-converter-owner decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-40e13792.md` and `docs/archive/CHANGELOG-through-40e13792.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity; ordered `conkey`/`conexclop`; operator kind/commutator/result; exact target `oprcode -> pg_proc`; target function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts; exact selected `pg_transform` rows and converter implementation identity; backing-index namespace/lifecycle/access-method/catalog controls; operator-family/strategy; and exact v3 source-content-generation binding.

### Acceptance

- This converter-owner repair is source-shaped, not an executed GREEN claim. The current runtime does not provide repository-pinned Rust 1.98, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc and owned coverage remain unexecuted for the current exact head.
- The bounded PostgreSQL 18 live differential must resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, preserve signature/definition evidence, and independently read raw `proowner` plus role-name resolution together with all retained target-function/operator/backing-index facts.
- Converter ACL, configuration, security mode, strictness, volatility, parallel safety and planner auxiliary facts remain separately reviewable residual surfaces; they are not inferred from owner evidence.
