# PostgreSQL transform-converter function extension-membership integrity

## Decision

ConceptWeave Source Observation preserves whether each exact FROM-SQL or TO-SQL converter function is a member of a PostgreSQL extension. The fact is captured from the converter function's `pg_depend` extension-membership edge and resolved to the referenced `pg_extension.extname` in the same source generation.

This is a lifecycle/operability fact, not another rendering of `pg_proc`. A converter function can keep the same schema/name, exact input and return contract, implementation, owner, ACL, configuration, security flags, planner/cost facts, argument metadata, `protrftypes`, and `pg_transform` binding while `ALTER EXTENSION ... ADD FUNCTION` or `DROP FUNCTION` associates or disassociates that existing function with an extension.

## PostgreSQL 18 authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER EXTENSION*. https://www.postgresql.org/docs/18/sql-alterextension.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 36.17. Packaging related objects into an extension*. https://www.postgresql.org/docs/18/extend-extensions.html

`pg_depend.deptype='e'` means the dependent object is a member of the referenced extension. PostgreSQL states that such an object can be dropped only through the extension. `ALTER EXTENSION ... ADD FUNCTION` attaches an existing function to an extension and `ALTER EXTENSION ... DROP FUNCTION` disassociates it without dropping the function. Extension member objects are also treated specially by `pg_dump`; their individual definitions are not dumped as independent objects.

The same `ALTER EXTENSION` grammar separately supports `TRANSFORM FOR type LANGUAGE language` as an extension member object. That is a different edge from converter-function membership. This successor captures only the converter function's membership; transform-object extension membership remains a separate semantic review and must not be inferred from the function edge.

## Capture contract

For the exact converter function OID already resolved by the predecessor chain, the PostgreSQL adapter must inspect `pg_depend` in the same source snapshot with:

- dependent class = `pg_proc`;
- dependent object = the exact converter function OID;
- dependent sub-object = zero;
- referenced class = `pg_extension`;
- dependency type = extension membership (`e`);
- referenced extension OID resolved to exact `pg_extension.extname` in the same generation.

No matching edge is represented as `None`. One matching edge is represented as the exact extension name. The extractor must fail closed rather than arbitrarily choose an extension if source corruption or an unsupported state exposes more than one matching membership edge.

The successor does not copy `pg_extension.extversion`, extension configuration tables, control-file metadata, update scripts, or any other extension-owned truth. Those remain separate owner facts. Its only claim is the exact lifecycle edge binding the already-resolved converter function to an extension.

## Governed effect

Omitting this edge made two operationally different database states share the same ConceptWeave successor digest: a standalone converter function and an otherwise identical function owned as an extension member. That difference changes independent-drop behavior, extension-drop behavior, upgrade ownership, and dump/restore treatment. It therefore meets the project's criterion for an independent governed distinction.

`IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot` is layered over the converter-function transform-selection snapshot. It requires exact predecessor coordinate coverage and exact converter schema/function binding, preserves absence separately from a resolved extension name, and domain-separates the new digest without rewriting predecessor domains.

## Traceability

- Finding review: PR #46 review `5252657912` on exact predecessor `e2e257d1c498a906dc844060d1e3b8cf2303967a`.
- RED contract: `ddc04f03c97ae78cccddefe34bd656aec32bd688`.
- Production observation/snapshot/receipt: `30976b643aa96203a203705a75100e6374ff4977`.
- Public composition: `64080cdc8188ca951f73faa95d6f9f656e6860dc`.

Source repair is not acceptance evidence. The final exact head still needs repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 differential. The differential must resolve extension membership from the same source generation as the exact converter `pg_proc` row rather than from application metadata, package names, function naming conventions, or another snapshot.
