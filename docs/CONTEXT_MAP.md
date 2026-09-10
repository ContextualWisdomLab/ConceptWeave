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

## Procedural model extension — Proposed, issue #42

- ConceptWeave Governance & Publication -> Noema: **Published Language + consumer ACL**.
  ConceptWeave publishes procedural representations; Noema pins a compatible projection
  per execution. Neither graph publication nor a relation label authorizes a tool call.
- Noema -> Source Observation: **Anti-Corruption Layer**, minimized observable training
  evidence only; no hidden reasoning, secrets or foreign domain-truth replication.
- Independent evaluation -> Governance & Publication: **Customer/Supplier**, authenticated
  exact-candidate paired evidence under a registered protocol; refiner output is not approval.
- The first local authoring schemas do not establish any of these live integrations.
  Shared wire contracts remain owned by `context-graph-contracts`.
