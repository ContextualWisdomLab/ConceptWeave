# PostgreSQL ordinary EXCLUDE operator result-contract integrity

## Problem

The ordinary-EXCLUDE lineage now proves the exact `pg_constraint.conexclop` operator, its self-commutator, and the independently resolved `pg_operator.oprcode -> pg_proc` implementation function. The stable operator identity still carries only schema/name/left-type/right-type, while the stable procedure identity carries schema/name/input-argument types. That is enough to identify the objects but not enough to preserve their result contract.

`pg_operator.oprresult` and `pg_proc.prorettype` are separate catalog facts. If either is lost, a corrupt or faulty bounded source observation can retain the same operator/procedure identity and exact backing-index binding while silently changing what the comparison returns. For an exclusion constraint this is semantic corruption: exclusion checks are Boolean search comparisons, not arbitrary scalar calculations.

## PostgreSQL 18 authority

PostgreSQL 18 documents `pg_operator.oprresult` as the operator result type and `pg_operator.oprcode` as the implementing function. `pg_proc.prorettype` separately stores the function return type. `pg_amop` distinguishes search (`amoppurpose='s'`) from ordering (`'o'`) members and states that search operators must return `boolean`.

The ordinary EXCLUDE creation path reinforces that boundary. In `src/backend/commands/indexcmds.c`, after checking commutativity, PostgreSQL resolves the selected opclass family and calls `get_op_opfamily_strategy(opid, opfamily)` before filling `ii_ExclusionOps`, `ii_ExclusionProcs`, and `ii_ExclusionStrats`. `get_op_opfamily_strategy()` considers search operators only. An operator admitted to an ordinary exclusion constraint is therefore a Boolean search operator, not an ordering operator whose result may be another sortable type.

PostgreSQL also exposes the two result facts independently in source helpers: `get_op_rettype()` reads `Form_pg_operator.oprresult`, while `get_func_rettype()` reads `Form_pg_proc.prorettype`. A governed capture must resolve both OID paths rather than deriving one result type from the other or from the operator-family rule.

Primary references:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_operator*. https://www.postgresql.org/docs/18/catalog-pg-operator.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_amop*. https://www.postgresql.org/docs/18/catalog-pg-amop.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Interfacing extensions to indexes*. https://www.postgresql.org/docs/18/xindex.html
- PostgreSQL source `REL_18_STABLE@1ac292cb1436c788fb6ea29551b0fe459e2cb340`, `src/backend/commands/indexcmds.c` and `src/backend/utils/cache/lsyscache.c`.

## Decision

Add `IndexExclusionConstraintOperatorResultSnapshot` as a domain-separated successor over `IndexExclusionConstraintOperatorProcedureSnapshot`. No issued operator, commutator, procedure, backing-index, or v3 digest is changed.

Each exact ordinary-EXCLUDE constraint/key position must provide one observation containing:

- the repeated governed operator signature;
- the repeated independently governed `oprcode` procedure signature;
- the type independently resolved from `pg_operator.oprresult`; and
- the type independently resolved from `pg_proc.prorettype`.

The successor requires complete and unique position coverage, exact operator/procedure binding to its predecessor, equality of the two independently resolved result types, and exact stable type identity `pg_catalog.bool`. Matching non-Boolean result types are rejected; equality alone is not enough. A lookalike type named `bool` outside `pg_catalog` is also rejected. Missing/shell OIDs must fail in the adapter before a resolved observation can be constructed.

Rejected alternatives:

- adding result types to the already-issued operator or procedure digest domains, because that retroactively changes predecessor identity;
- inferring `prorettype` from `oprresult` or vice versa, because that erases the cross-catalog integrity check;
- accepting any equal result type, because PostgreSQL search operators specifically require Boolean results;
- allowlisting operator names such as `=` or `&&`, because extension operators are valid when their catalog contracts satisfy the same rules.

## Traceability

- Finding review: `5227265048` on exact predecessor `29d99055d349b06f0753a2c99e9516185c84ba77`.
- Structural source/compile RED: `73701220240e875b8311c00a7d39bb23d00bfda3`. The dedicated contract referenced the new observation type before production composition existed; no executed Rust compiler failure is claimed.
- Production successor: `12b8b475903d99acb0ca285e2f40faff9df7ea40` in `index_exclusion_constraint_operator_result.rs`.
- Public composition: `28ddced879d792f44811f04c7891a343ea95add8` in `index_partition.rs`.
- Focused edge/provenance contract: `7cfcc459d9493b7e1d4483dde776b54e6e4211c8` in `index_exclusion_constraint_operator_result_contract.rs`, covering positive provenance, non-Boolean `oprresult`, non-Boolean `prorettype`, matching non-Boolean results, wrong-schema lookalike `bool`, operator/procedure binding drift, missing/duplicate positions, zero position, and unknown receipt coordinates.

## Acceptance

Source repair is not GREEN evidence. One unchanged exact head must pass repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the dedicated result-contract test and all retained Source Observation/relation-partition tests, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted quality/security/dependency/review gates.

The PostgreSQL 18 live differential must independently resolve each exact `conexclop` OID to `pg_operator`, read `oprresult`, `oprcom`, and `oprcode`, follow `oprcode` to the exact `pg_proc` row, read `prorettype`, resolve both result OIDs through `pg_type`, and compare stable identities. The positive control is a real Boolean search operator. Negative controls must alter one independently sourced result fact at a time and must include the case where both facts agree on the same non-Boolean type, proving that cross-catalog equality is not mistaken for valid exclusion semantics.
