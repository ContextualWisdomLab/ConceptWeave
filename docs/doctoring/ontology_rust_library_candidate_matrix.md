# Rust Ontology Library Candidate Matrix

**Decision status:** Proposed evidence only, 2026-09-09. No dependency is adopted and no semantic release is changed by this record.

## Purpose and KPI

The 2026-09-09 Zotero Local API replay reconciled 8,326 observed records into 3,715 classifier proposals and 4,611 unclassified records. This matrix turns the resulting ontology-research queue into a bounded engineering decision: every implementation candidate must have an identified responsibility, an upstream licence and immutable coordinate, and a standards-conformance test before it can become a dependency.

**KPI:** 0 of 3 candidates may cross from `Proposed` to `Evaluating` without all four admission receipts below. The queue count is discovery workload, not semantic coverage, review completion, precision, recall, publication authority, or Zotero write authority.

## Boundaries and admission receipts

ConceptWeave remains the owner of evidence-bound semantic-model generation, validation, and release. Semantic Data Portal may catalogue a released contract but is not a source of ontology truth. A Rust library can be admitted only behind a ConceptWeave-owned port and ACL, with:

1. a pinned crate and upstream source revision, reconciled with both repository and package licence metadata;
2. a minimal RDF/OWL/SKOS/SHACL fixture whose expected result is independently checked against the named W3C Recommendation;
3. a fail-closed unsupported-feature result, including provenance, truth status, publication state, and locale-label boundaries; and
4. a protected ConceptWeave PR with exact-head tests and an immutable release before any consumer adoption.

Neither a local successful test nor an upstream claim authorizes semantic truth, a Zotero mutation, or catalog publication. Candidate evaluation must not read another service's application tables or copy owner source.

## Candidate matrix

| Candidate | Bounded role if evaluated | Upstream evidence at inspection | Licence | Excluded scope and current decision |
| --- | --- | --- | --- | --- |
| [Sophia](https://github.com/pchampin/sophia_rs) | RDF terms, parsing/serialization, and bounded RDFS-oriented fixture support behind a ConceptWeave `RdfToolkitPort`. | Source revision `e9d4a4b0b3e65a17319c54b7d92f53bdb60baaa5`, inspected 2026-09-09. GitHub reports that commit as unsigned, so the SHA is a content coordinate rather than signed publisher provenance. | Apache-2.0. | It is not OWL-DL reasoning, SHACL validation, ontology authority, or a catalog. **Proposed; do not add.** |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Isolated RDF/SPARQL query or store experiment behind an ACL, never the default semantic system of record. | Release `v0.5.11` was published 2026-09-02 but GitHub marks the release `immutable=false`; the tag currently resolves to commit `df37a5c98e2497135cdd4cfce01a049b78ca6740`. The published source tarball reports SHA-256 `563c7ad6dc397cdbedb8ab9a1f4fdc671d9d5119b43d9498d8e4d0bc0b6a2567`. Evaluation must pin the commit and verified artifact digest rather than trust the mutable tag name alone. | `MIT OR Apache-2.0` in the pinned workspace metadata. | Its store/query implementation does not replace ConceptWeave release governance or prove query-performance fitness. **Proposed; do not add.** |
| [shacl-rust](https://github.com/ensaremirerol/shacl-rust) | SHACL Core validator experiment, only after a named conformance fixture passes. | Source revision `66d53fce5c39475260f574eb7acc4b93caed44b8`, inspected 2026-09-09. The upstream commit explicitly records that stable `source_shape` diagnostics were repaired while `source_constraint` remains unimplemented. | MIT. | Diagnostic constraint provenance is therefore an explicit unsupported feature for ConceptWeave until independently implemented and tested; do not silently drop it. Do not infer broader SHACL feature coverage from repository presence. It cannot publish or approve a model. **Proposed; do not add.** |

## Upstream evidence reconciliation

The admission receipt is the immutable coordinate plus independently checked metadata, not a release label. For Oxigraph, GitHub's release object for `v0.5.11` is explicitly mutable even though its current tag target and release assets are concrete. ConceptWeave must therefore retain the resolved commit and artifact digest in any evaluation receipt and re-check them before dependency admission. The pinned `Cargo.toml` at `df37a5c...` declares version `0.5.11`, `MIT OR Apache-2.0`, Rust edition 2024, and minimum Rust `1.87`; these are compatibility inputs, not evidence of semantic conformance.

For shacl-rust, the pinned upstream commit itself documents a provenance limitation: it makes `source_shape` stable across independently parsed shape graphs but states that `source_constraint` is not implemented. A ConceptWeave conformance fixture that consumes diagnostics must therefore fail closed when constraint-level provenance is required and unavailable. Treating a valid SHACL result as sufficient without preserving which constraint produced it would violate this product's evidence-bound validation boundary.

For Sophia, the pinned commit is resolvable but GitHub reports it as unsigned. This does not invalidate evaluation, but it prevents the commit SHA alone from being described as authenticated publisher provenance. Package/source licence reconciliation and the named standards fixture remain mandatory before the candidate can move to `Evaluating`.

## Standards and test contract

The fixture set must distinguish RDF graph syntax, OWL vocabulary/semantics, SKOS labelling, and SHACL constraint validation. In particular, `skos:broader` is not an `rdfs:subClassOf` substitute, and passing a SHACL fixture does not establish OWL entailment. The actual supported subset belongs in the release contract; an absent feature is an explicit failure, never a silently ignored constraint.

## Utility-repository decision

No utility repository is justified. These three candidates serve one bounded owner, ConceptWeave, and share no proven cross-product contract. Reconsider only after at least two independently released CWL consumers need the same versioned, conformance-tested adapter and its ownership cannot remain with ConceptWeave.

## References

Champin, P.-A. (2026). *Sophia: A Rust toolkit for RDF and linked data* [Computer software]. GitHub. https://github.com/pchampin/sophia_rs

Ensar Emirerol. (2026). *shacl-rust* [Computer software]. GitHub. https://github.com/ensaremirerol/shacl-rust

Oxigraph contributors. (2026). *Oxigraph* (Version 0.5.11) [Computer software]. GitHub. https://github.com/oxigraph/oxigraph

W3C Data Shapes Working Group. (2017). *Shapes Constraint Language (SHACL)*. World Wide Web Consortium. https://www.w3.org/TR/shacl/

W3C OWL Working Group. (2012). *OWL 2 Web Ontology Language document overview (Second Edition)*. World Wide Web Consortium. https://www.w3.org/TR/owl2-overview/

Miles, A., & Bechhofer, S. (Eds.). (2009). *SKOS Simple Knowledge Organization System reference*. World Wide Web Consortium. https://www.w3.org/TR/skos-reference/
