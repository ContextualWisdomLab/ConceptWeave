# Procedural model authoring implementation plan

Goal: extend ConceptWeave's Semantic Model Engineering responsibility to procedural
knowledge without moving runtime execution or foreign domain truth into this product.
Architecture: ConceptWeave authors and validates model candidates; its Governance &
Publication context later publishes immutable artifacts. Noema consumes released
projections for execution-local advice. CO supplies model calls, not publication authority.
Tech stack: JSON Schema 2020-12 and the existing AJV CLI 5.0.0 test toolchain for this
first input-contract slice; production graph validation and application code remain Rust.
Spec: ../../adr/pg_20260910_procedural_model_engineering.md
Tracking: ConceptWeave #42 and ContextualWisdomLab/.github #2067.

## First independently reviewable slice

1. Define valid draft and revision examples, plus mutation cases before the schemas.
2. Confirm existing generic candidate input cannot express these procedural envelopes;
   keep that original contract unchanged rather than reinterpreting its released identity.
3. Add `contracts/procedural-model-draft.schema.json`: procedure categories, advisory
   relation vocabulary, localized annotations, qualified semantic references and source
   evidence. Only inferred drafts are accepted; no tool/approval authority fields.
4. Add `contracts/procedural-revision-proposal.schema.json`: exact-base reference,
   complete candidate, training-only evidence envelope, rejection references and rationale.
   It cannot carry validation scores, final-test answers or a self-issued approval field.
5. Add a deterministic Node fixture-materialization/AJV driver using the already used
   CLI version; wire it into the existing Product JSON-contract step, without new triggers.
6. Run schema meta-validation and every generated fixture; retain toolchain identities
   and distinguish local validator evidence from native AJV and whole-workspace CI.
7. Link ADR, PRD/TRD/UML/security/test/recovery acceptance and product gaps. Publish a
   Draft child of the current Foundation; preserve all existing stack deltas and gates.

## Next slices, not delivered by input-schema validation

- Rust graph construction: byte/duplicate-key bounds before parse, unique identities,
  entry/endpoint membership, exact scope/parent matching, evidence closure, deterministic
  content identity, immutable aggregate and profile-specific cycle/reachability checks.
- Ontology alignment: exact released concept/tool-contract references; relation direction
  and unsupported profile detection. No silent synonym substitution or source-truth copy.
- CO-assisted discovery/refinement from admitted SOP/API/source/observable training evidence.
- Independently authenticated paired validation, final confirmation, scoped persistent
  rejection history, steward decisions and CAS-controlled immutable publication.
- Released contract/projection to Noema, then catalog/EA projections and opt-in product
  shadow/canary runs with policy, idempotency, cancellation, revocation and rollback evidence.

Schema success is structural only. It does not prove endpoint membership, actual evidence
partition, signature validity, authorization, performance, publication or deployment. Do not
add a production endpoint, LLM caller, new Python runtime, or automatic activation here.
