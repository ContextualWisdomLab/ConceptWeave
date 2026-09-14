# Product / Technical Gap Baseline

**Snapshot:** 2026-09-15

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. The ordinary-forward relation-partition successor preserves complete table-level `pg_class.relispartition`, direct declarative `pg_inherits` parentage, detach-state fail-closed handling, acyclic parent topology, exact receipts, and bidirectional coherence with the PostgreSQL 18 NOT NULL family. Its domain-separated index-partition successor preserves index/partitioned-index `relkind`, index `relispartition`, exact direct index parentage, detach state, receipts, table/index parent coherence, the partitioned-index validity lifecycle, and the modeled PostgreSQL attachment-equivalence facts `indisunique`, `indnullsnotdistinct`, access method, key/`INCLUDE` cardinality and role boundary, simple-column versus expression slot shape, and exact simple-column mapping. Exact-head native and hosted acceptance remain pending.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN Ready and mechanically mergeable. It remains the prerequisite for repository-owned Product `pull_request` evidence because the workflow is absent from protected ConceptWeave `main`.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column collation, generation declaration mode, default/generated expressions, identity declaration mode, first-class NOT NULL constraint evidence, and domain-separated declarative-partition relation and index topology layers. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Frozen `ColumnObservationV3` and the predecessor v3 snapshot digest meaning remain unchanged.

The relation-partition P1 recorded in review `5197218821` is source-repaired ordinary-forward rather than by mutating frozen v3 identity. The `conceptweave-relation-partition` crate layers complete per-relation `pg_class.relispartition` observations over an exact predecessor digest, resolves every positive direct parent to an observed `PartitionedTable`, rejects detach-pending state and cycles, and issues exact relation-partition receipts. Generic inheritance remains outside this family.

Review `5198504917` on exact `551862ff8647a673bcab5a96404a02eb39a28465` found that initial NOT NULL cross-family validation was one-way. Behavioral RED `7b00339e699ea83c77ed9ac45057d51d73fa83e4` and production repair `c708d45429f9b942d8b5ee7628f6876595a2a856` make relation membership and inherited parent NOT NULL linkage bidirectionally coherent without invalidating extra child-local NOT NULL constraints. Primary-source doctoring is `docs/doctoring/postgresql-relation-partition-not-null-inheritance-integrity.md`.

### Index-partition topology

Review `5199149942` on exact predecessor `18c19281260f98d450b399a12c469fe86d9acbb5` identified that relation-level partition evidence omitted PostgreSQL index partition topology. PostgreSQL 18 represents ordinary indexes as `pg_class.relkind='i'`, partitioned indexes as `relkind='I'`, defines `relispartition` for tables **and indexes**, and stores direct index parent-child edges in `pg_inherits`. Because v3 nested indexes carried none of those index-relation facts, an attached child index and an otherwise-identical local/unattached index could collapse to the same relation-partition identity.

The repair does not mutate the frozen v3 or relation-partition digest. `index_partition.rs` adds a domain-separated successor complete over every nested observed index. It retains index versus partitioned-index kind, `relispartition`, exact direct parent index coordinate, `inhdetachpending` fail-closed behavior, deterministic digesting, exact receipts, parent-index resolution, and consistency between the index parent owner and the independently observed direct table-partition parent.

Initial topology chronology is explicit:

- `209f2a5832396bba420ede11a52d54406340e3eb` staged the implementation file while it was unreachable from the crate root and therefore did not change reachable production behavior;
- `1dd392936aed8103c4ebb6879f923343ab733818` added the external behavioral contract while the module was still unreachable, requiring attached versus local/unattached topology to change identity and covering wrong table parent, completeness, detach-pending, and receipt boundaries;
- `7865809a5dc090679a1b34d3c8074193c652f395` wired the successor into the production crate API;
- `2dc8c91535acc99550e0760166451bf31631880f` corrected canonical coordinate ordering after static review found that `RelationKind` deliberately does not derive `Ord`;
- `docs/doctoring/postgresql-index-partition-topology-integrity.md` binds the repair to PostgreSQL 18 primary documentation and exact review/commit lineage.

A second review, `5199344556` on exact `d156ad01cd6bc7b964952076a1bab71af5c4d416`, found a lifecycle contradiction. The successor allowed `parent partitioned index indisvalid=true` while a direct table partition carried only a local/unattached child index. PostgreSQL 18 permits staged local child indexes only while a `CREATE INDEX ON ONLY` parent remains invalid; the parent is marked valid automatically after every partition index has been attached.

