# CWL ontology capability inventory

Evidence snapshot: 2026-09-05. Status: research inventory, not dependency-adoption approval.

## Scope and evidence limits

This bounded audit covers eight candidates from the existing ecosystem boundaries, not every CWL repository. Each row separates the owner's documented responsibility, its exact default-branch documentation, and published GitHub release evidence. A name, open PR or protected branch alone does not prove a usable released API. Package registries, deployed behavior, release attestations and consumer conformance were not audited here.

GitHub repository, branch, README-at-SHA and release endpoints supplied these observations. All eight default branches reported protected. `context-graph-contracts` and `enterprise-architecture-core` default to `develop`; their unprotected `main` branches are not the adoption baseline. GitHub detected MIT for semantic-data-portal, TEPP and LineageWeave, Apache-2.0 for RankWeave, and no SPDX identifier for the other four. An undetected license is unresolved evidence, not permission to adopt.

## Owner and maturity evidence

| Candidate | Responsibility and boundary | Exact default-head documentation | Release evidence / next cultivation requirement |
| --- | --- | --- | --- |
| ConceptWeave | Ontology/semantic-model generation, validation, review and release; not bibliography or catalog ownership. | `main@f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; [bootstrap README](https://github.com/ContextualWisdomLab/ConceptWeave/blob/f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425/README.md). | No GitHub release returned. Repair Research Intake integrity and pass Foundation's protected gates before claiming a released generator. |
| semantic-data-portal | Catalog, graph/semantic retrieval and governed consumption. Its concept-ingestion API does not prove reviewed ConceptWeave-release consumption. | `main@e48aa13c4af7a4875d4b53e6a60b50405c265a2f`; [API documentation](https://github.com/ContextualWisdomLab/semantic-data-portal/blob/e48aa13c4af7a4875d4b53e6a60b50405c265a2f/README.md). | No GitHub release returned. Prove released ingestion with unreviewed/incompatible-input rejection; do not copy catalog persistence into ConceptWeave. |
| context-graph-contracts | Shared versioned interoperability contracts, not ownership of product meaning. | `develop@99cb5468ba3c15c5e79688f53dee74724fae2d13`; [bootstrap README](https://github.com/ContextualWisdomLab/context-graph-contracts/blob/99cb5468ba3c15c5e79688f53dee74724fae2d13/README.md). | No GitHub release returned. Complete owner contracts, licensing and conformance release before consumer import. |
| enterprise-architecture-core | Enterprise-architecture and transformation decisions; ConceptWeave does not acquire EA authority. | `develop@dd71e40a86385fb7861b0f1be19891a3f3e29ece`; [bootstrap README](https://github.com/ContextualWisdomLab/enterprise-architecture-core/blob/dd71e40a86385fb7861b0f1be19891a3f3e29ece/README.md). | No GitHub release returned. Establish a versioned decision/Context Map contract while preserving product-owned truth. |
| EmbedRelay | Embedding identity and continuity across model migrations; vector similarity remains proposal evidence. | `main@816dcacd4fc1903d91c5cae9b77e37e21811a78d`; [bootstrap README](https://github.com/ContextualWisdomLab/EmbedRelay/blob/816dcacd4fc1903d91c5cae9b77e37e21811a78d/README.md). | No GitHub release returned. Release identity/compatibility contracts before cross-model retrieval; no consumer-side substitute conversion. |
| RankWeave | Retrieval fusion/evaluation adjacent to ontology candidate retrieval; no ontology learning or approval authority. | `main@92323cb8b55baf5d840cb97fa8534a0e75ef234c`; also inspected [release-source README](https://github.com/ContextualWisdomLab/RankWeave/blob/61c49c50d3b4a24fc9bd7c6d3a7f2f4ba19d7be6/README.md). | [v0.18.0](https://github.com/ContextualWisdomLab/RankWeave/releases/tag/v0.18.0), published 2026-08-06, resolves to `61c49c50d3b4a24fc9bd7c6d3a7f2f4ba19d7be6`. This is Python 3.10+ release-source evidence, not a Rust runtime or ConceptWeave adoption proof. Production arithmetic changes belong in the owner under the Rust-first policy. |
| TEPP | Temporal/event/relation measurement supplies bounded evidence; not ontology publication ownership. | `main@a243f18da4a4ca8a8d068c39922537f1f8ed6ad0`; [workspace and limitations](https://github.com/ContextualWisdomLab/TEPP/blob/a243f18da4a4ca8a8d068c39922537f1f8ed6ad0/README.md). | No GitHub release returned. Documented Rust contracts/partial analysis are not a complete commercial estimator. Prove released wire-contract and provenance compatibility before adoption. |
| LineageWeave | Reconstructed lineage candidates supply Source Observation evidence without becoming semantic authority. | `main@83eba56149eb802cd63642c507c324c9976ec78e`; [integration boundaries](https://github.com/ContextualWisdomLab/LineageWeave/blob/83eba56149eb802cd63642c507c324c9976ec78e/README.md). | No GitHub release returned. Verify a released observation contract and real consumer conformance; documented sibling dependencies alone do not establish readiness. |

No runtime dependency, repository, service or database was added. The [Context Map](../CONTEXT_MAP.md) and [ADR 0006](../adr/0006-zotero-research-intake.md) still place Research Intake in ConceptWeave. A separate utility owner needs an evidenced independent consumer and deployment contract.

## Actual Zotero evidence

The read-only executable at `22030ae6c8510d9eb8f7b07d98959bb69d2bd286` captured a distinct report/worksheet pair after detecting Zotero 10.0.1, API 3, schema 44, library version 2 and a present server identity. The earlier 9.0.6/schema-42/library-12341 pair remains historical. Zotero documents that pre-10 synced revisions could remain unchanged after local edits; version 10 uses instance-local revisions (Zotero, 2026). These version spaces must not be compared or merged. Equal record totals do not prove unchanged content.

The new read contains 8,326 records and 3,715 bibliographic proposals: 56 adjacent-evidence proposals, one semantic-consumption bridge and 3,658 abstentions. Among abstentions, 3,505 have no deterministic rule match and 153 have unsupported rule vocabulary. An abstract exists in review context or matched evidence for 2,715/3,715 records; 1,000 lack an abstract in this report. These measure metadata availability and routing, not ontology relevance or classification accuracy. Missing abstracts and unmatched phrases cannot prove irrelevance.

No externally approved paper-to-owner link has been demonstrated by this audit. The separate [research-to-capability register](RESEARCH_CAPABILITY_TRACEABILITY.md) supplies design hypotheses and evaluation families, not already-reviewed Zotero labels. PROV distinguishes evidence and producing activities; SKOS supplies vocabulary/label relationships; SHACL specifies graph validation. None grants business approval (Groth & Moreau, 2013; Miles & Bechhofer, 2009; Knublauch & Kontokostas, 2017).

Two existing PR #10 findings remain source-confirmed at the capture head: [provider metadata lost before hashing](https://github.com/ContextualWisdomLab/ConceptWeave/pull/10#discussion_r3934854209) and [mutable predictions evaluated under an unchanged receipt](https://github.com/ContextualWisdomLab/ConceptWeave/pull/10#discussion_r3934854221). File SHA-256 proves saved artifact bytes, but the current source digest does not yet prove complete raw-provider-field binding. Repair and regenerate before authority promotion. No labels, approval, authorization prompt, write or rollback were performed.

## KPI and next actions

| Measure | Observation | Required next evidence |
| --- | --- | --- |
| Bounded owner audit | 8/8 selected candidates have exact default-head documentation and release-query evidence | Expand for actual responsibilities/consumers; this is not an organization-wide denominator. |
| GitHub release with resolved source commit | 1/8 selected candidates | Artifact/provenance and consumer conformance; a tag is insufficient adoption evidence. |
| Verified ConceptWeave adoption | 0 demonstrated in this audit | Released owner contract, exact consumer revision and passing contract/runtime evidence. |
| Unverified steward decisions | 0/3,715 on the new snapshot | Authentic snapshot-bound decisions after source-integrity repair and regeneration. |
| Externally approved full review | 0/3,715 | Complete labels and independently verified approval; no sampled denominator or generated labels. |

First repair the PR #10 integrity findings and propagate non-force through the dependent stack. Then review the repaired current snapshot, including missing-abstract and unsupported-vocabulary records. Do not replace the full denominator with a hand-picked success sample or heuristic relevance threshold. Maturity observations select the owner for future work, not the labels to silently assign to papers.

## References

Groth, P., & Moreau, L. (Eds.). (2013, April 30). *PROV-overview: An overview of the PROV family of documents* (W3C Working Group Note). World Wide Web Consortium. https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/

Knublauch, H., & Kontokostas, D. (Eds.). (2017, July 20). *Shapes constraint language (SHACL)* (W3C Recommendation). World Wide Web Consortium. https://www.w3.org/TR/2017/REC-shacl-20170720/

Miles, A., & Bechhofer, S. (Eds.). (2009, August 18). *SKOS simple knowledge organization system reference* (W3C Recommendation). World Wide Web Consortium. https://www.w3.org/TR/2009/REC-skos-reference-20090818/

Zotero. (2026, July 29). *Zotero local API*. https://www.zotero.org/support/dev/web_api/v3/local_api
