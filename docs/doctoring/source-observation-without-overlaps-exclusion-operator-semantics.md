# PostgreSQL 18 `WITHOUT OVERLAPS` exclusion-operator semantics

## Problem

ConceptWeave preserves PostgreSQL `pg_constraint.conexclop` as resolved durable operator signatures rather than catalog OIDs. OIDs are capture-local join coordinates and cannot be governed semantic identity. An earlier #46 repair correctly required the operator vector to be present and to match temporal-key arity, but incorrectly treated the literal catalog operator names `=` and `&&` as the semantic authority for the prefix equality and final overlap roles.

That spelling rule is stricter than PostgreSQL 18 itself. A valid extensible GiST operator class can map PostgreSQL's fixed compare types to operator-family strategies whose selected operators have different names. Conversely, a literal operator named `=` or `&&` does not by itself prove membership in the backing index's operator family or the compare-type role PostgreSQL selected.

## Authoritative source semantics

PostgreSQL 18 documents `UNIQUE (..., valid_at WITHOUT OVERLAPS)` as behaving like `EXCLUDE USING GIST (... WITH =, valid_at WITH &&)`: prefix columns have equality semantics and the final range or multirange column has overlap semantics. PostgreSQL also records `conperiod` directly and exposes `conexclop` as the per-column exclusion-operator OID vector for exclusion constraints and `WITHOUT OVERLAPS` primary/unique constraints.

The server implementation resolves those semantic roles through the selected index operator class, not by testing operator spelling. In `ComputeIndexAttrs()`, a `WITHOUT OVERLAPS` key requests `COMPARE_EQ` for every non-final key and `COMPARE_OVERLAP` for the final key, then calls `GetOperatorFromCompareType()` and stores the returned operator OID in the exclusion vector. GiST operator classes can provide compare-type translation through their operator-family support function, so custom operator classes remain valid when their selected equality/overlap operators do not use the literal names `=` and `&&`.

The responsibilities therefore remain separate:

- `pg_constraint.conperiod=true` is the source-authoritative declaration that the primary/unique constraint uses `WITHOUT OVERLAPS`; operator shape never invents this fact.
- `pg_constraint.conexclop` supplies the exact selected per-column operators. Their namespace, name, and qualified operand types are retained and digested as provenance; spelling is not reinterpreted as semantic role.
- Equality/overlap role verification belongs to the PostgreSQL adapter boundary that already owns catalog-OID resolution. It must resolve each backing-index key's exact operator class/operator family and verify that PostgreSQL's compare-type translation for `COMPARE_EQ` or `COMPARE_OVERLAP`, as appropriate for the position, returns the same `conexclop` operator before the observation crosses the ACL.
- The owner aggregate still requires a complete contiguous vector whose arity matches the constrained key. It does not recreate PostgreSQL's operator-family catalog graph from names.

## Rejected alternatives

### Require literal `=` / `&&` names

Rejected after source verification. PostgreSQL selects the operators from compare-type translation in the resolved operator class. Literal spelling can reject valid custom GiST operator classes and still fails to prove operator-family membership.

### Accept a missing or wrong-length operator vector

Rejected. `conexclop` is material source evidence for `WITHOUT OVERLAPS`; the governed representation requires the complete ordered vector and exact key arity.

### Infer `conperiod` from operator or index shape

Rejected. PostgreSQL exposes `conperiod` directly. Inferring temporal declaration from index/operator shape reverses source authority and can confuse an ordinary exclusion constraint with a temporal key.

### Persist catalog OIDs as semantic identity

Rejected. OIDs remain capture-transaction join coordinates. The adapter resolves them to durable qualified operator and operator-class coordinates plus the verified compare-type relation before admission.

## Implementation traceability

Initial operator-shape lineage:

- finding review `5186120516`;
- behavioral RED `8b5c3ba74fc9f8dfb0c8f6500634d62c8feaf14f`;
- production repair `2333d7f7640931ab158734e12dacd8729aa9983f`;
- validation-boundary regression `5fa8112359a28850e73cf6912223b883d2c7f21d`;
- arity witness repair `ba40e6078461685e38bc2a7a2c2206ea1b37fb5c`.

Source-correction lineage:

- corrective finding review `5186175514` on exact predecessor `55d53adfa13297ff9eaa19555b6c2c2eb5890a3f`;
- behavioral RED `a9065d460af4c00d84c2453b744796effd4d0485` proving custom operator names selected by operator-class compare translation must remain admissible and preserved;
- production repair `e286c3524f036138546d91cb0d53631e8c8e41bf` removing operator-name inference while retaining non-empty, contiguous ordered signatures and aggregate arity checks.

Exact-head Rust/Product acceptance remains separate evidence and must be regenerated whenever the head moves. The future PostgreSQL adapter is not complete until compare-type/opclass membership is verified from the same bounded catalog snapshot.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: GiST indexes*. https://www.postgresql.org/docs/18/gist.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: `ComputeIndexAttrs()` (`src/backend/commands/indexcmds.c`)*. https://github.com/postgres/postgres/blob/512d3e8919b649d36a6f246c657c8b8df7536e8a/src/backend/commands/indexcmds.c
