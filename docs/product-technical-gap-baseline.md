# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This is the code-current ConceptWeave product/technical gap authority for the active Source Observation lane. Exact SHAs, reviews and runs are evidence snapshots, not mutable dependencies. A moved head invalidates predecessor execution evidence unless the successor itself produces equivalent evidence.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. Source-system business truth remains with its canonical owner.

- `semantic-data-portal`: catalog, governance and consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave owns authorization of ConceptWeave proposal/base/semantic resources.
- consumers: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL and mutable sibling-head dependencies are invalid integration mechanisms.

## Live stack and protected prerequisites

Fresh authority entering this update:

- protected/default ConceptWeave `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`;
- Product-CI bootstrap #35: `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft, still waiting on central protected-workflow settlement;
- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft;
- Source Observation #6: `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft;
- representation-v3 parent #45: `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6;
- representation/index successor #46: `c10a5d2cd5f1906c4318f1fb4d7aaaabd6bef63f`, OPEN Draft/mergeable and temporal-value-type RED-active; last production-source repair remains `d60de513d559c6359d55149ddc8707d1520cd6b0`.

Protected central `.github/main` is freshly verified at `cb0872c9a20d5584703dffacca65c096fc034c6c`; `.github#2051@558693e0333e48012beea142f739bc634b0674a7` remains Draft on historical `main@7fd571db...`, with `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` stacked on it. The central owner must land a backward-compatible handler, ordinary/non-force reconcile those PRs onto current protected main and obtain terminal GREEN before unchanged #35 can receive fresh acceptance and normal merge.

Protected/default ConceptWeave `main` still lacks repository-local `.github/workflows` authority for the Product `pull_request` workflow. A PR branch cannot bootstrap its own default-branch trigger. Draft/Ready toggles, no-op commits and manual retriggers are not acceptance evidence.

## Source Observation bounded context

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, exact evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

Historical v2 evidence is frozen. Its digest domain and `/schemas/{schema}/tables/{table}` coordinate vocabulary retain identical meaning. New PostgreSQL facts use successor contracts; new identity-bearing evidence requires an exact verified receipt coordinate without reinterpreting predecessor paths.

## PostgreSQL 18 representation-v3 state

Preserved source repairs include:

- relation-kind-aware owner/child coordinates and exact qualified type resolution;
- relation-backed composite row-type resolution for non-sequence modeled relations;
- shared schema-local `pg_type` namespace checks across domains, enums, relation row types and observed true arrays;
- exact `pg_type.typarray`/`typelem` true-array observations without underscore-name inference, OID identity or `search_path` dependence;
- one element -> one associated true array, no observed array-of-array relation, same-schema reciprocity and observed-empty versus unobserved array state;
- exact array evidence receipts through `ArrayTypeLocation`/`ArrayTypeSourceReceipt`, binding the public array-aware digest while leaving previous `SchemaObjectLocation` meanings frozen;
- relation-scoped index evidence for key/`INCLUDE` layout, expression keys, per-key collation/operator class/opaque `indoption`, operator-class parameters, `NULLS NOT DISTINCT`, material `pg_index` flags, `pg_class.reloptions`, resolved tablespace state and reconstructed-definition/comment provenance;
- schema-local `pg_class` namespace consistency across owning relations and nested indexes;
- local index admission only on ordinary tables, partitioned tables and materialized views;
- represented table constraints on ordinary/partitioned tables, CHECK-only on foreign tables, and no modeled table constraints on views/materialized views/sequences/standalone composite-type relations;
- at most one represented primary key per relation and exact `nullable = false` evidence for every primary-key column;
- explicitly observed PRIMARY KEY/UNIQUE timing as `NotDeferrable`, `InitiallyImmediate` or `InitiallyDeferred`, with exact relation/constraint coordinates, complete inventory, observed-empty versus unobserved state and a separate `conceptweave.postgres_schema_snapshot.v3.constraint_timings.v1` digest layer;
- `ConstraintTimingObservation` and `ConstraintPeriodObservation` themselves reject non-table relation kinds, so impossible view/materialized-view/foreign-table/sequence/composite-type catalog coordinates are not representable before aggregate admission; ordinary and partitioned tables remain valid;
- explicitly observed PRIMARY KEY/UNIQUE timing binds to same-relation/same-name supporting-index evidence and requires uniqueness, PK/non-PK role, `indimmediate`, exact ordered simple key columns, non-partial shape and explicitly observed UNIQUE null-treatment coherence before immutable timing evidence is admitted;
- when a represented PRIMARY KEY/UNIQUE backing index carries `pg_index.indisexclusion = true`, its exact observed access method must be `gist`; contradictory non-GiST catalog shapes fail closed as `constraint_backing_index` without inferring `pg_constraint.conperiod`;
- explicit PostgreSQL 18 `pg_constraint.conperiod` evidence as a separate domain-separated family covering every represented PRIMARY KEY, UNIQUE and FOREIGN KEY constraint, preserving unobserved versus observed `false` versus observed `true` without deriving truth from index shape or reconstructed DDL;
- represented key-constraint `conperiod` is checked for coherence with already-observed same-name backing-index exclusion/GiST facts, and same-snapshot PERIOD foreign keys require a non-PERIOD equality-key prefix plus an explicitly observed referenced `WITHOUT OVERLAPS` PK/UNIQUE on the exact referenced columns;
- a represented PERIOD foreign key additionally requires observed `ForeignKeyReferenceBehavior` with exact `NO ACTION` for both update and delete; missing action evidence and `RESTRICT`/`CASCADE`/`SET NULL`/`SET DEFAULT` fail closed as `constraint_period_action`, while ordinary non-temporal foreign keys retain their existing behavior;
- when that PERIOD foreign key targets a relation inside the same bounded snapshot, the referenced temporal PK/UNIQUE must also have exact observed `ConstraintTimingObservation::NotDeferrable`; missing timing evidence or either deferrable state fails closed as `constraint_period_reference_timing`, and GiST/`indimmediate` is never substituted for `pg_constraint.condeferrable`/`condeferred` truth;
- governed v3 foreign-key evidence admits PostgreSQL 18 `MATCH SIMPLE` and `MATCH FULL` but rejects reserved-yet-unimplemented `MATCH PARTIAL` as `foreign_key_match_type`; frozen shared/v2 vocabulary and digest tags remain unchanged for historical reproduction.

