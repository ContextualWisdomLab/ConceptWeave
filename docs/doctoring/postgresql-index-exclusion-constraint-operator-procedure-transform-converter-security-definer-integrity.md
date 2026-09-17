# PostgreSQL ordinary EXCLUDE transform-converter security-context integrity

## Decision

ConceptWeave Source Observation must bind each nonzero `pg_transform` converter function's raw same-row `pg_proc.prosecdef` after exact converter definition, owner, object-level `EXECUTE` ACL, and nullable `proconfig` identity. The converter-configuration predecessor remains immutable.

Both PostgreSQL states are observationally valid. `false` represents `SECURITY INVOKER`; `true` represents `SECURITY DEFINER`. The successor domain-separates them without imposing an ordinary-EXCLUDE admission rule.

## Problem and causal finding

Exact converter signature/body, owner, ACL, and function-local configuration do not determine execution privilege context. PostgreSQL 18 permits `ALTER FUNCTION ... SECURITY INVOKER|SECURITY DEFINER` as an auxiliary-property change. `SECURITY INVOKER` executes with the caller's privileges, while `SECURITY DEFINER` executes with the function owner's privileges.

This distinction is security material. PostgreSQL explicitly warns that `SECURITY DEFINER` functions require careful construction, including a trusted `search_path`, because code runs with owner privileges. Function-security guidance likewise treats functions as a potential privilege boundary. Converter `prosecdef` therefore cannot be inferred from owner, ACL, `proconfig`, language, definition material, or current caller identity.

## Constraints

- Preserve the converter-configuration snapshot and all issued digest domains unchanged.
- Observe raw same-row converter `pg_proc.prosecdef`; do not reconstruct it from DDL or infer it from owner/ACL/configuration.
- Require exactly one Boolean observation for every exact nonzero converter direction in the predecessor inventory.
- Repeat converter schema/function identity only to prove predecessor binding; any drift fails closed.
- Admit both invoker and definer states. Source Observation records PostgreSQL truth and does not silently outlaw `SECURITY DEFINER`.
- Keep effective caller authorization and product policy outside this source-identity layer.

## Alternatives considered

### Infer definer mode from a non-default owner or restrictive ACL

Rejected. Ownership and `EXECUTE` ACL are independent catalog dimensions and do not determine `prosecdef`.

### Treat all transform converters as invoker-only

Rejected. That would invent a stronger product admission policy instead of representing PostgreSQL catalog truth. Policy can consume the released semantic fact later.

### Fold the Boolean into the existing configuration digest

Rejected. `prosecdef` and `proconfig` are independently mutable fields. Extending an issued predecessor digest would erase ordinary-forward traceability and violate frozen predecessor semantics.

## Implementation

`IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation` binds raw `prosecdef` to the exact `(constraint, key_position, transform_type, direction, converter schema/function)` coordinate.

`IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot` takes `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot` as its direct predecessor. Expected and observed coordinate sets must match exactly; duplicate, missing, or extra directions fail closed. Converter schema/function identity must match the exact predecessor observation. The successor digest includes predecessor digest, exact coordinate, transform type, converter direction, converter binding, and one raw Boolean byte.

Provenance inherits the exact source connection, policy binding, extractor revision, and observation timestamp from the predecessor rather than creating a second source authority.

## Focused contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_security_definer_contract.rs` covers:

- raw `prosecdef` provenance;
- invoker/definer digest separation;
- complete nonzero converter-direction coverage;
- exact converter-function binding;
- duplicate-coordinate rejection;
- one-based key-position enforcement;
- exact receipt-coordinate lookup; and
- public composition.

Synthetic Boolean fixtures are unit-test distinguishability controls only.

## Live differential requirement

The bounded PostgreSQL 18 differential must resolve each selected `(trftype, target prolang)` `pg_transform` row and every nonzero converter to the exact same-generation `pg_proc` row. For each converter it must independently capture raw `prosecdef` in addition to definition, owner, ACL, and `proconfig`. A mixed-generation join, unresolved converter row, or inferred Boolean is capture failure.

## Security governance mapping

NIST's least-privilege principle requires minimizing privilege available to users or processes for assigned tasks. That supports treating privilege-context elevation as governed evidence, but NIST does not define PostgreSQL's `prosecdef` catalog semantics. PostgreSQL remains the primary technical authority for this field and its runtime behavior.

## Residual risk and next successor

This repair still does not claim complete auxiliary converter `pg_proc` identity. `proleakproof` is independently mutable and affects execution ordering around security-barrier views and row-level security. Strictness, volatility, parallel safety, planner support, planner cost and other fields remain later ordinary-forward review candidates rather than inferred state.

## TRACEABILITY

- Owner repository: `ContextualWisdomLab/ConceptWeave`
- PR: `#46`
- Finding review: `5235250993`
- Structural RED: `96aeb1b07cf32a225db250c570a5a6ff120221be`
- Production successor: `6bfa27f7c20e74dfa870515850bcb04424dad4fe`
- Public composition: `4652d52ad538bf17ba21bd1d8c8ad3b678a5d0d4`
- Source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_security_definer.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_security_definer_contract.rs`
- Direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot`
- Exact catalog fact: each nonzero converter function's raw same-row `pg_proc.prosecdef`

## References

National Institute of Standards and Technology. (n.d.). *Least privilege*. Computer Security Resource Center Glossary. https://csrc.nist.gov/glossary/term/least_privilege

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Function security*. https://www.postgresql.org/docs/18/perm-functions.html
