# PostgreSQL identifier fidelity: systemic Source Observation boundary

## Problem

ConceptWeave promises exact PostgreSQL source identifiers, but parts of `conceptweave-observation` still route identifier-bearing fields through `model::validate_nonblank()`, which rejects `value.trim().is_empty()`. That is presentation-oriented validation, not PostgreSQL identifier validation.

PostgreSQL 18 §4.1.1 defines quoted/delimited identifiers as arbitrary character sequences except code zero and explicitly permits spaces. Source Observation therefore must not collapse or reject legal whitespace-only catalog identifiers merely because their trimmed presentation is empty.

The earlier converter initial-privilege repair was intentionally bounded to ACL role and converter schema/function identifiers. It did not repair the shared v2/v3 observation validator.

## Current exact lineage

- Finding review: `5257069180` on #46 predecessor `427587e8be39d38240e57a2ebcc9cdaf291f6ebe`.
- Systemic structural RED: `a1638573b9d99f78c7fcd211fc93559d86d4f93a`, adding `postgresql_identifier_fidelity_contract.rs` for qualified type, collation, operator-class and column identifiers. It requires whitespace-only quoted identifiers to round-trip byte-for-byte and empty/code-zero identifiers to fail closed.
- Bounded column-identity RED: `9395d141c70f074a1b60ff1c5d26097973db768a`.
- Bounded production repair: `c2b247b24c45f9e45c90aad7b4fb68e296d0225e`, replacing trim-based admission only for `ColumnIdentityObservation` schema/relation/column coordinates with `empty || contains(code-zero)` rejection.
- Scope-completeness review: `5257371727` on exact predecessor `3b348facbba76f0a228aa677230864140979fe30`. The first systemic RED covered core v3 coordinate types but did not prove separately exported prefixed families that call the same shared validator.
- Prefixed-family structural RED: `3291d594da042d8497b1cc17493fc180f213f397`, adding `postgresql_identifier_fidelity_prefixed_contract.rs`. It requires `ColumnCollationObservation` and `ColumnExpressionObservation` schema/relation/column coordinates to preserve quoted whitespace and reject empty/code-zero identifiers while retaining whitespace-only rendered-expression rejection as a non-identifier negative control.

The systemic RED remains intentionally unresolved until the shared legacy/v3 identifier-bearing constructors stop using trim-based admission. The bounded column-identity repair and the new prefixed-family RED must not be interpreted as systemic GREEN.

## Invariant

For a field that is an exact PostgreSQL identifier decoded from catalog/source state:

1. preserve the complete string byte-for-byte, including whitespace;
2. reject zero-length values;
3. reject code zero;
4. do not case-fold, trim, Unicode-normalize or reinterpret quoting;
5. keep non-identifier fields such as rendered data-type/check-definition/expression text and extractor revision on their existing field-specific validation policy.

This separates identifier syntax from generic “nonblank text” policy and avoids widening unrelated product metadata contracts.

## Required acceptance

The shared production repair is complete only when current-head tests prove the exact behavior for schema, relation/table, column, constraint/reference, qualified type, collation, operator-class and other catalog identifier coordinates that currently use the shared trim-based validator. A repair that makes only the original four core-type assertions green is incomplete if separately exported coordinate families still reject legal quoted whitespace. At least one non-identifier negative control must remain in the same contract so a global weakening of `validate_nonblank()` cannot satisfy the suite.

The same exact head must pass Rust 1.98 fmt, strict Clippy, retained/workspace/doc tests, owned statement/branch/edge coverage, and a PostgreSQL 18 live differential with quoted whitespace identifiers and code-zero rejection.

No predecessor execution evidence transfers after source movement. Structural RED commits record executable contracts; they are not claims that a failing CI run was observed.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 4.1.1 Identifiers and key words*. PostgreSQL documents that quoted identifiers can contain any character except the character with code zero and specifically notes identifiers containing spaces.