The next unresolved P1 is the temporal final-column type contract: explicit `conperiod=true` currently does not prove that the final `WITHOUT OVERLAPS`/`PERIOD` column resolves to PostgreSQL range or multirange semantics. PostgreSQL 18.4+ also permits a domain over range/multirange, so the repair cannot be a direct-only `typtype = r/m` check.

### Key-constraint timing lineage

PostgreSQL 18 stores constraint timing in `pg_constraint.condeferrable`/`condeferred`. Source repair is preserved as:

- finding review `5181035661` on `ccd7e245010dab994a6540fe2de5e15db3f742ea`;
- behavioral RED `0a11a3285e7d5f371494656eac890b5b3a632796`, `constraint_timing_contract.rs`;
- timing VO `85d462795a462abd11e4c0782a80e0d82a5c7727`;
- domain-separated snapshot integration `929451e41b0f977f6bbb01299b843c4bfa2fbc68`;
- completeness regression `f0e97b4de53db9bdf6326abad36bfdfc4bf9b12e` and admission repair `72bdb7752483fc974a09fae0443cbd5ab231e919`;
- canonical coordinate repair `8a6092733c2417fd3eae88bd279e0dbdcc880e85`;
- code-current timing baseline `9020f3620afbe932aec7a0ca6f1bb3dbf5b0d316`.

The timing family is additive. It does not mutate `PrimaryKeyObservation`/`UniqueConstraintObservation` shared with frozen v2. Existing constraint receipt paths remain stable and bind the public digest, including timing when explicitly observed.

Review `5183681930` on `45a8b2ff442bef81f590ae61e8d4dbadb84035e8` later found that the original positive timing fixtures had become stale after supporting-index admission was strengthened: they supplied timing for PK/UNIQUE constraints without the now-required same-name backing indexes, so positive timing assertions would fail on `constraint_backing_index` before exercising timing identity. `76ff202412de9c09b9ebfca60f0a4f1d37eef946` added coherent backing-index fixtures and `2656b7508fe04e0b325245e49f63df57f941fbd4` removed a borrowed-coordinate move hazard. This repairs the test contract rather than weakening the production invariant.

### Key constraint to supporting-index coherence

Review `5181223180` on exact `9020f3620afbe932aec7a0ca6f1bb3dbf5b0d316` found that v3 modeled both sides of PostgreSQL's key-constraint/index relationship without binding them. `pg_constraint.conindid` identifies the index supporting PRIMARY KEY/UNIQUE, PostgreSQL constraint-owned indexes share the constraint name, and `pg_index.indimmediate` distinguishes immediate uniqueness enforcement from deferrable constraint support.

Initial behavioral RED `c0cec50ed29e2435cf1552302e166a7ff7494924` added `constraint_backing_index_contract.rs`. Fixture review then caught a non-canonical UTC provenance timestamp; ordinary-forward refinement `8efc1e5d24c3fb4670237515f3f760b73ee5303e` changed it to exact `Z` UTC so the test reaches the intended backing-index invariant rather than failing on provenance validation first.

