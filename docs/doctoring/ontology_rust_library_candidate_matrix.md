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
| [Sophia](https://github.com/pchampin/sophia_rs) | RDF terms, parsing/serialization, and bounded RDFS-oriented fixture support behind a ConceptWeave `RdfToolkitPort`. | Source revision `e9d4a4b0b3e65a17319c54b7d92f53bdb60baaa5` on 2026-09-09. | Apache-2.0. | It is not OWL-DL reasoning, SHACL validation, ontology authority, or a catalog. **Proposed; do not add.** |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Isolated RDF/SPARQL query or store experiment behind an ACL, never the default semantic system of record. | Latest upstream release `v0.5.11`, published 2026-09-02. | Apache-2.0 OR MIT. | Its store/query implementation does not replace ConceptWeave release governance or prove query-performance fitness. **Proposed; do not add.** |
| [shacl-rust](https://github.com/ensaremirerol/shacl-rust) | SHACL Core validator experiment, only after a named conformance fixture passes. | Source revision `66d53fce5c39475260f574eb7acc4b93caed44b8` on 2026-09-09. | MIT. | Do not infer SHACL feature coverage from repository presence; unsupported components must fail closed. It cannot publish or approve a model. **Proposed; do not add.** |

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
