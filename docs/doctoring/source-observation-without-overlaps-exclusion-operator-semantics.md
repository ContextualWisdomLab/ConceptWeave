# PostgreSQL 18 `WITHOUT OVERLAPS` exclusion-operator semantics

## Problem

ConceptWeave preserves PostgreSQL `pg_constraint.conexclop` as resolved, durable operator signatures rather than catalog OIDs. OIDs are capture-local join coordinates and cannot be governed semantic identity. The initial operator-family representation enforced presence, one-based order, arity, and digest materiality, but it still admitted arbitrary operator names when the vector length matched the constrained columns. That permits an impossible `WITHOUT OVERLAPS` observation to cross the Source Observation boundary.

## Authoritative source semantics

PostgreSQL 18 defines a `UNIQUE (..., valid_at WITHOUT OVERLAPS)` temporal key as behaving like `EXCLUDE USING GIST (... WITH =, valid_at WITH &&)`. The non-`WITHOUT OVERLAPS` columns therefore use equality, while the final range or multirange column uses overlap. PostgreSQL also records `conperiod` directly and exposes `conexclop` as the per-column exclusion-operator vector for exclusion constraints and `WITHOUT OVERLAPS` primary/unique constraints.

The two facts have different responsibilities:

- `pg_constraint.conperiod=true` is the source-authoritative declaration that the primary/unique constraint uses `WITHOUT OVERLAPS`; operator shape never invents this fact.
- Once `conperiod=true` is observed for a primary/unique key, its `conexclop` vector must be semantically coherent with PostgreSQL's generated temporal-key contract: every non-final operator name is `=`, and the final operator name is `&&`.

Operator namespace and qualified operand types remain retained evidence because operator names are overloadable and OIDs are not stable identity. The validation above constrains the operator *role* without collapsing the full resolved signature.

## Rejected alternatives

### Accept any operator vector of the correct arity

Rejected. This preserves raw catalog-shaped data but allows impossible temporal-key semantics such as `id WITH =#` or `valid_at WITH =` to become governed evidence.

### Infer `conperiod` from `=`/`&&`

Rejected. PostgreSQL exposes `conperiod` directly. Inferring temporal declaration from index/operator shape would reverse source authority and could confuse an explicit exclusion constraint with a temporal key.

### Require `pg_catalog` as the operator namespace

Rejected. The semantic contract is the operator role (`=` for prefix key columns and `&&` for the final temporal column). The resolved namespace and operand types remain evidence and digest material, but hard-coding a namespace would conflate semantic role with lookup location and unnecessarily constrain extension or user-defined type/operator resolution.

## Implementation traceability

- Finding review: `5186120516` on ConceptWeave PR #46.
- Behavioral RED: `8b5c3ba74fc9f8dfb0c8f6500634d62c8feaf14f` (`constraint_period_exclusion_operator_contract.rs`).
- Production repair: `2333d7f7640931ab158734e12dacd8729aa9983f` (`constraint_period.rs`).
- Boundary-adjusted regression: `5fa8112359a28850e73cf6912223b883d2c7f21d`.
- Error contract: `constraint_period_exclusion_operators`.

Exact-head Rust/Product acceptance remains separate evidence and must be regenerated whenever the head moves.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
