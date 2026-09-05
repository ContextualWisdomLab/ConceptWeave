# Context Map

## Internal relationships

- Source Observation -> Semantic Discovery: **Customer/Supplier**; Discovery consumes immutable observation contracts.
- Semantic Discovery -> Model Validation: **Conformist to published candidate contract**; validation must not rewrite discovery evidence.
- Model Validation -> Governance & Publication: **Customer/Supplier**; governance consumes deterministic validation receipts.
- Governance & Publication -> Interoperability: **Published Language**; adapters consume immutable release contracts.
- Research Intake -> Governance & Publication: **Anti-Corruption Layer**; Governance verifies complete duplicate and classification-write review sets and returns only opaque authority receipts before Intake emits canonical-key operations or write plans.

Research Intake also owns the optional private Full-Text Capture bound to a metadata report. It preserves provider observations while rejecting mixed-origin counters as a reliable incremental cursor. This is an adapter responsibility, not a new bounded context, shared catalog or approval owner; downstream proposal adoption remains a separate evidence/review transition.

The Full-Text Review View is an in-context read projection over that verified capture and the current pending worksheet. It introduces no new aggregate or authority owner. The separate Full-Text Review Worksheet carries one verified capture through atomic completed-view application and finalization. Governance receives the complete capture-bound reviewed set for independent verification; no metadata-only downcast or approval renewal is provided. The classification-write review remains a separate authority contract, with no full-text evaluation-to-write conversion.

## External relationships

- Zotero Local API -> research evidence intake: **Anti-Corruption Layer into Semantic Discovery**. Zotero remains the bibliographic system of record; ConceptWeave consumes a version-pinned snapshot and emits proposal evidence. Execute-mode metadata changes cross only a caller-owned authenticated adapter after complete preflight; ConceptWeave retains no API key and records verified item-level outcomes and rollback coordinates. Item metadata, attachments, collection/tag truth, and write authority remain in Zotero. No Zotero record becomes semantic authority without ConceptWeave validation/review/publication.
- contextual-orchestrator -> Semantic Discovery: **Anti-Corruption Layer**. Model/provider envelopes never enter the domain model directly.
- LineageWeave -> Source Observation: **Anti-Corruption Layer**. Inferred/proposed lineage remains explicitly non-authoritative until ConceptWeave governance evaluates it.
- context-graph-contracts <-> Interoperability: **Shared Kernel only for versioned public contracts**, kept minimal.
- semantic-data-portal <- Interoperability: **Published Language**. SDP consumes releases; ConceptWeave does not read SDP application tables.
- Keyverse -> future delivery layer: **Anti-Corruption Layer** for verified identity/tenant context.
- governance-risk-compliance -> Source Observation / Client Consumption: **Proposed Anti-Corruption Layer**, pending a released contract and exact-consumer proof. External requirements, internal controls, evidence links and effectiveness remain distinct GRC-owned meanings; ConceptWeave cannot infer assurance from evidence presence.
- Orgmetra -> Source Observation: **Proposed Anti-Corruption Layer**, pending a released evidence contract. Job/Task/KSAO relationships remain product-owned; tenant, source version and review status must survive observation without moving employment authority or ordinal scoring into ConceptWeave.