Production repair `40337bc086f5b12b981c0f602bfc6413287644ea` extended `canonicalize_constraint_timings` so every explicitly observed PRIMARY KEY/UNIQUE timing coordinate resolves one same-relation/same-name represented index with observed catalog flags. Admission requires PK/UNIQUE role, uniqueness and `indimmediate` coherence. Exact-head static review `5181883061` found that causal repair aligned with the corrected first RED.

A second exact-head review, `5182343267`, found that role/timing coherence still admitted impossible PostgreSQL enforcing-index shapes. Same-name index evidence could key different columns or order, use an expression key, carry a partial predicate, or disagree with an explicitly observed UNIQUE `NULLS [NOT] DISTINCT` state. Behavioral RED `a54ea984d72b5b0c1609e54826efd6e70101cc7b` added `constraint_backing_index_shape_contract.rs` with wrong-order, expression, partial, null-treatment mismatch and coherent `INCLUDE` cases.

Production repair `3c962108d09a8c02f3349ad79285ea8175ec1fa8` extends the same public aggregate admission seam without changing frozen v2 or original v3 digest identity. It requires:

- PRIMARY KEY -> `is_unique = true`, `indisprimary = true`;
- UNIQUE -> `is_unique = true`, `indisprimary = false`;
- `NotDeferrable` -> `indimmediate = true`;
- `InitiallyImmediate` or `InitiallyDeferred` -> `indimmediate = false`;
- backing-index key count and exact ordered simple-column names equal the constraint key; `INCLUDE` payload remains outside that key comparison;
- a constraint-owned backing index is not partial and therefore carries no predicate;
- when UNIQUE null comparison behavior is explicitly observed, the index's observed `NULLS [NOT] DISTINCT` state is present and exactly equal;
- missing or contradictory support fails closed as `constraint_backing_index`.

Static source review `5182395681` found the +25/-0 one-file repair causally aligned with the RED. `conindid` remains an adapter-local join coordinate, not governed semantic identity.

### PostgreSQL 18 temporal backing-index coherence

Finding review `5182818948` identified the next catalog-coherence gap. PostgreSQL 18 `WITHOUT OVERLAPS` PRIMARY KEY/UNIQUE constraints are backed by GiST rather than ordinary B-tree indexes, while `pg_constraint.conperiod` is the explicit catalog fact identifying `WITHOUT OVERLAPS` key constraints and `PERIOD` foreign keys. Index facts may be checked for coherence but must not be promoted into inferred `conperiod` semantic truth.

Behavioral RED `b7c8c11eee1444a96523d265f31277570bcc68b7`, refined ordinary-forward to canonical UTC at `4c863d0de0236e0ed6a1b8a3ddf2a8719c610dc9`, added `constraint_temporal_index_contract.rs`. It requires PRIMARY KEY/UNIQUE backing indexes with `indisexclusion = true` to reject exact observed `btree`, while preserving temporal `gist` and ordinary non-exclusion `btree` controls.

Production repair `5f00f911359451a2021f03392e8d0439e88c3fc0` minimally extends `canonicalize_constraint_timings`: `indisexclusion = true` now requires exact observed `access_method = gist`; unobserved or contradictory access methods fail closed through the existing `constraint_backing_index` invariant. Ordinary-forward `7400d2d3060ee7007d0346850af38eb468b3d422` only restored the canonical trailing newline introduced by the contents write. Exact-head source review `5183334477` verified the causal delta and retained the explicit `conperiod` boundary.

### PostgreSQL 18 `conperiod` and PERIOD representation

PostgreSQL 18 exposes `pg_constraint.conperiod` as the authoritative catalog fact: `true` means `WITHOUT OVERLAPS` for PRIMARY KEY/UNIQUE and `PERIOD` for FOREIGN KEY. `WITHOUT OVERLAPS` key constraints use exclusion-style GiST backing indexes, while a PERIOD foreign key requires at least one ordinary equality-key column before its final period column and must target a referenced PRIMARY KEY/UNIQUE declared `WITHOUT OVERLAPS`.

Review `5183704353` anchored behavioral RED `39bdccbb9d3cf8a26f46ba05f6ce59896f390f06`, `constraint_period_contract.rs`. The contract requires unobserved/false/true distinction, complete PK/UNIQUE/FK inventory, exact coordinate validation, key constraint/index temporal coherence, PERIOD-FK minimum shape, same-snapshot referenced temporal-key coherence, and input-order-independent identity. `69d4c734c5954e3ccf37b6965ca65ff238aa45b7` introduced the exact `ConstraintPeriodObservation` value object without changing the frozen shared PK/UNIQUE/FK types.

Production repair `3f2ecba28fdd5742de5af1cac68227767baadbed` integrates an additive `conceptweave.postgres_schema_snapshot.v3.constraint_periods.v1` digest layer and one-time `with_observed_constraint_periods(...)` admission on the public v3 aggregate. It never reconstructs authoritative `conperiod` from `indisexclusion`, GiST, OIDs or rendered DDL; already represented index facts are used only as a contradiction check. `c1abb9189d9f7c9539a65f87be654a6fe9ddbbc7` keeps the new contract Clippy-style clean.

