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

## Foundation sequence

```mermaid
sequenceDiagram
    participant Source
    participant Observation
    participant Discovery
    participant Validator
    participant Steward
    participant Publisher

    Source->>Observation: immutable snapshot
    Observation->>Discovery: observations + evidence refs
    Discovery->>Validator: inferred candidate proposal
    Validator-->>Discovery: validation report
    Validator->>Steward: validated proposal
    Steward->>Publisher: reviewed acceptance
    Publisher-->>Source: no source mutation
    Publisher-->>Steward: immutable release receipt
```

## Research intake sequence

```mermaid
sequenceDiagram
    participant Zotero as Zotero Local API
    participant Intake as Research intake
    participant Report as Local proposal report
    participant Steward

    loop bounded pages
        Intake->>Zotero: read items at one library version
        Zotero-->>Intake: items + observed library version
        Intake->>Intake: validate page consistency and every item revision; fail entire read on mismatch
    end
    Intake->>Intake: classify or abstain; link children; find duplicate candidates
    Intake->>Report: write proposals and evidence
    Report->>Steward: review dispositions and merge candidates
    Intake-->>Zotero: no mutation
```
