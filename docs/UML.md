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
