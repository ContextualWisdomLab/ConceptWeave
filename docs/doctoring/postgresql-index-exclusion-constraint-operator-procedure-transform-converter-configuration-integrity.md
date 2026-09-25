# PostgreSQL ordinary EXCLUDE transform-converter configuration integrity

## Decision

ConceptWeave Source Observation must bind each nonzero `pg_transform` converter function's nullable `pg_proc.proconfig` as an independently observed catalog fact after converter definition, owner, and object-level `EXECUTE` ACL identity. The already-issued converter access-control predecessor remains immutable.

The new successor preserves the exact raw `proconfig` null-state, array entry count, entry order, and entry bytes in a domain-separated SHA-256 material digest. Raw configuration values are not retained in provenance receipts. An explicit empty array is not normalized to `NULL`.

## Problem and causal finding

Exact converter schema/name, implementation body, `proowner`, and `proacl` do not determine function-local run-time settings. PostgreSQL 18 exposes function-local configuration through `SET configuration_parameter ...` in `CREATE FUNCTION` and permits `ALTER FUNCTION ... SET`, `SET FROM CURRENT`, `RESET`, and `RESET ALL` without changing the function's input identity. A `RESET` removes the function-local setting and returns execution to the environment-provided value.

This is material to semantic and security identity. PostgreSQL's own `CREATE FUNCTION` guidance uses a `SECURITY DEFINER` example where a controlled `search_path` prevents untrusted temporary objects from shadowing trusted objects. Equal converter definition/owner/ACL therefore cannot justify equal run-time semantics when `proconfig` differs.

## Constraints

- Preserve the converter access-control snapshot and digest domain unchanged.
- Observe raw same-row converter `pg_proc.proconfig`; do not infer configuration from current/session GUCs, `pg_settings`, reconstructed DDL, owner, ACL, language, or target-function settings.
- Preserve `NULL` versus explicit array, array order, and raw entry bytes.
- Keep arbitrary/custom GUC values out of downstream receipts by reducing raw entries immediately to a digest.
- Require one configuration observation for every exact nonzero converter direction already governed by the access-control predecessor.
- Repeat converter schema/function identity only to verify predecessor binding; drift fails closed.
- Do not interpret configuration values as product policy or compute an effective session configuration in Source Observation.

## Alternatives considered

### Infer from `CREATE FUNCTION` text or converter definition material

Rejected. `ALTER FUNCTION ... SET/RESET` can change function-local settings independently of definition text and stable input identity. Reconstructed DDL would also conflate observation with rendering.

### Read effective session values

Rejected. Effective session configuration combines cluster/database/role/session/function scopes and is not the same catalog fact as converter `pg_proc.proconfig`.

### Canonicalize or sort `proconfig` entries

Rejected. The contract is source identity, not semantic normalization. Exact catalog-array order and bytes remain distinguishable so the owner does not silently rewrite source state.

### Store plaintext settings in receipts

Rejected. Configuration entries may contain operational or environment-specific values. The snapshot needs stable identity and provenance, not downstream replication of raw GUC material.

## Implementation

`IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial::from_proconfig` frames `None` separately from `Some`, includes the exact array length and each entry in source order, and emits a dedicated SHA-256 digest.

`IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot` takes `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot` as its direct predecessor. Its expected coordinate set is the predecessor's exact `(constraint, key_position, transform_type, direction)` inventory. Duplicate/missing/extra coordinates fail closed. Each configuration observation repeats converter schema/function identity and must match its predecessor converter binding exactly.

The successor digest includes the predecessor digest, exact coordinate, transform type, converter direction, converter schema/function identity, and configuration-material digest. Provenance inherits the exact source connection, policy binding, extractor revision, and observation timestamp from the predecessor.

## Focused contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_configuration_contract.rs` covers:

- raw `proconfig` provenance and public composition;
- `NULL` versus explicit empty-array digest separation;
- setting-value digest separation;
- catalog-array order preservation;
- complete nonzero converter-direction coverage;
- exact converter-function binding;
- duplicate coordinate rejection;
- one-based key-position enforcement; and
- exact receipt-coordinate lookup.

Synthetic configuration values are unit-test distinguishability controls only; they are not claims about production PostgreSQL instances.

## Live differential requirement

The bounded PostgreSQL 18 differential must resolve each selected `(trftype, target prolang)` `pg_transform` row and each nonzero FROM-SQL/TO-SQL converter to the exact same-generation `pg_proc` row. For each converter it must independently capture raw nullable `proconfig` from that row in addition to all previously retained definition, owner, and ACL evidence. Unresolved converter rows, mixed-generation joins, or substitution with effective/session settings are capture failures.

## Residual risk and next successor

This repair does not claim complete auxiliary converter `pg_proc` identity. `prosecdef`, `proleakproof`, strictness, volatility, parallel safety, planner support, and planner cost remain independently mutable and must be reviewed as separate ordinary-forward successors rather than inferred from `proconfig` or other converter facts.

## TRACEABILITY

- Owner repository: `ContextualWisdomLab/ConceptWeave`
- PR: `#46`
- Finding review: `5235145397`
- Structural RED: `b99338232954258f93fd43253e61cd58c76e295b`
- Production successor: `f863ec28ac0d910befbd3c416e9a0ab4797d300c`
- Public composition: `07053742f96f253cf3b76d1563d500a0e17be232`
- Source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_configuration.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_configuration_contract.rs`
- Direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot`
- Exact catalog fact: each nonzero converter function's nullable `pg_proc.proconfig`

## References

National Institute of Standards and Technology. (2020, updated 2025). *Security and Privacy Controls for Information Systems and Organizations (NIST SP 800-53 Rev. 5), CM-6 Configuration Settings*. https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
