# UML and lifecycle views

## Candidate state machine

```mermaid
stateDiagram-v2
    [*] --> Draft
    Draft --> Proposed
    Draft --> Rejected
    Proposed --> Validated
    Proposed --> Rejected
    Validated --> Reviewed
    Validated --> Rejected
    Reviewed --> Published
    Reviewed --> Rejected
    Published --> Superseded
    Rejected --> [*]
    Superseded --> [*]
```

## Source Observation authorization and execution

```mermaid
sequenceDiagram
    participant Caller
    participant Request as ObservationRequest
    participant Policy as SourceConnectionRegistry
    participant Adapter as SourceObservationPort
    participant Source
    participant Snapshot as PostgresSchemaSnapshot

    Caller->>Request: key + exact schemas + requested resource envelope
    Request->>Policy: resolve source key
    Policy-->>Request: immutable key + policy binding
    Request->>Policy: authorize exact schema scope against binding
    Policy-->>Request: allow / deny
    Request->>Policy: admit complete ObservationResourceEnvelope against same binding
    Policy-->>Request: allow / deny
    Note over Request,Policy: one non-resetting monotonic operation budget
    Request-->>Caller: AuthorizedObservationRequest or typed failure
    Caller->>Adapter: authorized envelope + cancellation
    Adapter->>Adapter: verify exact binding; read remaining budget
    Adapter->>Source: least-privilege read-only metadata access
    Source-->>Adapter: complete bounded catalog evidence
    Adapter->>Snapshot: authorized envelope + complete observations
    Snapshot-->>Adapter: immutable snapshot or fail closed
```

A source key, schema list, or positive resource limit is never authority on its own. Schema and complete resource-envelope policy default to deny, and both decisions are bound to the same immutable source-policy revision. A wider-than-policy resource request fails before adapter/source/snapshot side effects. The adapter may resolve credentials only from the exact authorized key-and-binding pair and cannot restart the original operation timeout.

## Generation -> publication -> client sequence

```mermaid
sequenceDiagram
    participant Source
    participant Observation
    participant Discovery
    participant Validator
    participant Steward
    participant Publisher
    participant Client
    participant Consumer as Consuming Product ACL

    Source->>Observation: immutable snapshot
    Observation->>Discovery: observations + evidence refs
    Discovery->>Validator: inferred candidate proposal
    Validator-->>Discovery: validation report
    Validator->>Steward: validated proposal
    Steward->>Publisher: reviewed acceptance
    Publisher-->>Source: no source mutation
    Publisher-->>Client: immutable versioned semantic_release + detached artifact digest
    Client->>Client: validate contract version + Published + Authoritative
    Client->>Client: verify_detached_artifact(exact bytes)
    Client-->>Consumer: admitted public release contract
    Consumer->>Consumer: tenant/purpose authorization + physical query planning/execution
```

## Client admission and integrity decision

```mermaid
flowchart TD
    R[Semantic release] --> V{Supported contract version?}
    V -- no --> X1[Reject: incompatible]
    V -- yes --> P{Publication state = Published?}
    P -- no --> X2[Reject: not published]
    P -- yes --> T{Truth status = Authoritative?}
    T -- no --> X3[Reject: not authoritative]
    T -- yes --> A[Admit for deterministic client operations]
    A --> H[verify_detached_artifact: hash exact detached bytes]
    H --> M{Digest equals declared artifact digest?}
    M -- no --> X4[Reject: artifact digest mismatch]
    M -- yes --> C[Integrity evidence established for supplied artifact bytes]
    C --> D[Consuming product performs tenant/purpose authorization]
```

Client admission is not publication authority and is not downstream authorization. `ReleaseDigest` validates canonical digest identity syntax; `SemanticReleaseClient::verify_detached_artifact` separately proves whether the exact detached artifact bytes supplied by the caller match that declared identity.