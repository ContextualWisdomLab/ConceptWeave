# Changelog

All notable ConceptWeave changes through exact `cc64bab7d81f2adb691f627e7de852ace6946227` are preserved losslessly at `docs/archive/CHANGELOG-through-cc64bab7.md`; earlier release-era and Source Observation history remains in `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves raw `pg_proc.prosecdef` for every exact governed `pg_operator.oprcode` implementation function in `IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot`. The successor is layered over the exact `prokind='f'` predecessor, requires one independently observed security-context Boolean per exact constraint/key position, and binds the repeated stable operator and exact implementation function to that predecessor. `SECURITY INVOKER` (`false`) and `SECURITY DEFINER` (`true`) are both representable source states and are domain-separated rather than normalized. Missing/duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. This is observational evidence: ConceptWeave does not invent a PostgreSQL ordinary-EXCLUDE rule requiring one security mode.
- PostgreSQL ordinary `EXCLUDE` constraint evidence retains raw `pg_proc.prokind='f'`, `proparallel`, `provolatile`, `proisstrict`, scalar `proretset=false`, Boolean `prorettype`, and the exact `pg_operator.oprcode` binding in their existing domain-separated predecessors. Existing issued digest domains are not rewritten by the security-context successor.

### Security

- The governed ordinary-EXCLUDE function identity now distinguishes caller-privilege execution from owner-privilege execution. PostgreSQL 18.6 defines `SECURITY INVOKER` as execution with the calling user's privileges and `SECURITY DEFINER` as execution with the function owner's privileges, and `ALTER FUNCTION` can change the mode independently of the input-argument signature. Omitting raw `prosecdef` could therefore make materially different privilege boundaries converge on one semantic identity.

### Retained

- All ordinary-EXCLUDE source-integrity repairs archived through `docs/archive/product-technical-gap-baseline-through-cc64bab7.md` remain authoritative, including exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw binary `oprkind`; self-commutator; exact `oprcode -> pg_proc` implementation function; independent `oprresult`/`prorettype` with exact `pg_catalog.bool`; raw scalar `proretset=false`; raw `proisstrict`; raw `provolatile`; raw `proparallel`; raw `prokind='f'`; backing-index `pg_class.relnamespace`; ready/valid/live lifecycle; and exact v3 source-content generation binding.

### Acceptance

- The `prosecdef` repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass the repository-pinned Rust 1.98 native suite, the dedicated procedure-security-definer/procedure-kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus retained tests and coverage, applicable hosted gates, and the bounded PostgreSQL 18 live differential. That differential must independently read `pg_operator.oprkind`, `oprcom`, `oprresult`, `oprcode`, then exact `pg_proc.prokind`, `prosecdef`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel` from their source rows. `prosecdef` must be read from the exact joined `pg_proc` row and must not be inferred from owner, name, language, configuration, volatility, parallel safety, kind, or other auxiliary properties.