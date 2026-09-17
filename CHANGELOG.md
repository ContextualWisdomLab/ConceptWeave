# Changelog

All notable ConceptWeave changes through exact `e2284292ed83c0c957bbcdc5745ca15585f56438` are preserved losslessly at `docs/archive/CHANGELOG-through-e2284292.md`; earlier release-era and Source Observation history remains in `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves raw `pg_proc.proleakproof` for every exact governed `pg_operator.oprcode` implementation function in `IndexExclusionConstraintOperatorProcedureLeakproofSnapshot`. The successor is layered over the exact procedure-security-context predecessor, requires one independently observed leakproofness Boolean per exact constraint/key position, and binds the repeated stable operator and exact implementation function to that predecessor. `NOT LEAKPROOF` (`false`) and `LEAKPROOF` (`true`) are both representable source states and are domain-separated rather than normalized. Missing/duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. This is observational evidence: ConceptWeave does not invent a PostgreSQL ordinary-EXCLUDE rule requiring one leakproofness state.
- PostgreSQL ordinary `EXCLUDE` constraint evidence retains raw `pg_proc.prosecdef`, `prokind='f'`, `proparallel`, `provolatile`, `proisstrict`, scalar `proretset=false`, Boolean `prorettype`, and the exact `pg_operator.oprcode` binding in their existing domain-separated predecessors. Existing issued digest domains are not rewritten by the leakproofness successor.

### Security

- The governed ordinary-EXCLUDE function identity now distinguishes leakproof from non-leakproof implementation functions. PostgreSQL 18.6 allows leakproof functions/operators to be evaluated before security-barrier or row-level-security conditions and uses leakproofness when deciding whether protected planner statistics may be consulted. `ALTER FUNCTION ... LEAKPROOF|NOT LEAKPROOF` changes this property independently of the input-argument identity. Omitting raw `proleakproof` could therefore make materially different security/planner states converge on one semantic identity.

### Retained

- All ordinary-EXCLUDE source-integrity repairs archived through `docs/archive/product-technical-gap-baseline-through-e2284292.md` remain authoritative, including exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw binary `oprkind`; self-commutator; exact `oprcode -> pg_proc` implementation function; independent `oprresult`/`prorettype` with exact `pg_catalog.bool`; raw scalar `proretset=false`; raw `proisstrict`; raw `provolatile`; raw `proparallel`; raw `prokind='f'`; raw `prosecdef`; backing-index `pg_class.relnamespace`; ready/valid/live lifecycle; and exact v3 source-content generation binding.

### Acceptance

- The `proleakproof` repair and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass the repository-pinned Rust 1.98 native suite, the dedicated procedure-leakproof/security-definer/kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus retained tests and coverage, applicable hosted gates, and the bounded PostgreSQL 18 live differential. That differential must independently read `pg_operator.oprkind`, `oprcom`, `oprresult`, `oprcode`, then exact `pg_proc.prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel` from their source rows. `proleakproof` must be read from the exact joined `pg_proc` row and must not be inferred from owner, name, language, security mode, volatility, parallel safety, kind, or other auxiliary properties.