- behavioral RED `d819639b3f8973b2d3bca166c9a091b6868b92e1` requires the valid-parent/local-child state to fail and preserves invalid-parent/local-child staging;
- minimal production repair `664afcc1684ead7569d9d6dd070afca4ef847ceb` resolves the exact parent `IndexObservation.valid()` state and requires an explicit child `pg_inherits` edge for each direct table partition only when the parent is `Some(true)`;
- doctoring `695230758302ade2405be9c1249db76e5e90142f` updates the PostgreSQL 18 lifecycle rationale without inventing attachment from name or definition similarity.

A third review, `5199509892` on exact `9fd9bb20740d57def0019b3ddbebb9e8126cc096`, found that an admitted direct index-parent edge could still connect a unique partitioned parent to a non-unique child. PostgreSQL 18 `ALTER INDEX ... ATTACH PARTITION` requires an equivalent definition, and `REL_18_STABLE` `CompareIndexInfo()` rejects candidate indexes when `ii_Unique` differs before checking the remaining definition.

- behavioral RED `6161a9955cb03284e189a367bfcfc017af610222` isolates the impossible unique-parent/non-unique-child edge while satisfying table topology and attachment evidence;
- minimal production repair `71bd71af598cd9a9979bd235c167b558ab938804` resolves the exact parent and child `IndexObservation` values for every direct edge and rejects an `is_unique()` mismatch before hashing;
- `docs/doctoring/postgresql-index-partition-topology-integrity.md` records the primary-source rationale and intentionally does not compare names, comments, tablespaces, lifecycle flags, or raw rendered DDL.

A fourth review, `5199613530` on exact `d9b6de9709925d7a33502eaff7c5d338232b3543`, found that two additional attachment-equivalence properties already owned by v3 could still contradict the direct edge. PostgreSQL `CompareIndexInfo()` checks `ii_NullsNotDistinct` immediately after uniqueness and then requires the same access method.

- behavioral RED `b84e49773bc31c140ec6ed0638d75e4f54d75f80` covers a unique parent/child pair with mismatched `NULLS NOT DISTINCT`, a non-unique pair with mismatched access methods, and a matching positive control;
- minimal production repair `f44973e113dcbacd13ff5554bb464a252025361e` compares the modeled `nulls_not_distinct()` and `access_method()` properties after exact edge resolution and fails closed before hashing;
- `docs/doctoring/postgresql-index-partition-definition-equivalence-integrity.md` records the narrower source contract and the remaining representation gap.

A fifth review, `5199853671` on exact `10b3dbb8ed58ecea065fbe5fab8bb1aa1039f2bf`, found that an admitted direct edge could still connect indexes whose structural attribute mapping could never pass PostgreSQL 18 `CompareIndexInfo()`. That function requires equal total/key attribute counts and maps each simple-column slot through the partition attribute map before expression-tree comparison.

- behavioral source RED `3a8fbb9b023d260ecda658f6685d6ccac88574e2` covers a parent key mapped to a different child partition column, a key/`INCLUDE` cardinality mismatch, and a matching positive control;
- minimal production repair `9f7320046b7a0b86ed4734ab9fec990711ff4ec5` validates key and `INCLUDE` counts, column/expression slot shape, and exact simple-column mapping after exact parent/child definition resolution and before hashing;
- `docs/doctoring/postgresql-index-partition-attribute-mapping-integrity.md` records the source contract, repair, and the intentionally unclaimed semantic remainder.

Full PostgreSQL `CompareIndexInfo()` parity is still not claimed. Collation/operator-family identity, expression-tree equality, predicates, and exclusion semantics remain the next representation-aware definition-equivalence gap. Current evidence stores operator class, not operator family, so operator-class equality must not be substituted for the PostgreSQL check; raw rendered DDL is not an acceptable semantic shortcut.

This remains source repair, not executed Rust GREEN. The active host does not provide `cargo`, `rustc`, or `rustup`, and protected ConceptWeave `main` still lacks the Product PR workflow.

## PostgreSQL 18 NOT NULL constraint identity

`ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores explicit table NOT NULL specifications as `pg_constraint.contype = 'n'` rows. The domain-separated NOT NULL family retains exact schema/relation kind/relation/column coordinates, constraint name, `convalidated`, `conenforced`, `conislocal`, signed-`int2` `coninhcount`, `connoinherit`, and a stable parent coordinate when `conparentid != 0`. Catalog OIDs are capture-time joins and never governed identity.

Retained repair lineage:

- first-class family and digest framing: `5189886354 -> 8ce7fd7c... -> 8912039b... -> c5ac66ff... -> e60cfec3...`;
- parent relation-kind validation: `5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07...`;
- signed-`int2` `coninhcount`: `5190362750 -> 28916758... -> 385bf437... -> 1b16ce65...`;
- PostgreSQL 18 PRIMARY KEY completeness: `5190470810 -> d4df54d9... -> 0cdd4fd3... -> 6c37275c...`;
- partitioned-table `NO INHERIT`: `8ef85d71... -> 7331fe06... -> e311a497...`;
- child locality/direct-ancestor tuple: `5192015894 -> 57c7f6c4... -> 03bb42ac... -> 4f032eb8...`;
- same-family parent-row resolution: `5193008264 -> b20a54c3... -> 7d62df17... -> 238a4811...`;
- corresponding parent-column integrity: `5193201137 -> 212d17e9... -> cfe1851f... -> d1012cd3...`;
- partition-parent inheritance flags: `5193490988 -> 1bd8e20e... -> e1618a20... -> c84ad970...`;
- NOT NULL enforcement base rule: `5193694445 -> 02207332... -> a47d916c... -> 948282a3... -> e35b880f...`;
- parent-constraint graph acyclicity: `5193997785 -> a8ef4462... -> 2a064395... -> 1d529c02...`;
- direct partition-parent relation integrity: `5194258395 -> 51a8f268... -> 4cbb426d... -> d7c11072... -> 610a5575...`;
- direct-parent detach-state integrity: `5194302440 -> ee12c677... -> e4d33a9d... / 4359936e... -> eaa354bb... -> 5f557299...`;
- inherited parent-validation compatibility: `5194758807 -> 6cdadfa9... -> 503e88bb... -> 1e0514e3...`;
- inheritance-origin coherence: `5195128595 -> 2940852b... -> 8a7dfae2... -> ec0e23e6...`;
- NOT NULL owning relation-kind integrity: `5195739325 -> 7c39e5ef... -> 2e3be259... -> 461959dd...`;
- relation-wide constraint-name integrity: `5195981967 -> 79d91622... -> aaf4b314... -> 0cff8f0a... -> ffad14c0...`;
- superseded foreign-table enforcement interpretation: `5196207955 -> 6d2b2289... -> 41de3e39... -> 5f786846... -> 72bdbc51...`;
- foreign-table NOT NULL validation integrity: `5196715571 -> 06839f04... -> f715dbff... -> d580256d... -> 141a632f...`;
- authoritative foreign-table enforcement correction: `5196804677 -> 4a611cc3... -> 3e982831... -> 632f678a...`;
- `NO INHERIT` origin coherence: `5197273298 -> dc6fc29b... -> 9d20fc05... -> af22bf6e...`;
- relation-partition / NOT NULL bidirectional inheritance coherence: `5198504917 -> 7b00339e... -> c708d454... -> postgresql-relation-partition-not-null-inheritance-integrity.md`.

### Direct parent relation, validation, and origin integrity

`pg_constraint.conparentid` identifies the corresponding constraint on the parent partitioned table, while `pg_inherits` separately records every direct parent-child relation with `inhrelid`, `inhparent`, and `inhdetachpending`. Every nonzero parent constraint requires an independently captured direct relation witness and exact agreement with the relation owning the resolved parent constraint. A witness without a parent constraint, a missing or mismatched witness, or `inhdetachpending=true` fails before governed hashing.

The inverse is also validated. Once `relispartition=true` and the direct partition parent are observed, every observed inheritable parent NOT NULL row must resolve to the corresponding inherited child row with the exact parent constraint coordinate. A child-local NOT NULL with no parent counterpart remains legal; relation membership never invents constraint rows.

`REL_18_STABLE` `AdjustNotNullInheritance()` refuses attaching a valid inherited parent to an existing NOT VALID child, while the inverse is allowed. Canonicalization therefore rejects only `parent.validated() && !child.validated()` after exact parent-row and corresponding-column resolution.

`conislocal=false / coninhcount=0` has neither a local origin nor an inheritance ancestor and fails closed. `connoinherit=true` requires `conislocal=true && coninhcount==0`; purely local ordinary/foreign-table NO INHERIT remains valid, while partitioned-table NO INHERIT is rejected.

### Owning relation-kind, enforcement, validation, and naming integrity

`NotNullConstraintObservation::new()` admits only ordinary tables, partitioned tables, and foreign tables. Views, materialized views, sequences, and standalone composite types fail before governed identity.

Every PostgreSQL 18 relation NOT NULL row requires `conenforced=true`; the prior foreign-table exemption was superseded by `REL_18_STABLE` implementation evidence. Foreign-table NOT NULL additionally requires `convalidated=true`; ordinary-table NOT VALID semantics remain represented where PostgreSQL supports them.

Constraint names share the owning relation's `pg_constraint` namespace. The separate NOT NULL optional family is not a second naming namespace. A NOT NULL name colliding with CHECK/PK/UNIQUE/FK evidence on the same relation fails with `DuplicateConstraintName`; the same name on another relation remains valid.

Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / RELATION_PARTITION_SOURCE_REPAIRED / RELATION_PARTITION_NOT_NULL_INHERITANCE_SOURCE_REPAIRED / INDEX_PARTITION_TOPOLOGY_SOURCE_REPAIRED / INDEX_PARTITION_VALIDITY_SOURCE_REPAIRED / INDEX_PARTITION_UNIQUENESS_SOURCE_REPAIRED / INDEX_PARTITION_NULLS_NOT_DISTINCT_SOURCE_REPAIRED / INDEX_PARTITION_ACCESS_METHOD_SOURCE_REPAIRED / INDEX_PARTITION_ATTRIBUTE_MAPPING_SOURCE_REPAIRED / INDEX_PARTITION_DEFINITION_EQUIVALENCE_OPEN / RELATION_KIND_SOURCE_REPAIRED / CONSTRAINT_NAME_SOURCE_REPAIRED / ENFORCEMENT_SOURCE_REPAIRED / FOREIGN_TABLE_VALIDATION_SOURCE_REPAIRED / INHERITANCE_ORIGIN_SOURCE_REPAIRED / NO_INHERIT_ORIGIN_SOURCE_REPAIRED / PARENT_RESOLUTION_SOURCE_REPAIRED / PARENT_COLUMN_SOURCE_REPAIRED / PARENT_NO_INHERIT_SOURCE_REPAIRED / PARENT_GRAPH_SOURCE_REPAIRED / PARTITION_PARENT_RELATION_SOURCE_REPAIRED / PARTITION_PARENT_DETACH_STATE_SOURCE_REPAIRED / PARENT_VALIDATION_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

One unchanged #46 exact head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, focused relation-partition/index-partition/NOT NULL contracts plus retained expression/generation/identity/collation/temporal/type/index contracts, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host does not provide `cargo`, `rustc`, or `rustup`. Protected/default ConceptWeave `main` still lacks the Product PR workflow. Source-level RED fixtures and causal repairs are not local Rust RED/GREEN or hosted Product acceptance. Draft/Ready cycling, synthetic status, copied central workflows, manual/no-op reruns, self-approval, review dismissal, force-push, destructive rebase, administrator bypass, and gate weakening remain prohibited.

## Central Product-CI and review owners

Central workflow ownership remains outside ConceptWeave; central evidence never transfers to a ConceptWeave leaf head. Protected `.github/main` is `91be6442906c7b6b4f600272c953699708394327`.

- `.github#2106@9defd52f4a3b42d6a63a9520d6da82224d8c864d` is OPEN Draft / mergeable. Runtime Quality `34825970200` and SAST Semgrep `34825970371` are success; Python Security `34825970282`, Security Scan `34825970259`, and CodeQL PR `34825970251` remain queued. Its owner-local cross-repository traceability repair remains required; predecessor evidence does not transfer.
- `.github#2170@c346b8324fa23e23d4007799d26ad3a8ac6ae4c3` is OPEN Ready / mergeable. Runtime Quality `34826735972` and SAST Semgrep `34826735939` are success; Security Scan `34826736000`, Python Security `34826735889`, and CodeQL PR `34826735991` remain queued.
- `.github#2079@2b2f348d823d48f81ea6c1f7e36d0870ce35e5c4` is OPEN Ready / mergeable. SAST Semgrep `34826683831` is success; CodeQL PR `34826683838`, Security Scan `34826683861`, and Python Security `34826683797` remain queued. A fresh Required OpenCode generation and qualifying independent review must bind the same head.
- ConceptWeave #35 exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` remains the Product-workflow bootstrap. Its historical CodeQL PR run is terminal failure; historical Semgrep and Security success do not authorize merge. Fresh compatible unchanged-head acceptance remains required before normal landing.

