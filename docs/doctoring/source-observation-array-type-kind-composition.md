# Source Observation array/type-kind composition traceability

## Problem

Source Observation models PostgreSQL true-array identity and PostgreSQL type-kind evidence as separate optional catalog families. The true-array family preserves the exact qualified array `pg_type` coordinate together with its exact element coordinate from `pg_type.typarray`/`typelem`. The type-kind family separately preserves direct `pg_type.typtype`, domain base coordinates and reciprocal range/multirange evidence.

These families are independently valid, but exact review `5185362949` found that they do not currently compose. A snapshot created through `PostgresSchemaSnapshotV3::new_with_array_types(...)` can admit a column bound to an exact custom true-array coordinate such as `public._status`. Calling `with_observed_type_kinds(...)` on that immutable public snapshot re-validates the original qualified bindings through `validate_type_bindings_with_type_kinds()`, whose current resolver does not receive the already-observed array inventory. The same source fact can therefore become `UnknownTypeBinding` solely because another independent catalog family was attached.

Behavioral RED `ef9c8d61d0864e7b867902746969ea4d33592407`, refined ordinary-forward at `1cd78d9a9561aea9dc6d980bee59f31b9086b0d9`, adds `array_type_kind_composition_contract.rs`. The contract proves both sides of the boundary: exact array evidence plus coherent type-kind evidence must compose, while `PostgresTypeKind::Base` by itself must not invent array identity.

## Authoritative semantics

PostgreSQL 18 describes arrays as first-class container types and automatically creates an array type for each base, composite, range and domain type. Multidimensional values use that same array type rather than a distinct array-of-array type.

`pg_type.typarray` points from an element type to its associated true-array row, while the array row's `typelem` identifies the element type. True arrays are therefore exact catalog objects, not a naming convention. An array's direct `pg_type.typtype` classification does not replace the reciprocal `typarray`/`typelem` relationship that proves array identity.

The relevant primary references are:

- PostgreSQL Global Development Group. (2026). *The PostgreSQL Type System — Container Types*. https://www.postgresql.org/docs/18/extend-type-system.html
- PostgreSQL Global Development Group. (2026). *pg_type*. https://www.postgresql.org/docs/18/catalog-pg-type.html

## Repair contract

The minimum owner repair is compositional, not a new provider abstraction:

- when `with_observed_type_kinds(...)` is applied to a snapshot whose array family is already observed, exact qualified bindings proven by that immutable `ArrayTypeObservation` inventory remain resolvable;
- an arbitrary `PostgresTypeKind::Base` coordinate is not accepted as array evidence without the exact true-array family;
- no underscore-prefix naming inference, `search_path` inference or catalog OID enters governed identity;
- the array and type-kind observed families retain separate domain-separated digest layers;
- attaching type-kind evidence preserves the already-observed array inventory and adds new identity material rather than erasing or projecting it;
- unknown array coordinates remain fail closed.

The repair belongs in the public Source Observation aggregate's type-binding validation seam. It must not reinterpret frozen v2, merge true-array identity into the generic type-kind vocabulary, or weaken direct user-defined range/multirange support introduced by the temporal value-type repair.

## Acceptance

This slice remains behavioral-RED-active until production validation consumes the already-observed exact array inventory and the focused contract is GREEN. Native/Product acceptance then requires the unchanged exact successor to pass repository-pinned Rust 1.98 formatting, strict Clippy, workspace and doc tests, release build, owned coverage, and applicable Product/security/dependency/review gates. Predecessor execution evidence does not transfer across a moved head.