### PostgreSQL 18 PERIOD foreign-key referential actions

Finding review `5184007447` on `d94644e9ee542a0cec5c7902915b47dee13e209e` found that explicit `conperiod=true` foreign keys still admitted impossible referential-action states because `canonicalize_constraint_periods()` did not inspect `ForeignKeyReferenceBehavior`. PostgreSQL 18 supports `NO ACTION` for temporal foreign keys but does not support `RESTRICT`, `CASCADE`, `SET NULL`, or `SET DEFAULT` there.

Behavioral RED `03e4443b5834383f4d25a8e83786cccb62e003be` added update/delete rejection coverage. Ordinary-forward `60b59db961ee35a0a0d5de91422ca68612afb8eb` strengthened the evidence boundary: because the same `pg_constraint` row exposes referential-action codes, explicit PERIOD governance with missing behavior evidence fails closed rather than silently inferring defaults. `1fee67a5223ecc4f1acb13536204d311684a6a9d` adds primary-source doctoring at `docs/doctoring/source-observation-temporal-foreign-key-actions.md`.

Production repair `1a77e006553a39e3752ee3e9f08c57e9160dac78` minimally extends the PERIOD-FK branch: `reference_behavior()` must be observed and both update/delete actions must equal `ForeignKeyAction::NoAction`; otherwise admission fails as `constraint_period_action`. Retained positive `constraint_period_contract.rs` fixtures were ordinary-forward repaired at `c8947613d665b5061ea445d1dfd6a7165447483f` to carry explicit NO ACTION evidence. The contents-API rewrite also exposed an EOF-newline cleanup; the first cleanup commit `95d3720625bd029b9b6bc46b1841faac2058cb66` accidentally changed the public connection-policy accessor, and immediate causal repair `6975d94a51d5dc4793ccfd1d39a4c2a849195ace` restored `self.inner.connection_policy_binding()`. Comparing `c894761...` to `6975d94...` leaves only the intended canonical trailing-newline normalization. Exact-current static review `5184074266` records this correction chain.

### PostgreSQL 18 PERIOD foreign-key referenced-key timing

Finding review `5184299133` on exact `bff3455e9ddd256a7aa9e1ba7eaa6466151b9e82` found that same-snapshot PERIOD references could still authorize a referenced temporal key whose `pg_constraint.condeferrable`/`condeferred` family was unobserved or explicitly deferrable. PostgreSQL 18 requires an explicit referenced column list to resolve to a non-deferrable UNIQUE/PRIMARY KEY, while PERIOD additionally requires the referenced PK/UNIQUE to be declared `WITHOUT OVERLAPS`.

Behavioral RED `6c77cb6fb1664cd7ffeb517ad7bcd85382ebd825` added `constraint_period_reference_timing_contract.rs`, covering unobserved referenced-key timing, explicitly deferrable referenced-key timing, and the exact NOT DEFERRABLE positive control. Review `5184303271` records that the live production seam had no timing-family input and therefore admitted the first two invalid states.

Production repair `fa21b47653192af83627ac77d14c9141f4419cbd` passes the already-observed key-timing family into `canonicalize_constraint_periods()` and requires the exact referenced temporal key coordinate to resolve `ConstraintDeferrability::NotDeferrable`; missing or deferrable timing fails closed as `constraint_period_reference_timing`. Initial retained-fixture updates `b501b003fbcbfe612f92aa65d83a7fd82cedb68a` and `8b36c7a67f8a90b24ad2f08c02ead23374dc4c94` supplied the new timing evidence.

Static review `5184320142` then found a fixture-only compile regression: the integration tests had called private `with_observed_constraint_timings()`. The public API was not widened for tests. Ordinary-forward corrections `90d4ba255ebcc56f4f6eed76b4e6d0d4be5ced15`, `f402b0e39d3a7125476f471edac79fa776e347f2`, and `95074fdff8f3e66c4ed4e54215bd4f59f2cf3e86` move the action, RED, and retained period fixtures onto `new_with_constraint_timings(...)`. Doctoring was correspondingly currentized at `194612f3ea5586484980f25f51cba133b5b187d1`.

This repair deliberately does not infer timing from GiST, exclusion state, or `pg_index.indimmediate`, and it does not require timing evidence for a temporal key that is not acting as an in-snapshot foreign-key reference target. The slice is source-repaired and acceptance-pending, not native/Product GREEN.

### Constraint catalog relation-kind coordinates

