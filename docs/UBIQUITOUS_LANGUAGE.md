# Ubiquitous Language

| Term | Meaning |
| --- | --- |
| Source Snapshot | Immutable revision of source evidence observed by ConceptWeave. |
| Full-Text Capture | Separate private record of exact text/metadata responses, missing results and read interval, bound to an earlier metadata report; not an atomic Source Snapshot or an Authority Receipt. |
| Observation | Deterministically extracted fact from a Source Snapshot. |
| Evidence Reference | Stable source identity, digest, and location supporting a candidate. |
| Semantic Candidate | Evidence-bound proposal for a concept, relation, constraint, dimension, measure, or physical mapping. |
| Semantic Model Proposal | Versioned collection of candidates presented for validation/review. |
| Validation Report | Deterministic result describing structural or semantic contract validity; not a review decision. |
| Review Decision | Authorized accept/reject decision over validated candidates or a model proposal. |
| Semantic Model Release | Immutable governed publication artifact. |
| Truth Status | Epistemic classification: observed, inferred, proposed, authoritative, superseded, rejected. |
| Publication State | Governance workflow state: draft, proposed, validated, reviewed, published, superseded, rejected. |
| Physical Mapping | Mapping from a physical schema/API/event element to a semantic concept or field. |
| Dimension | Governed categorical or temporal axis used to group/filter analytical facts. |
| Measure | Governed calculation with explicit expression, grain, units, null semantics, and evidence. |
| Semantic Steward | Authorized reviewer responsible for accepting or rejecting semantic meaning. |
| Reviewed Classification Change | Authorized complete replacement of one paper's collection and tag state, bound to its observed revision. |
| Classification Write Plan | Local deterministic dry-run artifact containing exact preconditions and before/after/rollback metadata; not proof of execution. |
| Classification Write Receipt | Secret-free, reviewed-plan-bound result that distinguishes dry-run, verified completion, preflight failure, and partial failure while retaining applied, failed, untouched, and safely reversible coordinates. |
| Classification Rollback Operation | Reverse-ordered complete-state restoration bound to the post-write item revision returned by the Local API. |
| Reviewed Duplicate Merge Set | Complete steward decisions selecting one consistent canonical item across every overlapping duplicate group in a snapshot. |
| Authority Receipt | Opaque proof checked by the Governance & Publication boundary; it contains no reviewer identity or credential. |
| Canonical-Key Operation | Reversible local mapping from every source in a connected duplicate component to one retained key, with complete reviewed revisions and the exact rollback mapping. |
