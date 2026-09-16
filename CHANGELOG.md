# Changelog

All notable ConceptWeave changes through the operator-kind predecessor decision surface are preserved losslessly at `docs/archive/CHANGELOG-through-c9777662.md`; the earlier release-era changelog through exact `08847df6124f8834ed8b8ec33c9451a2a1474d3e` remains at `docs/archive/CHANGELOG-through-08847df6.md`. This active changelog records the current Source Observation delta without deleting the archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves raw `pg_proc.proretset` for every exact governed `pg_operator.oprcode` implementation procedure in `IndexExclusionConstraintOperatorProcedureScalarSnapshot`. The successor rebinds the raw operator-kind predecessor to the supplied result-contract predecessor, requires one independently observed scalar-cardinality fact per exact constraint/key position, binds the repeated stable operator and implementation procedure to the exact predecessor, and admits only `proretset=false`. Set-returning state, missing/duplicate evidence, operator/procedure binding drift, zero position, cross-predecessor mismatch, and unknown receipt coordinates fail closed. The raw flag enters a new domain-separated digest/provenance family; existing operator, commutator, procedure, result, kind, and earlier Source Observation digest domains remain unchanged.
- PostgreSQL ordinary `EXCLUDE` constraint evidence retains the raw `pg_operator.oprkind` discriminator for every exact governed `conexclop` key position in `IndexExclusionConstraintOperatorKindSnapshot`. The successor is layered over the exact Boolean result-contract predecessor, requires one independently observed raw kind per exact position, binds the repeated stable operator to the predecessor, and admits only PostgreSQL's binary `b` state. Prefix `l`, unknown kind, missing/duplicate position, zero position, operator-binding drift, and unknown receipt coordinates fail closed. The raw discriminator is never inferred from normalized left/right operand types, operator name, commutator state, operator-family membership, strategy, or implementation procedure.

### Retained

- All ordinary-EXCLUDE source-integrity repairs archived through `c9777662921be7c8d19c3c7844178105d6356d4e` remain authoritative, including exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw binary `oprkind`; self-commutator; exact `oprcode -> pg_proc` implementation function; independent `oprresult`/`prorettype` with exact `pg_catalog.bool`; backing-index `pg_class.relnamespace`; ready/valid/live lifecycle; and exact v3 source-content generation binding.

### Acceptance

- The raw procedure-cardinality and operator-kind repairs are source-complete but not an executed GREEN claim. One unchanged exact head must still pass the repository-pinned Rust 1.98 native suite, the dedicated procedure-scalar/kind/result/procedure/commutator contracts plus retained tests and coverage, applicable hosted gates, and the bounded PostgreSQL 18 live differential that independently reads `oprkind`, `oprcode`, exact `pg_proc.prorettype`, and raw `pg_proc.proretset` from their source rows. Ordinary EXCLUDE enforcement requires `proretset=false`; that fact must not be inferred from Boolean result type or stable procedure identity.