Finding review `5184492366` on exact `db879504dce68dc532a8b09ed2d4fec71f68d244` found that the public successor timing/period value objects still admitted relation kinds on which PostgreSQL 18 cannot own the corresponding table-constraint families. Although aggregate admission later rejected many such combinations indirectly, impossible source evidence remained representable before aggregate construction.

Behavioral RED `ccf1a0558389dcf6b7d5af456c83f58f018a8704` added `constraint_catalog_relation_kind_contract.rs`. Production repairs `635a9ea05af9daffd78a469bc72e69df4633317e` and `67da1f9d7ac34e9aa75eb92696c7ec695553e481` restrict `ConstraintPeriodObservation` and `ConstraintTimingObservation` to `RelationKind::Table` and `RelationKind::PartitionedTable`. Primary-source doctoring `8581766af558a596c8548fde11742bea829ce90a` records the decision, PostgreSQL 18 `CREATE TABLE`/`CREATE FOREIGN TABLE`/`pg_constraint` authority and rejected alternative. Exact-head static review `5184498366` found the source delta causal and isolated; native/Product acceptance remains pending.

### PostgreSQL 18 foreign-key match support

Finding review `5184685945` on `5d70535e8fa420ca1814c0fa9287d715bf3f32e7` found a catalog/runtime-support mismatch: the shared model preserves `ForeignKeyMatchType::Partial` and v3 admitted it, while PostgreSQL 18 `CREATE TABLE` explicitly says `MATCH PARTIAL` is not implemented even though `pg_constraint.confmatchtype` reserves the `p` code.

Behavioral RED `e367b81bd1af07ea28f3a044a92e8dd39239048e` adds `foreign_key_match_contract.rs`: governed v3 must reject `Partial` as `foreign_key_match_type`, while `Simple` and `Full` remain positive controls. Primary-source doctoring `653da9f34345471178e99ca45c84835c92d5166d` records the distinction between reserved catalog vocabulary and executable PostgreSQL support.

Production repair `d60de513d559c6359d55149ddc8707d1520cd6b0` adds only a v3 owner-level admission check in `validate_schema_relation_invariants`; predecessor comparison is +12/-0 in `lib.rs`. Static review `5184707338` found the causal delta isolated. The shared enum and frozen-v2 tag remain intact, so historical evidence can still be decoded/reproduced without allowing a new governed v3 snapshot to promote an unimplemented runtime state.

### PostgreSQL 18 temporal period-value type gap

Finding review `5184917000` on `5051f8d2262851ebfd9119518ceb633d92bbef40` identified that `canonicalize_constraint_periods()` validates explicit `conperiod`, GiST/exclusion coherence, PERIOD-FK action/reference/timing semantics and shape, but never proves the final temporal column's PostgreSQL type semantics. Behavioral RED `ef86be21c06477d210a9725874f10a969f4f37a9` adds `constraint_period_type_contract.rs`: an otherwise coherent temporal primary key using scalar `pg_catalog.text` as its last column must fail closed as `constraint_period_column_type`.

Current PostgreSQL 18.x materially constrains the fix. PostgreSQL 18.4 repaired `WITHOUT OVERLAPS` to allow a domain over a range or multirange. Exact-head review `5185079749` therefore rejects a direct-only `pg_type.typtype IN ('r','m')` implementation. Ordinary-forward `558fb3c880b03dcac6456d168b460c4619dbac86` adds a retained domain-over-`tstzrange` positive control, and `46a77be635ece833611ee677664d15d7e35627de` updates `docs/doctoring/source-observation-temporal-period-value-type.md` with the current source contract.

The minimum causal production repair now requires a versioned observed type-kind family that preserves exact qualified type coordinates, direct `pg_type.typtype`, and exact qualified `pg_type.typbasetype` for domains; `pg_range` may supply reciprocal range/multirange evidence while raw OIDs remain adapter-local joins. Temporal admission must resolve the final local value—and same-snapshot referenced temporal key where applicable—directly or through an explicitly observed domain chain to range/multirange, fail closed on missing/cyclic/scalar evidence, retain direct user-defined range/multirange coordinates, and keep `pg_constraint.conperiod` as the sole authority for whether temporal semantics were declared. This family must be domain-separated in successor identity and distinguish unobserved from explicitly observed evidence.

`cf5aa6d1b3239b2b34eb9707c425708d7f2c86eb -> c10a5d2cd5f1906c4318f1fb4d7aaaabd6bef63f` is ordinary-forward, 4 commits ahead / 0 behind, with net changes only in the temporal-type contract test and doctoring. Production source remains unrepaired for this P1. Review `5185086381` records the exact-current boundary. No native/Product GREEN is claimed.

### Preserved high-value repair lineage

