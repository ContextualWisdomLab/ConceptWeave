# PostgreSQL transform-object extension-membership integrity

## Problem

ConceptWeave already preserves the extension membership of each nonzero transform **converter function**. That does not preserve the extension membership of the `pg_transform` object itself.

PostgreSQL 18 gives every transform row its own catalog object identity in `pg_transform`; the row is keyed semantically by transform type and target language and can reference FROM SQL and/or TO SQL converter functions. Independently, `ALTER EXTENSION name ADD TRANSFORM FOR type LANGUAGE lang` and the matching `DROP` form attach or detach that existing transform object from an extension. `ADD` makes the object an extension member that can subsequently be dropped only through the extension; `DROP` disassociates the object without dropping it. Therefore two source states can have the same type/language transform row and the same converter functions while differing in extension lifecycle, upgrade and dump/restore behavior.

## Decision

Source Observation preserves transform-object extension membership separately from converter-function membership.

`IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot` is layered on the current converter-function extension-membership digest and cross-checks the original same-generation transform-converter snapshot so the observation is one fact per exact `(constraint, key_position, transform_type, target_language)` transform row, not one fact per converter direction.

The extractor must:

- start from the exact same-generation `pg_transform` OID already resolved for the transform type/language row;
- query `pg_depend` for the transform object's extension-membership edge with `deptype='e'` and `objsubid=0`;
- resolve the referenced extension OID to the exact same-generation `pg_extension.extname`;
- report zero edge as standalone and one edge as the exact extension name;
- fail closed before contract construction if an unsupported ambiguous multiple-membership state is observed.

The contract does not infer membership from converter-function membership, extension naming, installed-package lists, function namespaces or application metadata. It also does not copy `pg_extension.extversion`, configuration tables, control-file fields or update-script truth.

## Alternatives rejected

**Reuse converter-function extension membership.** Rejected because PostgreSQL exposes `TRANSFORM FOR type LANGUAGE lang` as its own `ALTER EXTENSION` member object. Function membership and transform membership can therefore diverge.

**Infer membership from an installed extension or object naming convention.** Rejected because extension installation and object membership are distinct catalog facts.

**Duplicate extension-owned metadata into ConceptWeave.** Rejected because ConceptWeave owns the observed membership edge, not the extension's domain truth.

## Contract and evidence

Finding review: `5252885919` on exact head `a2ebefecb185f1c1ba0006fc9f118f5e25c4a9cc`.

Structural RED: `708761cc9ef5a14ea1201aa4c5d785463229e40d` adds a contract that references the not-yet-public transform-object membership types and requires standalone/member digest separation, one fact per transform row, completeness, language/type binding, input validation, exact receipts and collision-safe provenance.

Production observation/snapshot/receipt: `c055f0a351e0aec4bec6174327a9cd0cf8a9a4b0`.

Public composition: `0d543dc5cdc2bfe23a7ee11e961bc7510730c0bd`.

No execution GREEN is transferred from predecessor heads. The final exact head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production coverage, and a bounded PostgreSQL 18 same-generation live differential.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER EXTENSION*. https://www.postgresql.org/docs/18/sql-alterextension.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 52.22. pg_extension*. https://www.postgresql.org/docs/18/catalog-pg-extension.html

PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
