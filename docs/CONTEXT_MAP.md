# Context Map

## Internal relationships

- Source Observation request admission -> local source policy: **Customer/Supplier**. A structurally valid key/schema/resource request is not authority; policy binds the source to an immutable revision and explicitly admits exact schema scope plus the complete provider-independent resource envelope. Schema/resource policy defaults to deny.
- Local source policy -> Source Observation adapter: **Anti-Corruption Layer**. Only `AuthorizedObservationRequest` crosses into live source access. The adapter resolves credentials from the exact authorized key-and-binding pair, consumes the remaining shared operation budget, and keeps PostgreSQL/runtime objects outside provider-independent contracts.
- Source Observation -> Semantic Discovery: **Customer/Supplier**; Discovery consumes complete immutable observation contracts and cannot gain source-access authority from them.
- Semantic Discovery -> Model Validation: **Conformist to published candidate contract**; validation must not rewrite discovery evidence.
- Model Validation -> Governance & Publication: **Customer/Supplier**; governance consumes deterministic validation receipts.
- Governance & Publication -> Client Consumption: **Published Language**; clients consume immutable, versioned semantic-release contracts and never generator-private implementation.
- Governance & Publication -> Interoperability: **Published Language**; export adapters consume immutable release contracts.
- Client Consumption -> Interoperability: **Customer/Supplier** for versioned consumer bindings/adapters only; deterministic admission remains usable without an adapter or LLM.

## External relationships

- contextual-orchestrator -> Semantic Discovery: **Anti-Corruption Layer**. Model/provider envelopes never enter the domain model directly.
- contextual-orchestrator -> future Model Alignment/Client Consumption assistance: **Anti-Corruption Layer**. Matching/explanation outputs remain candidate evidence and never grant authority.
- LineageWeave -> Source Observation: **Anti-Corruption Layer**. Inferred/proposed lineage remains explicitly non-authoritative until ConceptWeave governance evaluates it.
- context-graph-contracts <-> Interoperability: **Shared Kernel only for versioned public contracts**, kept minimal.
- semantic-data-portal <- Client Consumption/Interoperability: **Published Language**. SDP consumes releases; ConceptWeave does not read SDP application tables.
- governance-risk-compliance <- Client Consumption: **Published Language + downstream ACL**. GRC validates/uses releases while retaining business truth, tenant/purpose authorization, and physical execution. A future Source Observation adapter may read only explicitly authorized metadata and must not become a GRC repository or copy GRC truth.
- enterprise-architecture-core <- Client Consumption: **Published Language + downstream ACL** under the same boundary.
- Keyverse -> future delivery/consumer authorization seams: **Anti-Corruption Layer** for verified identity/tenant context; ConceptWeave does not take ownership of downstream authorization policy.