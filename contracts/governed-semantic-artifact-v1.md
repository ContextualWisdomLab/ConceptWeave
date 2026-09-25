# Governed semantic artifact v1

`publish` produces an immutable byte artifact from one validated alignment and an externally authorized steward decision. Its SHA-256 digest is the release's declared artifact digest. The release manifest binds that digest to the published release; clients obtain the manifest pin through an independent protected channel.

The byte format is ordered and length prefixed. `text(x)` is an unsigned 64-bit big-endian UTF-8 byte length followed by the exact bytes; `blob(x)` uses the same length prefix for arbitrary bytes; `count(n)` is an unsigned 64-bit big-endian count. Candidate and evidence collections sort by exact UTF-8 byte coordinates. The complete artifact cannot exceed 16 MiB.

1. `text("conceptweave.governed_semantic_artifact.v1")`, then release ID, contract version, and ontology version as text.
2. `blob(alignment)`, where alignment starts with `text("conceptweave.validated_alignment.v1")`, proposal ID, source digest, and candidate count. Each candidate contains its ID as text, a kind byte, a decision byte, decision fields as text, then evidence count and each evidence's source ID, source digest, and location as text. Candidates sort by ID. Kind tags are Concept `0`, TaxonomyRelation `1`, SemanticRelation `2`, Constraint `3`, Dimension `4`, Measure `5`, PhysicalMapping `6`. Map uses decision tag `1` followed by semantic ID, name, and rationale; Exclude uses tag `0` followed by rationale. Evidence sorts by `(source_id, source_digest, location)`. The alignment ends with counts for source-bound candidates, unique semantic IDs, mapped fields with concepts, and mapped relations with endpoints.
3. Steward ID, review rationale, immutable audit receipt ID, and alignment digest as text, in that order.

The authority adapter must authenticate the steward, authorize the exact proposal/source/alignment digest and decision, and issue an immutable audit receipt. This crate supplies no default authority adapter, durable storage, or protected pin distribution. A fixture authority is test evidence only.
