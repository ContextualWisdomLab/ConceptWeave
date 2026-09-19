# PostgreSQL EXCLUDE transform-converter access-control integrity

**Status:** Draft / Source Observation successor on `ContextualWisdomLab/ConceptWeave#46`

## Problem

The transform-converter chain already binds each selected `pg_transform` direction to its exact converter definition and independently binds the converter function's `pg_proc.proowner`. That is still insufficient for governed source identity because `pg_proc.proacl` is an independently mutable catalog fact.

PostgreSQL 18 records routine access privileges in `pg_proc.proacl`. Object privileges can be changed with `GRANT` and `REVOKE` without redefining a function's input identity, source body, or owner. `CREATE TRANSFORM` separately requires the creator to own and have `EXECUTE` privilege on each specified FROM-SQL and TO-SQL converter function. A converter can therefore preserve the exact predecessor definition and owner while its authorization surface changes.

## Constraint and boundary

This successor observes object-level converter-function ACL identity; it is not an authorization engine.

For every nonzero converter direction already present in `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot`, the adapter must provide exactly one access-control observation bound to the same `(constraint, key_position, transform_type, direction, converter schema, converter function)` coordinate. Evidence preserves whether raw `pg_proc.proacl` was `NULL` and supplies the canonical object-level `EXECUTE` grant set after PostgreSQL ACL/default-ACL interpretation and same-generation role resolution. `PUBLIC` remains distinct from named roles, grantor identity is retained, and `WITH GRANT OPTION` is retained.

Transitive role-membership closure, superuser bypass, session role selection, and product allow/deny policy are deliberately excluded. PostgreSQL documents effective privileges as the sum of direct grants, role-membership grants, and `PUBLIC`; that runtime authorization computation is not the same fact as the converter object's own `proacl` source identity.

## Alternatives considered

1. **Ignore converter ACL.** Rejected because `GRANT`/`REVOKE EXECUTE` can change the converter's privilege surface while every existing converter-definition and owner digest remains unchanged.
2. **Store only a boolean such as “callable”.** Rejected because it collapses grantor, grantee, `PUBLIC`, grant option, and raw NULL-versus-explicit state, and would require caller/session role closure that does not belong in Source Observation.
3. **Reuse target-function ACL evidence.** Rejected because target `oprcode` and transform converters are distinct `pg_proc` rows with independent ownership and ACLs.
4. **Widen the existing converter-owner digest.** Rejected because issued predecessor digest domains are immutable. The access-control fact is introduced as an ordinary-forward successor.
5. **Bind exact raw ACL text.** Rejected as the public semantic contract because ACL textual rendering is not the intended stable interface and can expose role names unnecessarily. The boundary validates and canonicalizes grant semantics, then reduces them to a domain-separated digest while retaining only the raw NULL-state and grant count outside the digest.

## Selected design

`IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial` consumes `proacl_was_null` and a canonical set of converter-specific `EXECUTE` grants. Duplicate grant evidence fails closed instead of being silently deduplicated. `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation` binds that material to the exact nonzero converter coordinate. `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot` requires set equality with the converter-owner predecessor and rejects missing/extra directions, duplicate coordinates, and converter-function binding drift.

The successor digest includes the frozen converter-owner digest, exact constraint/key/type/direction coordinate, converter schema/function identity, and the access-control material digest. It does not rewrite any prior digest domain.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5234509421`
- structural source/compile RED: `8b65892785b5c4b1db0f30812d1a010fff6ac71f`
- production successor: `c6d8f4ca6c49ec3be44fd797df5f6ce9f8d83a0b`
- public composition: `816621e01667b76e22fe7633a56bf50428d6c2f8`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_access_control.rs`
- focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_access_control_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot`
- exact catalog fact: converter function `pg_proc.proacl`, including raw NULL-state plus object-level `EXECUTE` grant semantics.

The RED commit is structural: the focused contract referenced public access-control types that did not yet exist. No executed `rustc` failure is claimed. Exact-head GREEN requires repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and applicable hosted gates on one unchanged head.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. `pg_proc.proacl` stores routine access privileges.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: GRANT*. `EXECUTE` is an object privilege for functions; `PUBLIC` is an implicit group, and effective privileges also include role-membership grants.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TRANSFORM*. Creating a transform requires ownership and `EXECUTE` privilege on each specified converter function.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. `CREATE OR REPLACE FUNCTION` preserves ownership and permissions while replacing a definition, confirming that definition identity and privilege identity are separate dimensions.

## Risks and follow-up

The adapter must not mistake “object-level ACL grant set” for a complete caller-specific authorization decision. It must not infer grants from owner identity or from target-function ACLs. Same-generation resolution failures are capture failures rather than unknown placeholders.

After this successor reaches exact-head native/hosted GREEN and the bounded PostgreSQL 18 differential validates real converter rows, the next auxiliary converter `pg_proc` fact should be separately reviewed. Nullable `proconfig` is the next identified candidate because function-local execution settings can change independently of definition, owner, and ACL. Security mode, leakproofness, strictness, volatility, parallel safety, planner support, and cost remain later independently reviewable facts.
