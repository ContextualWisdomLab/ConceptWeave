# PostgreSQL ordinary-EXCLUDE transform-converter owner integrity

## Decision

ConceptWeave treats each nonzero `pg_transform.trffromsql` or `pg_transform.trftosql` converter function's `pg_proc.proowner` as an independently observed semantic/security identity fact. The transform-converter predecessor remains immutable. A new successor binds every retained converter direction to the raw nonzero owner OID plus the role name resolved from the same source generation.

This is observational evidence, not an authorization policy. The successor does not require a particular owner, does not infer ownership from schema ownership or transform creation, and does not treat ownership as equivalent to `EXECUTE` ACL, `SECURITY DEFINER`, configuration, leakproofness, volatility, parallel safety, planner support, or cost.

## Problem

`IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot` authenticates the selected `(type, language)` transform row, optional FROM-SQL and TO-SQL converter presence, exact converter function coordinate/signature, implementation language, and exact source-definition material. It intentionally left other mutable converter `pg_proc` properties outside its digest.

That boundary means two source generations can retain the same converter function coordinate and definition while changing `pg_proc.proowner`. PostgreSQL permits `ALTER FUNCTION ... OWNER TO` without changing the function's input identity or implementation body. Ownership is therefore not derivable from converter source identity.

The distinction is operationally material. PostgreSQL assigns object-control authority to the owner, and `CREATE TRANSFORM` requires ownership and `EXECUTE` privilege on converter functions when a transform is created. An ownership change can therefore alter who controls a converter even while the transform row and function code remain unchanged.

## Authoritative basis

- PostgreSQL Global Development Group. (2026). *ALTER FUNCTION — change the definition of a function* (PostgreSQL 18). https://www.postgresql.org/docs/18/sql-alterfunction.html
  - `ALTER FUNCTION ... OWNER TO` changes function ownership without redefining the function identity.
  - A `SECURITY DEFINER` function subsequently executes as the new owner, demonstrating that owner identity is a separate runtime-security fact rather than a naming attribute.
- PostgreSQL Global Development Group. (2026). *CREATE TRANSFORM — define a new transform* (PostgreSQL 18). https://www.postgresql.org/docs/18/sql-createtransform.html
  - Creating a transform requires ownership and `EXECUTE` privilege on each specified converter function. Ownership and ACL are therefore related but separate facts.
- PostgreSQL Global Development Group. (2026). *pg_proc* (PostgreSQL 18 system catalog). https://www.postgresql.org/docs/18/catalog-pg-proc.html
  - `proowner` is a dedicated `oid` reference to `pg_authid`, independent of function source, language, security mode, planner attributes, and ACL.
- National Institute of Standards and Technology. (2020, updated 2025). *Security and Privacy Controls for Information Systems and Organizations (SP 800-53 Rev. 5), AC-3 Access Enforcement and AC-6 Least Privilege*. https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
  - Governance mapping only: preserving the exact controlling principal supports later access-control evidence. NIST does not define PostgreSQL catalog semantics.

## Alternatives considered

### Reuse converter source definition as owner identity

Rejected. `prosrc`/`probin`/`prosqlbody` and `proowner` can change independently. Content identity cannot prove controlling-principal identity.

### Infer owner from schema owner, transform creator, or session principal

Rejected. PostgreSQL stores function ownership directly in `pg_proc.proowner`; those other principals are not substitutes for the function owner.

### Combine owner, ACL, configuration, security mode, and planner facts into one repair

Rejected. Those fields are independently mutable catalog facts and already follow the project's ordinary-forward successor discipline for target functions. Combining them would blur causal review and make later evidence harder to localize. Owner is repaired first; remaining converter auxiliary facts stay explicit residual gaps.

### Rewrite the transform-converter predecessor digest

Rejected. Issued predecessor digest domains are immutable. The repair is a domain-separated successor.

## Contract

`IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot` consumes one exact `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot` and requires exactly one owner observation for every nonzero converter direction.

Each observation binds:

- ordinary-EXCLUDE constraint coordinate;
- one-based exclusion-key position;
- exact selected transform type;
- converter direction (`from_sql` or `to_sql`);
- exact converter schema/function coordinate repeated from the predecessor;
- raw nonzero `pg_proc.proowner` OID;
- exact same-generation resolved owner role name.

The snapshot canonicalizes observations by constraint, key, transform type, and direction. Missing or extra converter-owner coordinates, duplicate coordinates, zero positions, blank function/role identifiers, zero owner OID, and converter binding drift fail closed. The digest incorporates the exact predecessor digest and every owner fact under a new domain.

The owner role name is retained together with the OID because an opaque OID alone is not a portable semantic identifier. The raw OID is retained because same-name role recreation is not the same catalog identity within one captured generation.

## Privacy and data minimization

No credential, password verifier, membership graph, connection string, or role secret is retained. Only the already-public catalog identity required to explain converter control is stored: converter coordinate, raw owner OID, and resolved role name.

## Verification strategy

The focused contract fixes the following behavior:

- exact owner provenance is exposed through a source receipt;
- changing owner OID/role changes the successor digest while the converter predecessor remains unchanged;
- every nonzero converter direction requires one owner observation;
- wrong converter-function binding fails closed;
- zero owner OID fails closed;
- duplicate owner coordinates fail closed;
- unknown receipt coordinates fail closed;
- the new snapshot is publicly composed through the relation-partition crate.

The first test commit is structural RED only because the new public types did not yet exist and this execution environment does not provide the repository-pinned Rust 1.98 toolchain. No compiler RED or GREEN is claimed.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5233862979`
- structural RED: `6b8f1daf317ccae54a9d8ebbd7a1144bf9881b5c`
- production successor: `11c9cbebab54685b8a06516d640f0b518a698a5b`
- public composition: `0ccc3415f999bce9353330affaff2b71335f4d08`
- focused receipt-edge correction: `fdc9c792db13148af66dcd4bb0f1fe414fca582a`
- canonical receipt-location correction: `fd3ced1bb5325ceabdd0c0c166410eda4a260035`
- pre-owner gap-baseline archive: `22bc5cf3e0806e07d156f1ad9cabd66c2f8edf64`
- pre-owner CHANGELOG archive: `20f7eabf5274b9a394332a41260c80fb62cc16e6`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_owner.rs`
- composition: `crates/conceptweave-relation-partition/src/index_partition.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_owner_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot`
- exact catalog fact: each nonzero converter function's `pg_proc.proowner`, resolved to the same-generation role name.

## Residual risk and next work

Owner identity closes only the first converter auxiliary `pg_proc` gap. Complete converter runtime/security identity still requires separately reviewed observation of mutable ACL, `proconfig`, `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, planner support/cost, and any other material auxiliary field not already content-bound by the converter predecessor.

The bounded PostgreSQL 18 live differential must now resolve every nonzero converter OID to the exact converter `pg_proc` row and independently capture `proowner` plus role resolution in the same source-content generation. An unresolved owner OID or generation mismatch is capture failure, not `unknown` owner evidence.