No ConceptWeave leaf copies or edits central owner sources. #2106 proceeds through owner-local repair and fresh exact-head gates; #2170/#2079 must finish their own exact-head acceptance. Only after central owners settle and #35 lands normally can one unchanged #46 head obtain meaningful repository-owned Product acceptance. The complete #46 delta then flows ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter uses a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolves least-privilege credentials only through the authorized source/policy binding, and uses bounded `REPEATABLE READ READ ONLY` catalog capture. It never keeps an explicit database transaction or lock open while waiting on an LLM or long external computation.

For every bounded relation, the adapter captures `pg_class.relispartition`. Positive table membership resolves direct `pg_inherits.inhparent` and `inhdetachpending`; the parent must be an observed partitioned table, detach-pending state fails closed, and the direct relation graph must be acyclic. Generic table inheritance remains separate.

For every observed nested index, the adapter captures `pg_class.relkind` (`i` ordinary index or `I` partitioned index), index `relispartition`, `pg_index.indisvalid`, `pg_index.indisunique`, `pg_index.indnullsnotdistinct`, the exact access method, ordered key/`INCLUDE` attribute roles and simple-column names, and any direct index `pg_inherits` row in the same bounded catalog snapshot. A positive index partition resolves the exact parent index coordinate, requires `inhdetachpending=false`, and must agree with the independently observed direct parent of its owning table partition. Catalog OIDs are capture-time joins only; semantic identity uses resolved schema/relation-kind/relation/index coordinates. The adapter must not infer attachment merely because the owning table is a partition or because definitions look similar. An explicitly valid parent partitioned index requires a real attached child-index edge on every direct table partition; invalid or unobserved-validity parents may retain staged local indexes. Any admitted direct edge must preserve parent/child `indisunique`, `indnullsnotdistinct`, access method, key/`INCLUDE` cardinality and role boundary, simple-column/expression slot shape, and exact mapped simple-column identity.

