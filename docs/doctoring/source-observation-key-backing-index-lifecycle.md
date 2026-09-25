# Source Observation doctoring: key backing-index lifecycle evidence

## Decision

When ConceptWeave explicitly observes PostgreSQL `pg_index` lifecycle state for an index that is being used as governed evidence for a PRIMARY KEY or UNIQUE constraint, an observed unusable state must fail closed before the snapshot digest or receipt is authoritative.

The existing owner contract already binds exact same-relation/same-name supporting-index identity, ordered key shape, uniqueness, partial-predicate absence, PK/UNIQUE catalog role, observed UNIQUE null treatment, timing `indimmediate`, and temporal exclusion/GiST coherence. The remaining contradiction is that `IndexObservation` also carries `indisready`, `indisvalid`, and `indislive`, while the shared backing-index predicate currently ignores those observed facts.

PostgreSQL 18 defines the three flags operationally:

- `pg_index.indisvalid = false`: the index can be incomplete and cannot safely be used for queries; if the index is unique, its uniqueness property is not guaranteed.
- `pg_index.indisready = false`: `INSERT`/`UPDATE` must ignore the index.
- `pg_index.indislive = false`: the index is being dropped and must be ignored for all purposes.

`pg_constraint.conindid` separately identifies the supporting index for UNIQUE, PRIMARY KEY, FOREIGN KEY, and exclusion constraints. Catalog OIDs are capture-time join coordinates only; ConceptWeave keeps the canonical domain binding as the exact relation + constraint/index coordinate.

Therefore an explicitly observed false lifecycle flag cannot coexist with an index being promoted as coherent supporting evidence. This slice deliberately does not redefine `None` (unobserved) as false. Whether governed constraint evidence must require lifecycle-family completeness is a separate contract decision; this repair only prevents known contradictory source state from being accepted.

## Behavioral contracts

- `crates/conceptweave-observation/tests/constraint_backing_index_contract.rs`
  - explicitly observed `indisready=false`, `indisvalid=false`, or `indislive=false` must reject PK/UNIQUE timing admission with `constraint_backing_index`;
  - an explicitly ready + valid + live control remains admissible.
- `crates/conceptweave-observation/tests/constraint_period_backing_index_presence_contract.rs`
  - the same three explicit false states must reject positive `conperiod=true` PK/UNIQUE admission with `constraint_period_backing_index`;
  - an explicitly ready + valid + live temporal control remains admissible.

The intended causal source seam is the shared `key_constraint_backing_index_static_shape_matches()` predicate in `crates/conceptweave-observation/src/lib.rs`, so timing and temporal-period admission cannot drift. `conperiod` remains the authority for temporal declaration; lifecycle state never invents constraint semantics.

## Traceability

- Review finding: ConceptWeave PR #46 review `5186802339`.
- RED commits: `681e280fa608b90495a035c04272bc89c546eaca` and `ed4882e7138f8b055b2913a7b25b8089ddc1b95c`.
- Production target: `key_constraint_backing_index_static_shape_matches()` and its existing callers in `canonicalize_constraint_timings()` / `canonicalize_constraint_periods()`.
- Acceptance: repository-pinned Rust 1.98 fmt, strict all-target/workspace Clippy with warnings denied, focused contracts above, workspace/doc tests, release build, owned coverage, and applicable Product/security/dependency/review gates on one unchanged exact successor.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
