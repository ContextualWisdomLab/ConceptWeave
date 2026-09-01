# Standards and Research References

This file records the initial evidence basis for ConceptWeave architecture decisions. Stable Recommendations and in-progress specifications are deliberately distinguished.

## Stable standards / recommendations

Miles, A., & Bechhofer, S. (Eds.). (2009). *SKOS Simple Knowledge Organization System Reference*. World Wide Web Consortium. https://www.w3.org/TR/skos-reference/

W3C OWL Working Group. (2012). *OWL 2 Web Ontology Language document overview (Second Edition)*. World Wide Web Consortium. https://www.w3.org/TR/owl2-overview/

W3C Provenance Working Group. (2013). *PROV-O: The PROV Ontology*. World Wide Web Consortium. https://www.w3.org/TR/prov-o/

W3C RDF Working Group. (2014). *RDF 1.1 concepts and abstract syntax*. World Wide Web Consortium. https://www.w3.org/TR/rdf11-concepts/

W3C Data Shapes Working Group. (2017). *Shapes Constraint Language (SHACL)*. World Wide Web Consortium. https://www.w3.org/TR/shacl/

World Wide Web Consortium. (2020). *JSON-LD 1.1*. https://www.w3.org/TR/json-ld11/

## In-progress / emerging specifications tracked, not claimed as final standards

W3C RDF-star Working Group. (2026). *RDF 1.2 concepts and abstract data model* (Candidate Recommendation Snapshot, April 7, 2026). World Wide Web Consortium. https://www.w3.org/TR/rdf12-concepts/

W3C Data Shapes Working Group. (2026). *SHACL 1.2 Core* (Working Draft, August 3, 2026). World Wide Web Consortium. https://www.w3.org/TR/shacl12-core/

Apache Software Foundation. (2026). *Apache Ossie (incubating)*. https://ossie.apache.org/  
Formerly Open Semantic Interchange (OSI); tracked as an emerging vendor-neutral semantic-model exchange specification rather than a W3C/ISO standard.

## Research basis

Babaei Giglou, H., D'Souza, J., & Auer, S. (2023). LLMs4OL: Large language models for ontology learning. In *The Semantic Web – ISWC 2023* (pp. 408–427). Springer. https://doi.org/10.1007/978-3-031-47240-4_22

Hertling, S., & Paulheim, H. (2023). OLaLa: Ontology matching with large language models. In *Proceedings of the 12th Knowledge Capture Conference 2023* (pp. 131–139). Association for Computing Machinery. https://doi.org/10.1145/3587259.3627571

Lo, A., Jiang, A. Q., Li, W., & Jamnik, M. (2024). *End-to-end ontology learning with large language models* [Preprint]. arXiv. https://arxiv.org/abs/2410.23584

Li, J., Garijo, D., & Poveda-Villalón, M. (2026). Large language models for ontology engineering: A systematic literature review. *Semantic Web, 17*(4), 1–45. https://doi.org/10.1177/22104968261465514

## Decision implications

- LLMs can assist ontology learning, matching, modeling, and maintenance, but current research does not justify automatic authority promotion.
- Evaluation practices across LLM ontology engineering are still heterogeneous; ConceptWeave therefore requires reproducible deterministic checks plus human-reviewed benchmark cases.
- Modular ontology engineering and explicit source provenance are preferred over one opaque prompt that attempts to generate an entire enterprise semantic layer in a single step.
