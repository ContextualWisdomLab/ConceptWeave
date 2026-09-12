# Source Observation — cross-schema type-kind authorization

## Problem

`PostgresSchemaSnapshotV3::new_with_type_kinds(...)` admits exact qualified PostgreSQL type coordinates that the frozen private v3 representation cannot resolve directly. The prior admission logic treated “this schema already has a represented relation/domain/enum” as a proxy for whether a `TypeKindObservation` schema was legitimate. That proxy rejects valid cross-schema qualified bindings such as `app.embedding_fact.embedding -> types.vector3` even when the bounded observation request explicitly authorizes both `app` and `types`.

The opposite failure is also unacceptable: removing the schema check entirely would allow source evidence from a schema outside the authorized request scope.

## PostgreSQL authority

PostgreSQL 18 states that schemas apply to named objects including data types, and that data type names can be schema-qualified in the same way as table names. `search_path` is used only when a data type is referenced without an explicit schema; an object outside the search path can still be referenced by its qualified dotted name. PostgreSQL also stores a type's namespace directly as `pg_type.typnamespace -> pg_namespace.oid`, so the source-authoritative identity is the exact namespace/name coordinate rather than the presence of some unrelated modeled object in that namespace.

References:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 5.10 Schemas*. https://www.postgresql.org/docs/18/ddl-schemas.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 19.11 Client connection defaults (`search_path`)*. https://www.postgresql.org/docs/18/runtime-config-client.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.64 `pg_type`*. https://www.postgresql.org/docs/18/catalog-pg-type.html

## Decision

Direct `new_with_type_kinds(...)` admission uses the exact `AuthorizedObservationRequest.request().allowed_schema_names()` as an additional schema authority. A non-`pg_catalog` type-kind coordinate is admissible when its schema is explicitly authorized by that request, even if the bounded relation/domain/enum inventory contains no other object in that schema. An unapproved coordinate remains fail-closed as `type_kind_schema`.

The existing consuming `with_observed_type_kinds(...)` path does not receive a fresh observation request and therefore retains its prior bounded-inventory rule. That keeps the change causal: the direct constructor, which is specifically the compatibility seam for otherwise-unresolvable exact Base/Range/Multirange bindings, gains the request authority it already receives; the consuming path is not silently widened.

`pg_catalog` remains the built-in special case. No `search_path`, display text, OID, underscore naming convention, or inferred local-schema relation is introduced as semantic identity. Base-type admission does not create true-array evidence and does not satisfy `PERIOD`/`WITHOUT OVERLAPS` range semantics; those remain owned by their separate observed families.

## RED → repair traceability

- PR: ConceptWeave #46
- Finding review: `5185712815` on exact `7e236102ef6d09c70b4bcffa119039913fa01094`
- Behavioral RED: `c166247a2ae0604a81f315994a66b199dd98a183`
  - authorized `app -> types.vector3` qualified binding must succeed when both schemas are in the request allowlist;
  - the same coordinate must remain fail-closed when `types` is outside that allowlist.
- Production repair: `050a2ad679bf2246046638be1e8b89692ee8c251`
  - only `crates/conceptweave-observation/src/lib.rs` changes;
  - direct type-kind canonicalization receives the exact request allowlist;
  - consuming type-kind canonicalization explicitly passes no widened allowlist.

## Acceptance

Source repair is not native/Product acceptance. The exact successor head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy with warnings denied, workspace and doc tests including `array_type_kind_composition_contract`, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review terminal evidence. No predecessor execution result transfers after head movement.
