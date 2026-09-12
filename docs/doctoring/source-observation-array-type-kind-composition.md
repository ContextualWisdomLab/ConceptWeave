# Source Observation array/type-kind composition traceability

## Problem

Source Observation models PostgreSQL true-array identity and PostgreSQL type-kind evidence as separate optional catalog families. The true-array family preserves the exact qualified array `pg_type` coordinate together with its exact element coordinate from `pg_type.typarray`/`typelem`. The type-kind family separately preserves direct `pg_type.typtype`, domain base coordinates and reciprocal range/multirange evidence.

The first composition defect was that an array-aware snapshot could admit `public._status` through exact `ArrayTypeObservation`, then reject the same qualified binding when `with_observed_type_kinds(...)` revalidated it without the already-observed array inventory. That defect was repaired by making the public aggregate compose the two independently observed families.

Follow-up review `5185657110` found a second, distinct defect in the repaired resolver: it treated `PostgresTypeKind::Base` as if accepting the exact type coordinate would itself assert true-array identity. Consequently a normal user-defined base type such as `public.vector3` was rejected by `new_with_type_kinds(...)`, even though the same source-authoritative `pg_type.typtype = 'b'` evidence is sufficient to establish that exact type row as a binding coordinate. Behavioral RED `670fd280d32a1fcfa2cf776bdd779812e10d31bf` corrects the contract: exact Base coordinates must resolve as types while the array family remains unobserved unless reciprocal true-array evidence was separately captured.

Production repair `058b434640816a36914923803b6c4946733ec483` admits exact Base/Range/Multirange type-kind coordinates in the public binding resolver and compatibility-projects those user-defined coordinates only for the private legacy v3 validator. The governed public relation/domain bindings and type-kind evidence remain unchanged and continue to participate in the successor digest. No underscore-prefix or display-text inference is introduced.

## Authoritative semantics

PostgreSQL 18 describes arrays as first-class container types and automatically creates an array type for each base, composite, range and domain type. Multidimensional values use that same array type rather than a distinct array-of-array type.

`pg_type.typtype = 'b'` classifies a base type row. That classification establishes the observed type kind of the exact qualified `pg_type` coordinate; it does not say that the row is a true array. PostgreSQL exposes true-array identity separately: an element type's `typarray` identifies its associated array type and the array type's `typelem` identifies its element. PostgreSQL documentation also warns callers not to depend on the conventional underscore-prefixed generated array name because conflicts can change the actual name.

The relevant primary references are:

- PostgreSQL Global Development Group. (2026). *The PostgreSQL Type System — Container Types*. https://www.postgresql.org/docs/18/extend-type-system.html
- PostgreSQL Global Development Group. (2026). *pg_type*. https://www.postgresql.org/docs/18/catalog-pg-type.html

## Decision and repair boundary

The source-observation contract separates two questions that PostgreSQL itself exposes separately:

- **Does this exact qualified type coordinate exist with a source-observed kind?** `TypeKindObservation` answers this. A source-observed Base coordinate is therefore a valid type binding just as an observed Range or Multirange coordinate is.
- **Is this exact type coordinate the true array for another type?** Only `ArrayTypeObservation` answers this from reciprocal `typarray`/`typelem` evidence.

Accordingly:

- `new_with_type_kinds(...)` may resolve an exact user-defined Base, Range or Multirange binding from its source-authoritative type-kind observation;
- the private compatibility representation may receive a bounded built-in projection for those user-defined coordinates, but the public immutable aggregate retains and digests the original exact binding and observed kind;
- `PostgresTypeKind::Base` never creates an `ArrayTypeObservation`, array source receipt or array semantic claim;
- an array-aware snapshot that already carries exact array evidence continues to compose with coherent type-kind evidence;
- no underscore-prefix naming rule, `search_path`, OID or rendered type text participates in governed identity;
- range/multirange temporal semantics still require Range/Multirange (or a domain chain resolving to them); a Base observation does not satisfy `WITHOUT OVERLAPS`/`PERIOD` value-type validation merely because Base is now a resolvable binding.

This keeps array identity, type-kind identity and temporal range semantics as separate bounded evidence families rather than collapsing them into one heuristic vocabulary.

## Acceptance

The behavioral contract now covers three cases: already-observed custom true arrays compose with type-kind evidence; Base-only evidence resolves its exact coordinate without inventing array identity; and an ordinary user-defined Base coordinate resolves directly. Source repair is present at `058b434640816a36914923803b6c4946733ec483`.

Native/Product acceptance still requires one unchanged exact successor head to pass repository-pinned Rust 1.98 formatting, strict Clippy, workspace and doc tests, release build, owned coverage, and applicable Product/security/dependency/review gates. Predecessor execution evidence does not transfer across a moved head.
