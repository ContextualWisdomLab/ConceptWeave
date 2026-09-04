# Ubiquitous Language

| Term | Meaning |
| --- | --- |
| Source Snapshot | Immutable revision of source evidence observed by ConceptWeave. |
| Observation | Deterministically extracted fact from a Source Snapshot. |
| Evidence Reference | Stable source identity, digest, and location supporting a candidate or release. |
| Semantic Candidate | Evidence-bound proposal for a concept, relation, constraint, dimension, measure, or physical mapping. |
| Semantic Model Proposal | Versioned collection of candidates presented for validation/review. |
| Validation Report | Deterministic result describing structural or semantic contract validity; not a review decision. |
| Review Decision | Authorized accept/reject decision over validated candidates or a model proposal. |
| Semantic Model Release | Immutable governed publication artifact consumed only through versioned public contracts. |
| Release Contract Version | Explicit version of the client-visible semantic-release schema/compatibility contract. |
| Release Digest | Declared `sha256:<64 hex>` digest identity. Syntax validation alone is not cryptographic verification of artifact bytes. |
| Client Admission | Deterministic decision that a release has a supported contract version and is Published + Authoritative; it does not grant downstream authorization. |
| Client Consumption | Supporting bounded context for release admission, compatibility, and future diff/match/resolve/explain/query-plan contracts. |
| Truth Status | Epistemic classification: observed, inferred, proposed, authoritative, superseded, rejected. |
| Publication State | Governance workflow state: draft, proposed, validated, reviewed, published, superseded, rejected. |
| Physical Mapping | Mapping from a physical schema/API/event element to a semantic concept or field. |
| Dimension | Governed categorical or temporal axis used to group/filter analytical facts. |
| Measure | Governed calculation with explicit expression, grain, units, null semantics, and evidence. |
| Semantic Steward | Authorized reviewer responsible for accepting or rejecting semantic meaning. |
| Consuming Product ACL | Downstream product boundary that retains tenant/purpose authorization and physical data/query execution after ConceptWeave client admission. |
