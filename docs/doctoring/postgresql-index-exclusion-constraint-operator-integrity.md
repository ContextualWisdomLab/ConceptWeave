# PostgreSQL ordinary EXCLUDE `conexclop` integrity

## Decision

ConceptWeave preserves `pg_constraint.conexclop` for ordinary PostgreSQL `EXCLUDE` constraints as independent source evidence rather than deriving it from the supporting index. Capture-time operator OIDs are resolved to stable operator signatures before governance. The complete ordered constraint-side vector must equal the already-governed exclusion-operator semantics of the exact `conindid` backing index.

This is a domain-separated successor. It does not rewrite the v3, relation-partition, index-partition, ordinary-EXCLUDE identity, period, `conkey`, operator-family, or backing-index exclusion-semantics digest domains.

## Problem and source authority

PostgreSQL 18 stores the supporting index in `pg_constraint.conindid` and separately stores `conexclop`, the OID array of exclusion operators for the constraint. The catalog definition also notes that temporal PRIMARY KEY/UNIQUE constraints using `WITHOUT OVERLAPS` populate `conexclop`; those rows remain in ConceptWeave's key-constraint family and are not counted as ordinary `contype='x'` EXCLUDE constraints.

Without a constraint-side successor, two source states could collapse to one governed identity: the same ordinary EXCLUDE row and backing index could be observed while the independently stored `conexclop` vector disagreed with the backing index's resolved `ii_ExclusionOps` semantics. That is material source ambiguity, not a presentation difference.

Primary source authority is PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/include/catalog/pg_constraint.h`, which defines `conindid` independently and documents `conexclop` as the exclusion-operator OID array. The branch head was re-read on 2026-09-16 before this repair.

## Alternatives considered

1. **Infer `conexclop` from the backing index.** Rejected because it erases independently stored catalog evidence and cannot detect a contradictory catalog tuple.
2. **Persist raw OIDs in governed identity.** Rejected because PostgreSQL OIDs are instance-local join coordinates, not stable semantic identities.
3. **Resolve each OID to a stable operator signature and cross-check the exact backing index.** Selected. The signature retains operator schema/name and both operand types, while the snapshot binds the ordinary constraint coordinate and exact predecessor digests.
4. **Fold temporal PK/UNIQUE `WITHOUT OVERLAPS` into this ordinary-EXCLUDE family.** Rejected because PostgreSQL uses `conexclop` there too, but those rows have different `contype` ownership and already belong to the key-constraint lineage.

## Implementation traceability

- Finding review: `ContextualWisdomLab/ConceptWeave#46` review `5219874038` on exact predecessor `1051c8cddcd16bdb6d47bc5c2c13a7c736b2b859`.
- Source/compile regression contract: `1be6077cc9b888c013acfface68afaa739ccc80d`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_contract.rs`.
- Causal successor implementation: `20b9184cb7bef9a4c6baa4379055c34968ca030a`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator.rs`.
- Public composition/export: `02805d76f63cc5e3a704b415c0161bf44b6955bd`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

The regression contract covers an agreeing constraint-side vector, a contradictory operator vector, missing ordinary-EXCLUDE inventory, and domain-separated provenance. The production successor rebounds the exact source -> relation-partition -> index-partition -> ordinary-EXCLUDE -> period -> `conkey` chain and separately rebounds operator-family -> exclusion-semantics before comparing the two independently observed operator vectors.

## Invariants

- Every ordinary `contype='x'` EXCLUDE coordinate has exactly one constraint-side operator-vector observation.
- The vector is ordered by constraint key position and cannot be empty.
- Resolved operator signatures are exact; schema, operator name, left operand type, and right operand type are material identity.
- The vector must equal the resolved exclusion operators of the exact `conindid` backing index.
- Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` rows are excluded from ordinary EXCLUDE inventory rather than double-counted.
- LLM output, documentation, or a caller-supplied operator name cannot substitute for catalog observation.

## Risk, effect, and remaining verification

The repair prevents a contradictory `pg_constraint.conexclop` row from being silently normalized to backing-index semantics. It also makes the later PostgreSQL transport responsible for reading and resolving the constraint-side OID vector explicitly.

No exact-head GREEN is claimed by this document. The execution host used for this repair exposes no Rust toolchain, and the ConceptWeave Product pull-request workflow is not yet present on protected `main`. One unchanged successor head still needs repository-pinned Rust 1.98 formatting, strict all-target Clippy, focused/workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, hosted Product/security/dependency checks, and qualifying independent review.

The PostgreSQL 18 live differential must read `contype`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, `conkey`, and `conexclop` in one bounded observation, while also reading supporting `pg_index.indkey` and resolved backing-index exclusion semantics. It must retain temporal p/u controls separately so `WITHOUT OVERLAPS` is not reclassified as ordinary EXCLUDE.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `pg_constraint` catalog definition* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `src/include/catalog/pg_constraint.h`.