PostgreSQL requires an `ALTER INDEX ... ATTACH PARTITION` target to have an equivalent definition. The authoritative governed attachment evidence is the exact index `pg_inherits` edge created by PostgreSQL after that check, but ConceptWeave also rejects contradictions in mandatory properties already modeled. Broader equivalence must follow PostgreSQL `CompareIndexInfo()` semantics rather than raw index names or rendered-DDL equality; collation/operator-family identity, expression-tree equality, predicates, and exclusion semantics remain explicitly open.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull`, bounded `pg_constraint.contype='n'` rows including exact `convalidated`, `conenforced`, `conislocal`, signed `coninhcount`, and `connoinherit`, parent constraints, owning `pg_class` rows, the complete standard relation constraint-name inventory, and required direct `pg_inherits` witnesses inside the same catalog snapshot. `conkey` resolves to exactly one bounded column. The owning `pg_class.relkind` must resolve without coercion to ordinary table (`r`), partitioned table (`p`), or foreign table (`f`).

Every NOT NULL row requires `conenforced=true`; foreign-table NOT NULL additionally requires `convalidated=true`. `conislocal=false / coninhcount=0` fails closed. `connoinherit=true` requires `conislocal=true / coninhcount=0`; purely local ordinary/foreign-table NO INHERIT remains valid, while partitioned-table NO INHERIT is rejected. A nonzero `conparentid` requires a real parent-constraint join, parent `relkind='p'`, `conislocal=false`, `coninhcount=1`, `connoinherit=false`, corresponding constrained columns, compatible parent/child validation state, an acyclic parent graph, and an independent stable direct `pg_inherits` witness with `inhdetachpending=false`.

When relation-partition evidence is observed with the complete NOT NULL family, validation is bidirectional: existing child `conparentid` must agree with the relation-level direct parent, and every inheritable NOT NULL row on that direct partitioned-table parent must have the corresponding inherited child row/link. The adapter never synthesizes the missing side from the other family.

Missing, unauthorized, contradictory, origin-less, unsupported-relation-kind, duplicate, cross-family-name-colliding, cross-column, cyclic, invalid parent/child validation, NOT ENFORCED NOT NULL, foreign-table NOT VALID NOT NULL, inherited-ancestry NO INHERIT, partitioned-table NO INHERIT, detach-pending, missing inherited partition linkage, incomplete index topology, wrong index parent, explicitly valid partitioned-index topology with a missing attached direct child, attached index uniqueness mismatch, attached `NULLS NOT DISTINCT` mismatch, attached access-method mismatch, attached index attribute-mapping mismatch, or partially resolved evidence fails closed before immutable snapshot construction.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_inherits*. https://www.postgresql.org/docs/18/catalog-pg-inherits.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table Partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER INDEX*. https://www.postgresql.org/docs/18/sql-alterindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-createforeigntable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: tablecmds.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/tablecmds.c
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: index.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
