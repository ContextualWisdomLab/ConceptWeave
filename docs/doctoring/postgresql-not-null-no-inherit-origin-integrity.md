# PostgreSQL 18 NOT NULL `NO INHERIT` origin integrity

Status: Draft source-observation doctoring; acceptance remains exact-head gated.

## Problem

ConceptWeave records PostgreSQL 18 first-class `NOT NULL` constraints from `pg_constraint`. Before this repair, `NotNullConstraintObservation::new()` allowed an ordinary or foreign-table observation with `connoinherit=true` while `coninhcount>0`. That tuple contradicts PostgreSQL's inheritance semantics: `connoinherit` describes a locally defined non-inheritable constraint, while `coninhcount` counts direct inheritance ancestors. PostgreSQL 18 refuses changing an inherited constraint to `NO INHERIT` while inherited ancestry remains.

This matters because ConceptWeave assigns immutable governed identity to accepted source observations. Accepting an impossible `(connoinherit=true, coninhcount>0)` tuple would make caller-fabricated catalog state indistinguishable from source-representable PostgreSQL evidence.

## Decision

For PostgreSQL 18 relation-scoped `NOT NULL` observations:

- `connoinherit=true` requires `conislocal=true`;
- `connoinherit=true` requires `coninhcount=0`;
- a purely local ordinary-table or foreign-table `NOT NULL ... NO INHERIT` remains representable;
- partitioned-table `NOT NULL ... NO INHERIT` remains rejected by the stricter partitioned-table invariant already owned by this family;
- partition-child `conparentid` rules remain separate and unchanged.

The rule is intentionally one-way. A local constraint may be inheritable (`connoinherit=false`), and PostgreSQL documents that a constraint may be locally defined and inherited simultaneously. Therefore ConceptWeave does not collapse `conislocal`, `coninhcount`, and `connoinherit` into a single boolean.

## Alternatives considered

1. **Accept every catalog-shaped tuple.** Rejected because immutable source identity would admit states PostgreSQL 18's DDL path refuses.
2. **Require `conislocal == (coninhcount == 0)`.** Rejected because PostgreSQL explicitly permits a constraint to be both locally defined and inherited.
3. **Treat `NO INHERIT` as valid only for ordinary tables.** Rejected because PostgreSQL 18 `CREATE FOREIGN TABLE` also exposes `NOT NULL [ NO INHERIT ]`.
4. **Infer `NO INHERIT` from absence of children.** Rejected because inheritability is explicit source state, not a fact derivable from a bounded child inventory.

## Traceability

| Evidence | Exact coordinate | Role |
| --- | --- | --- |
| PR review | `#46` review `5197273298` | Verified source-integrity finding; explicitly records that the focused review was written after the same-run RED/fix rather than fabricating chronology |
| RED contract | `dc6fc29bb65b2324fa3a8c0f1e1f1ef0948aa853` | Reject inherited/nonlocal `NO INHERIT`; preserve purely local ordinary/foreign-table cases |
| Production repair | `9d20fc05453f4abde223c7b38fc3bdabc57626be` | Adds `no_inherit => conislocal && coninhcount == 0` before governed identity is admitted |
| Production module | `crates/conceptweave-observation/src/not_null_constraint.rs` | Source-coherence invariant |
| Contract | `crates/conceptweave-observation/tests/not_null_constraint_no_inherit_origin_contract.rs` | Focused edge contract |

`7031f4914e6b59fa262c98fea564f78c57b030ac..9d20fc05453f4abde223c7b38fc3bdabc57626be` is ordinary-forward, two commits ahead and zero behind. Its net delta is the focused contract plus eight production lines. This traceability is source evidence only; it is not Rust execution GREEN or hosted acceptance.

## Adapter obligation

The PostgreSQL adapter must read `conislocal`, `coninhcount`, and `connoinherit` from the same bounded catalog observation and pass their exact values. It must not derive `connoinherit` from child presence, normalize a nonzero `coninhcount` to zero, or repair contradictory catalog data before the domain validator sees it. A contradictory tuple fails closed and requires source/RCA evidence rather than silent coercion.

## Remaining boundary

Relation-level declarative-partition identity (`pg_class.relispartition` plus the direct `pg_inherits` parent and detach state) is a distinct open finding recorded in #46 review `5197218821`. The NOT NULL-specific `conparentid` witness must not be misused as a substitute for that general relation identity. Because the original v3 digest is frozen, that follow-up should use a domain-separated versioned observation family rather than mutating old v3 digest meaning.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-createforeigntable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source, REL_18_STABLE: `src/backend/commands/tablecmds.c`*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/tablecmds.c
