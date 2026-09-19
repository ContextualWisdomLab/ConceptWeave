# PostgreSQL transform-object extension-membership root-lineage integrity

Status: **RED / repair required**

Exact finding authority began at ConceptWeave PR #46 head `7f95969af37dadb307c6b4145c0151bd7c088284` with review `5253398088`. Behavioral regression `8ab7c431cdba2d0ba43a66313253b93b53b9e72a` pins the remaining cross-input lineage defect.

## Problem

`IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new` composes two independently supplied inputs:

1. `IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot`, whose digest descends through the converter-function successor chain; and
2. `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot`, which is supplied again to recover the exact `pg_transform` row identity and target language.

The preceding repair correctly rejects direction-set drift and converter schema/function-name substitution. It still does not prove that input 2 is the exact raw converter snapshot from which input 1 ultimately descends.

A caller can construct another same-generation raw converter snapshot with the same constraint/key/type/direction and the same converter schema/function names but different immutable converter-function definition material. The present name/count binding accepts that graft. The new regression changes only the FROM-SQL converter definition material while retaining the same converter name and requires fail-closed rejection.

This is not a new PostgreSQL catalog-field gap. It is an internal evidence-lineage invariant: a governed successor must have one exact raw converter root, not two independently supplied roots that merely agree on names.

## Constraints

- Do not collapse transform-object membership into per-direction membership. `pg_transform` membership remains one fact per exact `(transform type, target language)` row.
- Do not copy extension-owned metadata into ConceptWeave.
- Do not treat source connection key, policy binding, extractor revision, and observation timestamp as proof that two independently supplied snapshots have identical content ancestry.
- Do not weaken the existing direction/function binding regression.
- Do not infer root identity from converter schema/function names. Same-name function definitions can change.
- Do not use mutable branch/head identity as evidence lineage.

## Repair alternatives

### A. Propagated immutable raw-root digest — preferred

At the first successor that directly consumes `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot`, capture its exact `snapshot_digest()` as a private immutable lineage anchor. Propagate that anchor unchanged through every converter-function successor. Expose it read-only at the latest converter extension-membership snapshot. The transform-object membership constructor then requires exact equality with the separately supplied raw transform-converter snapshot digest before evaluating row/direction/function bindings.

This preserves the current successor decomposition while making ancestry explicit and machine-checkable.

### B. Single-root construction

Redesign the transform-object membership constructor so it cannot receive a second independent raw converter snapshot. Instead, carry the transform-row identity required by the final successor through one canonical predecessor chain.

This is structurally stronger but broader because target-language and row identity currently live in the raw transform-converter snapshot rather than every later converter-function fact.

### Rejected: name/count matching only

The current repair is necessary but insufficient. Schema/function names and direction cardinality do not bind converter return/argument/implementation evidence or the raw snapshot digest.

### Rejected: observed-at equality as ancestry

Equal observation time is provenance metadata, not content identity. Public constructors can present same-generation metadata for distinct snapshot content; acceptance must remain content-bound.

## Acceptance

The repair is complete only when the exact current #46 head demonstrates all of the following on one unchanged source generation:

- the new same-name/different-definition regression fails on the pre-repair constructor and passes after the causal production repair;
- the existing direction-set and function-name drift regressions remain GREEN;
- the transform-object successor rejects any separately supplied raw converter snapshot whose immutable root digest differs from the converter-function predecessor lineage;
- the ordinary positive path still produces one transform-object extension-membership fact per exact transform row;
- repository-pinned Rust 1.98 formatting, strict all-target/workspace Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned-production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential run on the final exact head.

No predecessor execution result transfers after any source movement.
