# Changelog

All notable ConceptWeave changes through exact `f43ec0d9ae878b0cad292fc9f927c63350f8c2c3` are preserved losslessly at `docs/archive/CHANGELOG-through-f43ec0d9.md`; earlier release-era and Source Observation history remains in `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now binds the executable implementation definition behind each exact governed `pg_operator.oprcode -> pg_proc` function in `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`. The successor independently resolves the implementation language and consumes same-row `pg_proc.prosrc`, optional `probin`, and optional `prosqlbody`; plaintext implementation material is immediately reduced to a domain-separated SHA-256 identity and is not retained in receipts. Changes to language, source/link symbol, binary reference, or pre-parsed SQL body therefore cannot collapse to the same governed evidence identity while the stable function signature remains unchanged.
- The definition successor is layered over the exact procedure-leakproof predecessor and retains all prior ordinary-EXCLUDE operator/function, security, result/cardinality, timing, backing-index, access-method, namespace, lifecycle, and source-generation controls without rewriting predecessor digest domains.
- Ordinary `EXCLUDE` implementation-function evidence now also preserves exact `pg_proc.proowner` plus the independently resolved `pg_roles.oid -> rolname` identity in `IndexExclusionConstraintOperatorProcedureOwnerSnapshot`. Raw owner OID and resolved role name are both domain-separated successor material, so ownership transfer, role rename, and dropped/recreated same-name role states do not silently collapse while the function signature and body remain unchanged.

### Security

- Function bodies and object-file paths are not copied into downstream provenance. Only resolved language identity and a content digest are retained after construction. This closes the executable-definition collision while avoiding unnecessary disclosure of proprietary function source or deployment paths.
- Raw `pg_proc.prosqlbody` `pg_node_tree` text is treated only as exact source-generation evidence. ConceptWeave does not claim that PostgreSQL internal node serialization is a cross-major semantic canonical form.
- Function ownership is observed rather than inferred. The owner successor reads nonzero same-row `pg_proc.proowner` and resolves it through the public `pg_roles` view; it does not treat schema ownership, session identity, or a function name as execution-principal evidence. PostgreSQL's existing `prosecdef` fact remains independent and no SECURITY-DEFINER-only product rule is introduced.

### Retained

- All ordinary-EXCLUDE source-integrity repairs archived through `docs/archive/product-technical-gap-baseline-through-f43ec0d9.md` remain authoritative, including exact constraint/backing-index identity; namespace/name/role/immediacy; access-method exclusion capability; catalog-family shape; ordered `conkey`/`conexclop`; raw binary `oprkind`; self-commutator; exact `oprcode -> pg_proc` function; independent `oprresult`/`prorettype` with exact `pg_catalog.bool`; scalar `proretset=false`; raw `proisstrict`, `provolatile`, `proparallel`, `prokind='f'`, `prosecdef`, and `proleakproof`; implementation-definition identity; backing-index `pg_class.relnamespace`; ready/valid/live lifecycle; and exact v3 source-content generation binding.

### Acceptance

- The implementation-definition and function-owner repairs and all retained procedure/operator repairs are source-shaped repairs, not an executed GREEN claim. One unchanged exact head must still pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc and owned coverage, applicable hosted gates, and a bounded PostgreSQL 18 live differential. That differential must independently read exact `pg_operator` facts, then exact `pg_proc.proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, and `prosqlbody`; resolve `proowner` to the exact same-generation `pg_roles.oid -> rolname` and `prolang` to `pg_language.lanname`; and keep all retained backing-index/operator-family controls in the same v3 source-content generation.
