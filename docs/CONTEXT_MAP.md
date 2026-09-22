# Context Map

## Internal relationships

- Source Observation -> Semantic Discovery: **Customer/Supplier**; Discovery consumes immutable observation contracts.
- Semantic Discovery -> Model Validation: **Conformist to published candidate contract**; validation must not rewrite discovery evidence.
- Model Validation -> Governance & Publication: **Customer/Supplier**; governance consumes deterministic validation receipts.
- Governance & Publication -> Interoperability: **Published Language**; adapters consume immutable release contracts.

## External relationships

- contextual-orchestrator -> Semantic Discovery: **Anti-Corruption Layer**. Model/provider envelopes never enter the domain model directly.
- LineageWeave -> Source Observation: **Anti-Corruption Layer**. Inferred/proposed lineage remains explicitly non-authoritative until ConceptWeave governance evaluates it.
- context-graph-contracts <-> Interoperability: **Shared Kernel only for versioned public contracts**, kept minimal.
- semantic-data-portal <- Interoperability: **Published Language**. SDP consumes releases; ConceptWeave does not read SDP application tables.
- Keyverse -> future delivery layer: **Anti-Corruption Layer** for verified identity/tenant context.