- `5176683395 -> 908d1b10e63254aa4cb85eda9c078ee79f950c0c -> 68efaf0fc735faa74202b420df9bf031069e5116`: reject impossible non-unique + `NULLS NOT DISTINCT=true` state.
- `5176751905 -> 0e6c7314bdd36f313abb6c09231d1e1271383ce0 -> f24242708cf82f4405c12ed2b8fa7153b1c58b24`: preserve material `pg_index` flags.
- `f796bf51110863e98e5d4d16a7f7bbea689b4705 -> 70455fdfbc28dffc8f306619e806b79ca9678693 -> 3eab943ad85584b535417b770f05c67192a7a081`: preserve canonical `pg_class.reloptions` and observed-empty state.
- `5176975409 -> 1a47d6b16838006e5f7a75407e69464740f368b1 -> 5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce -> b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`: exact resolved index tablespace identity.
- `5177885832 -> b820c7b80b6c95e6ae419515882d850d79e578ec -> 1bede23588956c11500beb9a54f1f617ceb5429f`: schema-local `pg_class` namespace enforcement.
- `5178743508 -> f81ae51af614a39e648a9c314776ba79bf99d64e -> a8e9fac16896e3e6d48ec5bc20cafae8c855da39`: reject indexes on non-indexable relation kinds.
- `5179341855 -> 089df3d4d59a45cd87afd30c4d86390a96c8c674 -> 50b8d05e286181a3d39e88186116a4af173a4285`: relation-kind/constraint admission.
- `5180058119 -> eca8adb2e5667048c220a37ad863971a8457e9d3 -> 66130c568705092ffd4dabc9bf56bf2a8c88da3a`: exact relation-backed composite row-type identity.
- `5180207753 -> 51075e48da8da3059cb6ec764f8c45b88b1f933c -> 8d8bd115f080fbcc11fb2be755f41436ba3b8886 -> 21c216aae61009d54dcb4a593f502fdd8654598d -> a7d20d90f1de5a4b94ac23e1d22737be3c4d9c1d -> da6fe0fd51431fa0f902566a9d8d3fac6bd8caf9`: exact true-array identity, reciprocity, digest and receipt coordinates.
- `5180938301 -> 65be0a02da340ecc4ab96b34b8a5325d8f8b197c -> 27de693b18d6cb1936e572a44e91c5ace2b7209a`: primary-key cardinality/nullability invariant.
- `5181223180 -> c0cec50ed29e2435cf1552302e166a7ff7494924 -> 8efc1e5d24c3fb4670237515f3f760b73ee5303e -> 40337bc086f5b12b981c0f602bfc6413287644ea -> 5181883061`: key constraint/supporting-index role and timing coherence.
- `5182343267 -> a54ea984d72b5b0c1609e54826efd6e70101cc7b -> 3c962108d09a8c02f3349ad79285ea8175ec1fa8 -> 5182395681`: key constraint/supporting-index shape and null-treatment coherence.
- `5182818948 -> b7c8c11eee1444a96523d265f31277570bcc68b7 -> 4c863d0de0236e0ed6a1b8a3ddf2a8719c610dc9 -> 5f00f911359451a2021f03392e8d0439e88c3fc0 -> 7400d2d3060ee7007d0346850af38eb468b3d422 -> 5183334477`: temporal key exclusion/access-method coherence.
- `5183681930 -> 76ff202412de9c09b9ebfca60f0a4f1d37eef946 -> 2656b7508fe04e0b325245e49f63df57f941fbd4`: repair timing positive fixtures after the supporting-index invariant became mandatory.
- `5183704353 -> 39bdccbb9d3cf8a26f46ba05f6ce59896f390f06 -> 69d4c734c5954e3ccf37b6965ca65ff238aa45b7 -> 3f2ecba28fdd5742de5af1cac68227767baadbed -> c1abb9189d9f7c9539a65f87be654a6fe9ddbbc7`: explicit `pg_constraint.conperiod`/PERIOD representation and admission.
- `5184007447 -> 03e4443b5834383f4d25a8e83786cccb62e003be -> 60b59db961ee35a0a0d5de91422ca68612afb8eb -> 1fee67a5223ecc4f1acb13536204d311684a6a9d -> 1a77e006553a39e3752ee3e9f08c57e9160dac78 -> c8947613d665b5061ea445d1dfd6a7165447483f -> 95d3720625bd029b9b6bc46b1841faac2058cb66 -> 6975d94a51d5dc4793ccfd1d39a4c2a849195ace -> 5184074266`: explicit PERIOD-FK action evidence and PostgreSQL-valid NO ACTION admission, including immediate correction of the contents-write accessor regression.
- `5184299133 -> 6c77cb6fb1664cd7ffeb517ad7bcd85382ebd825 -> 5184303271 -> fa21b47653192af83627ac77d14c9141f4419cbd -> b501b003fbcbfe612f92aa65d83a7fd82cedb68a -> 8b36c7a67f8a90b24ad2f08c02ead23374dc4c94 -> 5184320142 -> 90d4ba255ebcc56f4f6eed76b4e6d0d4be5ced15 -> f402b0e39d3a7125476f471edac79fa776e347f2 -> 95074fdff8f3e66c4ed4e54215bd4f59f2cf3e86 -> 194612f3ea5586484980f25f51cba133b5b187d1`: PERIOD-FK referenced temporal key requires exact observed NOT DEFERRABLE timing; integration fixtures use the public timing constructor.
- `5184492366 -> ccf1a0558389dcf6b7d5af456c83f58f018a8704 -> 635a9ea05af9daffd78a469bc72e69df4633317e -> 67da1f9d7ac34e9aa75eb92696c7ec695553e481 -> 8581766af558a596c8548fde11742bea829ce90a -> 5184498366`: direct successor timing/period value objects reject relation kinds that cannot own those PostgreSQL table-constraint families.
- `5184685945 -> e367b81bd1af07ea28f3a044a92e8dd39239048e -> 653da9f34345471178e99ca45c84835c92d5166d -> d60de513d559c6359d55149ddc8707d1520cd6b0 -> 5184707338`: PostgreSQL 18 governed v3 rejects unimplemented `MATCH PARTIAL` while retaining shared historical vocabulary.
- `5184917000 -> ef86be21c06477d210a9725874f10a969f4f37a9 -> 5185079749 -> 558fb3c880b03dcac6456d168b460c4619dbac86 -> 46a77be635ece833611ee677664d15d7e35627de -> 5185086381`: temporal final-column scalar RED plus PostgreSQL 18.4+ domain-over-range compatibility control and current type-kind/domain-base repair contract.

