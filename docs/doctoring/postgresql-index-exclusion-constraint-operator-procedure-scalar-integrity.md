# PostgreSQL ordinary EXCLUDE operator procedure scalar integrity

## Decision

ConceptWeave Source Observation must preserve raw `pg_proc.proretset` for every exact implementation function reached through an ordinary EXCLUDE constraint's `pg_operator.oprcode`. Governed ordinary-EXCLUDE enforcement admits the position only when the independently observed raw flag is `false`.

This is an enforcement-capability invariant, not a repository-wide prohibition on set-returning functions and not a claim that generic `CREATE OPERATOR` syntax rejects every possible SRF-backed operator definition.

## Problem

The retained #46 lineage already preserves, for each exact `pg_constraint.conexclop` position:

- a stable qualified binary operator signature;
- raw `pg_operator.oprkind='b'`;
- independently resolved self-commutator state;
- the exact `pg_operator.oprcode -> pg_proc` implementation-function signature;
- independent `pg_operator.oprresult` and `pg_proc.prorettype`, both resolved to exact `pg_catalog.bool`.

Those facts do not include `pg_proc.proretset`. A function declared `SETOF bool` still has `prorettype = bool`; normalizing only the result type therefore cannot distinguish singleton Boolean execution from set-returning execution.

PostgreSQL 18 exclusion enforcement does distinguish the execution model. In `src/backend/executor/execIndexing.c`, the conflict/recheck path invokes each exclusion procedure through `OidFunctionCall2Coll(...)` and immediately converts the returned datum with `DatumGetBool(...)`. That path is a scalar function call and does not establish a set-returning-function protocol or consume multiple rows. `src/backend/parser/parse_oper.c` separately records `get_func_retset(opform->oprcode)` in ordinary operator expression nodes, confirming that return cardinality is independent of `oprresult`/`prorettype`.

The source model must therefore retain the raw cardinality flag rather than infer scalar behavior from the Boolean result type.

## Constraints

1. Do not rewrite existing v3, relation-partition, index-partition, ordinary-EXCLUDE, operator, commutator, operator-procedure, operator-result, or operator-kind digest domains.
2. Bind the raw flag to the exact already-governed operator/procedure position.
3. Rebind the supplied operator-kind predecessor to the supplied result predecessor before accepting successor evidence, so cross-generation or mismatched predecessor chains fail closed.
4. Treat OIDs as capture-local join coordinates; semantic identity continues to use stable qualified operator/procedure signatures.
5. Do not infer `proretset` from `prorettype`, operator name, function name, operator-family membership, strategy number, or `oprkind`.
6. Do not broaden this invariant into a global ban on SRFs. The scope is the scalar invocation contract used by ordinary EXCLUDE enforcement.

## Alternatives considered

### Reuse `prorettype = pg_catalog.bool` as sufficient evidence

Rejected. `prorettype` identifies the element/result type, while `proretset` independently identifies set-returning cardinality. The same Boolean type can therefore describe incompatible execution contracts.

### Add `proretset` to `QualifiedProcedureSignature`

Rejected. That would retroactively change a stable procedure identity used by existing digest families. Raw execution-shape evidence belongs in an ordinary-forward successor.

### Reject every PostgreSQL set-returning function during source observation

Rejected. SRFs are valid PostgreSQL objects and belong to other semantic domains. Only the exact function used as an ordinary EXCLUDE enforcement operator is constrained here.

### Require a particular function language, volatility, strictness, or leakproof flag in the same repair

Rejected. Those properties are separate catalog facts with different PostgreSQL semantics. Bundling them without an independently verified EXCLUDE invariant would widen the change beyond the causal finding.

## Repair

`IndexExclusionConstraintOperatorProcedureScalarSnapshot` is layered after `IndexExclusionConstraintOperatorKindSnapshot` while receiving the exact `IndexExclusionConstraintOperatorResultSnapshot` needed to verify the operator/procedure binding.

For every exact constraint/key position it requires exactly one `IndexExclusionConstraintOperatorProcedureScalarObservation` containing:

- the ordinary-EXCLUDE coordinate;
- one-based key position;
- the exact stable operator signature;
- the exact stable `oprcode` procedure signature;
- raw `pg_proc.proretset` as `returns_set`.

Construction first rebuilds the kind snapshot from the supplied result snapshot and retained kind observations. A digest mismatch is rejected as `index_exclusion_constraint_operator_procedure_scalar_predecessor_binding`. It then rejects duplicate or incomplete position inventories, operator/procedure binding drift, and `returns_set=true`. The predecessor kind digest, exact coordinate/key position, stable operator/procedure signatures, and raw Boolean flag enter a new domain-separated digest.

## RED / GREEN semantics

Review `5228163996` records the valid finding on exact predecessor `ef67a083c57f9dca142ce09b691a76fc5e1c03d3`.

Commit `c1fb88d1a29be1ac7503b1eb04f574e5a32a2d8b` added the dedicated contract before the production types existed, so it is a structural source/compile RED. No executed compiler failure is claimed because the current execution host does not provide the repository-pinned Rust 1.98 toolchain.

Production and public composition proceed ordinary-forward. Source repair is not GREEN until one unchanged exact head passes the pinned native suite and applicable hosted gates.

## Live differential

A bounded PostgreSQL 18 differential must, for each exact ordinary-EXCLUDE `conexclop` position:

1. resolve the exact `pg_operator` row independently;
2. read raw `oprkind`, `oprcom`, `oprresult`, and `oprcode` from that row;
3. follow `oprcode` to the exact `pg_proc` row;
4. independently read both `prorettype` and `proretset` from that procedure row;
5. resolve the result types and retain the existing Boolean-result contract;
6. require `proretset=false` for the governed EXCLUDE enforcement position;
7. retain the existing operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation.

A negative control may use synthetic/corrupt source evidence with `proretset=true`; it must not be described as a claim that normal PostgreSQL DDL necessarily creates such an enforcement-valid EXCLUDE constraint.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5228163996`
- structural RED: `c1fb88d1a29be1ac7503b1eb04f574e5a32a2d8b`
- production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_scalar.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_scalar_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorKindSnapshot` rebound to `IndexExclusionConstraintOperatorResultSnapshot`
- catalog fact: `pg_proc.proretset`
- executor authority: PostgreSQL `REL_18_STABLE@1ac292cb1436c788fb6ea29551b0fe459e2cb340`, `src/backend/executor/execIndexing.c`
- operator-expression authority: same PostgreSQL revision, `src/backend/parser/parse_oper.c`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE OPERATOR*. https://www.postgresql.org/docs/18/sql-createoperator.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source code: REL_18_STABLE* (commit `1ac292cb1436c788fb6ea29551b0fe459e2cb340`). https://github.com/postgres/postgres/tree/1ac292cb1436c788fb6ea29551b0fe459e2cb340
