# PostgreSQL transform extension-membership lineage integrity

## Decision

`IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot` must fail closed unless its converter-function extension-membership predecessor and the separately supplied raw transform-converter snapshot describe the same immutable raw converter root. Equal source-generation metadata, equal converter direction cardinality, and equal converter schema/function names are necessary but not sufficient ancestry proof.

The transform-object membership fact remains one fact per `(constraint, key_position, transform_type, target_language)` `pg_transform` row. The repair does **not** duplicate extension membership per FROM SQL / TO SQL direction; it proves that the separately supplied raw transform binding is the exact root from which the converter-function successor chain descended before object-level membership evidence is issued.

## Problem and realistic RED

The first repair on exact head `fcaa330965915a85ea7116a66c04286b79ef5931` closed a concrete mixed-lineage case: source-generation metadata could match while raw converter directions or converter function identities differed. Finding review `5253304917`, RED `3f3a947e31c3b80e1a66120b89705ec3e515e43c`, production repair `37742d774d7c33a693dc3a0359a6583134bc5cbf`, and edge contract `276821d5d5fdafbd4776c9856acfc04221e470c1` established exact direction/function binding.

A follow-up review found a stronger ancestry defect that survives those checks. Two same-generation raw converter snapshots can retain the same transform type, target language, FROM/TO direction set, converter schema, and converter function names while differing in raw converter-definition material. Reconstructing equality from repeated names and counts therefore cannot prove that the latest converter-function successor was derived from the separately supplied raw snapshot.

Finding review `5253398088` records this defect. Behavioral RED `8ab7c431cdba2d0ba43a66313253b93b53b9e72a` keeps generation metadata and reconstructed direction/function identities stable while changing the raw converter-definition root. The constructor must reject that composition instead of treating repeated identifiers as ancestry proof.

## Causal repair

The ordinary-forward source repair propagates the immutable raw transform-converter root digest through the converter-function successor chain as `converter_snapshot_digest`. Later owner, access-control, configuration, security, planner/cost, shape, argument, transform-type, and extension-membership successors carry that same root rather than substituting their own derived successor digests.

Exact source head `7ee7847b629e4f2e5ec5a12f93f7da06f23f366c` adds the decisive constructor invariant: `converter_extension_membership_snapshot.converter_snapshot_digest()` must equal the separately supplied `transform_converter_snapshot.snapshot_digest()` before the existing direction count and exact converter binding checks run. A mismatch fails closed with `index_exclusion_constraint_operator_procedure_transform_extension_membership_lineage`.

The earlier direction/function checks remain required defense in depth. Root equality proves immutable ancestry; exact coordinate, direction, schema, and function checks prove that the composed object-level observation still matches the expected converter bindings. Neither check infers extension membership from converter functions, adds a PostgreSQL catalog field, copies extension-owned truth, or changes object-level digest granularity.

Exact-head COMMENT review `5253528602` records source/contract coherence at `7ee7847b...`; it is not approval or GREEN evidence.

## PostgreSQL authority

PostgreSQL 18 defines a transform by type and procedural language and allows independent FROM SQL and TO SQL converter functions; either direction may be omitted. Therefore converter direction and exact function identity are material parts of the raw `pg_transform` binding. The additional raw-root digest is an internal ConceptWeave provenance invariant used to prevent two same-generation but independently constructed snapshots from being composed as one lineage.

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

## Acceptance

Source repair and documentation coherence are not execution evidence. The final exact PR head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the same-name/different-definition root-lineage regression plus retained direction/function drift regressions, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential. Hosted or predecessor results do not transfer after source or documentation movement.