## Acceptance still required

The current #46 lineage is **behavioral RED-active**, not source/native/Product GREEN. Before any Ready/adoption/merge claim, the temporal type-kind/domain-base production repair must land and one unchanged exact successor must produce:

- repository-pinned Rust 1.98 `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including frozen-v2 and retained v3 index/type/array/constraint contracts plus `foreign_key_match_contract`, `constraint_catalog_relation_kind_contract`, `constraint_timing_contract`, `constraint_backing_index_contract`, `constraint_backing_index_shape_contract`, `constraint_temporal_index_contract`, `constraint_period_contract`, `constraint_period_action_contract`, `constraint_period_reference_timing_contract`, `constraint_period_type_contract`, `primary_key_invariants_contract`, `array_type_identity_contract`, `array_type_digest_contract`, `array_type_schema_contract` and `array_type_receipt_contract`;
- temporal type coverage proving scalar rejection, built-in and direct user-defined range/multirange admission from exact catalog evidence, domain-over-range/multirange admission, domain-over-scalar rejection, missing/cyclic evidence rejection, and digest order stability;
- rustdoc/doc tests, release build and owned production docstring/test/edge-case coverage;
- applicable Product/security/dependency/review workflows terminal on the same exact head.

No predecessor GREEN transfers. Draft/bot-only status is not acceptance.

## Concrete PostgreSQL adapter boundary

No transport is admitted before representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must:

- use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate and passing dependency/SBOM review;
- resolve least-privilege credentials only for the authorized source key and immutable policy binding;
- use one explicit `REPEATABLE READ READ ONLY` catalog transaction and one non-resetting connect/query/cancellation budget;
- use catalog OIDs only for adapter-local joins, then cross the ACL with exact names/coordinates;
- resolve `pg_class.reltype`/`pg_type.typrelid`, `pg_type.typarray`/`typelem`, exact relation/type namespaces, direct `pg_type.typtype`, exact qualified domain `typbasetype`, and `pg_range.rngtypid`/`rngmultitypid` relationships required by the released successor contract;
- collect complete `pg_constraint` PK/UNIQUE timing, `conindid` support relationships, exact `conperiod`, `confupdtype`, `confdeltype`, and `confmatchtype` values for every represented relevant constraint before crossing the ACL;
- reject `confmatchtype = 'p'` for governed PostgreSQL 18 v3 evidence even though the catalog vocabulary reserves it, because PostgreSQL 18 does not implement `MATCH PARTIAL`; preserve SIMPLE/FULL exactly;
- validate `condeferrable`/`condeferred` against `pg_index.indimmediate`, PK against `indisprimary`/`indisunique`, exact constraint/index names, exact key columns/order, no partial predicate and observed UNIQUE null treatment rather than persisting OIDs;
- where a represented key constraint has backing `indisexclusion = true`, require exact observed GiST access method while separately carrying authoritative `pg_constraint.conperiod`; never synthesize `conperiod` from the index;
- for temporal constraints, resolve the exact final constrained type through observed direct type kind and any domain-base chain to range/multirange; do not infer from type names, display text, GiST, reconstructed DDL or built-in allowlists;
- for PERIOD foreign keys, preserve the exact final period-column position, require at least one preceding equality-key column, require observed update/delete actions and exact `NO ACTION`/`NO ACTION`, and when the referenced relation is present in the same bounded snapshot require both an explicitly observed `conperiod=true` PK/UNIQUE on the exact referenced columns and exact observed `NOT DEFERRABLE` key timing from `condeferrable`/`condeferred`;
- validate schema-local `pg_class`, derived index `relkind`, tablespace/options, relation-kind constraint rules, single-PK cardinality and PK NOT NULL consistency;
- never infer temporal-key truth, temporal type semantics, referenced-key timing, referential-action defaults or unsupported match semantics from reconstructed DDL/index shape/conventions when direct catalog evidence exists;
- enforce policy-admitted row/byte/concurrency ceilings and complete-or-fail snapshot construction.

## Standards and primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE — UNIQUE/PRIMARY KEY indexes, INCLUDE, NULLS NOT DISTINCT, WITHOUT OVERLAPS, PERIOD foreign keys, match types, referential actions, deferrability and constraint naming*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FOREIGN TABLE — supported foreign-table constraint families*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE — constraint ownership of supporting indexes; expression/partial-index restrictions for USING INDEX*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint — `condeferrable`, `condeferred`, `conindid`, `conperiod`, `confupdtype`, `confdeltype`, `confmatchtype`*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index — `indisunique`, `indisprimary`, `indisexclusion`, `indimmediate`, `indnullsnotdistinct`, `indkey`, `indpred` and index state*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_type — direct type kind and domain base type*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_range — range and associated multirange catalog relationships*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class and CREATE INDEX*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes — temporal constraints using WITHOUT OVERLAPS and PERIOD*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes — allow domains over range/multirange for WITHOUT OVERLAPS*.
- PostgreSQL 18 source `src/backend/catalog/index.c` for supporting-index state and `src/backend/parser/parse_utilcmd.c` for deferrable-index/constraint compatibility.

Catalog OIDs are adapter-local joins, never governed semantic identity. `pg_get_indexdef`/`pg_get_expr` are reconstructed provenance text, never the sole semantic carrier.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams unchanged. |
| Truth/publication lifecycle | RED_ACTIVE_NO_PUBLICATION | No protected immutable semantic release exists; current Source Observation successor has an unresolved temporal-type RED. |
| Source Observation | REPRESENTATION_V3_TEMPORAL_TYPE_RED_ACTIVE | Scalar temporal-column RED `ef86be21...` remains unrepaired in production source; review `5185079749`, positive control `558fb3c...` and doctoring `46a77be...` require PostgreSQL 18.4+ domain-over-range compatibility and source-authoritative type-kind/domain-base evidence. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | Protected/default ConceptWeave `main` still lacks Product workflow authority; #35 waits on central owner settlement. |
| Quality gate | SOURCE_REPAIR_REQUIRED | No Ready/adoption/merge before the temporal type-kind repair and unchanged-head Rust/Product/security/dependency/review evidence. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| PostgreSQL 18 temporal keys | PERIOD_VALUE_TYPE_RED_ACTIVE | Existing conperiod/action/reference/timing/coordinate repairs are preserved, but temporal final-column type semantics are not yet governed. |
| PostgreSQL 18 FK match | MATCH_PARTIAL_FAIL_CLOSED_ACCEPTANCE_PENDING | Reserved catalog code remains historical vocabulary; governed v3 admits only implemented SIMPLE/FULL semantics. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. On #46, implement the minimum source-authoritative PostgreSQL type-kind/domain-base family and temporal admission repair: exact qualified coordinates, direct `typtype`, exact domain `typbasetype` chain, direct user-defined range/multirange support, observed/unobserved digest distinction, and fail-closed missing/cyclic/scalar behavior. Keep `conperiod` authoritative for temporal declaration.
2. On the resulting unchanged exact #46 head, run repository-pinned Rust 1.98 plus applicable hosted Product/security/dependency/review acceptance and causally repair any real failure.
3. Ordinary/non-force adopt verified #46 into #45 and obtain fresh parent acceptance; then adopt #45 into #6. Never transfer predecessor GREEN.
4. In parallel, central owner lands the backward-compatible protected handler, reconciles #2051/#2056 onto current protected `.github/main`, obtains terminal GREEN, then unchanged #35 gets fresh acceptance and normal merge.
5. Foundation ordinary/non-force restacks after #35; descendants consume only released/versioned owner contracts.
6. Only after representation/adapter prerequisites are GREEN implement the bounded PostgreSQL adapter and frozen conformance fixture, followed by discovery/alignment/deterministic validation/independent evaluation/steward review/immutable publication under the canonical owner boundaries.

Adapters remain outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.
