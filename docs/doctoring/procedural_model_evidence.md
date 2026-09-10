# Procedural model source and conformance record

Status: Proposed source evidence, not released product authority. Issue #42.
Decision, alternatives and APA references:
[ADR-PG-20260910](../adr/pg_20260910_procedural_model_engineering.md).

The paper motivates procedural relationships, localized situational guidance and
offline graph refinement. ConceptWeave ownership, strict candidate-only envelopes,
eight-locale label grammar, independent approval and release/activation separation
are CWL adaptations, not claims made by that paper. No benchmark replication or
full supplementary implementation audit is claimed by this input-contract slice.

| Trace | Exact local evidence scope |
| --- | --- |
| Procedural authoring grammar | Two new local draft schemas, referencing the unchanged generic evidence schema |
| Fixture generation | Same 46 materialized cases supplied to the independent validator and planned native AJV driver |
| Native test infrastructure | Node 22.16.0: 16 fixture-runner tests pass; this is not production Rust coverage |
| Independent schema validation | Python jsonschema 4.26.0: all three schemas pass meta-validation; 46/46 case expectations pass |
| Strict identity regression | Before the fix, 44/46 pass and two trailing-newline identity/digest cases incorrectly pass schema validation; after the fix, 46/46 pass |
| Native AJV availability | Offline probe returned ENOTCACHED; no native AJV or hosted Product GREEN is claimed |
| Semantic gaps | Four shape-positive witnesses explicitly require later entry/endpoint/identity/evidence checks |

The strict-identity failure was caused by the conventional dollar anchor matching
before a final line terminator. The local identity and SHA-256 patterns now require
that no character remain after the matched value. The regression corpus retains
both counterexamples. This hardens input syntax; it does not authenticate a digest.

Python was used only as an independent validation tool in the authoring environment;
no Python runtime or dependency was added to ConceptWeave. The repository driver uses
the existing AJV CLI 5.0.0 in Product, without coercion or changed security gates.
Rust model validation, authenticated source/partition/base binding, independent
evaluation, publication and consumer conformance remain separate prerequisites.
