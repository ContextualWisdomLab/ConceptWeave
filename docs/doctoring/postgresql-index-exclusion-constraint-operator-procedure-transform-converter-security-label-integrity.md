# PostgreSQL transform-converter security-label integrity

## Decision

ConceptWeave Source Observation must preserve the complete same-generation PostgreSQL security-label map for every exact FROM SQL / TO SQL transform converter function. Security labels are a separate security fact from owner, discretionary EXECUTE ACL, `SECURITY DEFINER`, leakproofness, extension membership, and `DEPENDS ON EXTENSION` lifecycle edges.

The observation boundary stores exact `(provider, label)` text for `pg_seclabel` rows whose `classoid` identifies `pg_proc`, whose `objoid` is the exact converter function OID selected by the bounded `pg_transform` row, and whose `objsubid=0`. It does not interpret provider policy and does not normalize label text. Provider names form the map key and must be unique per function; an empty label string remains observable if the loaded provider accepts it.

## Problem and consequence

At pre-finding exact head `106cfb3f1278afe12f7f126b4458a21649745e05`, the converter chain preserved definition, owner, ACL, configuration, `SECURITY DEFINER`, leakproofness, strictness, volatility, parallel safety, planner support, cost, function shape, transform types, extension membership, and auto-extension dependencies. It did not preserve `pg_seclabel`.

PostgreSQL 18 permits an arbitrary number of security labels on a database object, one per registered label provider. Provider modules own both label validity and label semantics. The built-in `sepgsql` integration demonstrates that function labels can participate in mandatory-access-control decisions and can determine trusted-procedure behavior. Consequently two otherwise identical converter functions can differ materially in authorization behavior while collapsing to the same governed Source Observation successor if security labels are omitted.

## Alternatives considered

1. Treat EXECUTE ACL or `SECURITY DEFINER` as a proxy. Rejected: label-based MAC is additional to PostgreSQL discretionary privileges and may govern function execution independently.
2. Interpret known providers such as `selinux` inside ConceptWeave. Rejected: PostgreSQL explicitly delegates label validity and meaning to the provider. Copying provider policy would violate owner boundaries and make unknown providers lossy.
3. Preserve only a boolean `has_security_label`. Rejected: provider identity and exact provider-owned label text can change security semantics and must remain distinguishable.
4. Preserve the complete provider/label map as raw Source Observation evidence. Selected.

## Contract

Finding review `5254227769` established the security distinction. Structural RED `49059a31e9445f364fca190dba5b049a440e78e1` referenced the not-yet-existing public security-label contract. Production `58abc98906919ca803adb4bbe3f197df61ffc37c` added `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel`, observation, immutable receipt, and snapshot. Public composition followed at `0b1a513c6c4ca50353160b986e70a088381360e6`.

The contract requires one explicit observation for every exact converter direction in the auto-extension-dependency predecessor. Observations retain exact converter schema/function binding and the immutable raw `converter_snapshot_digest`. Provider ordering is canonicalized because row order is not semantic, but provider text and label text remain exact. Duplicate providers, missing/extra/duplicate converter coordinates, converter binding drift, blank provider identity, zero key position, unknown receipt coordinates, and provenance-location collisions fail closed.

Transform-object extension membership was ordinary-forward restacked on the security-label successor at `03d434c88e1251d6d28977be4805738c374d183b`. Retained contract and hostile lineage tests were restacked at `e764a73751bb88fd362fbc5af4948691d8dd02aa` and `c52fce2d4ea782e83bae4b31636335ad8c417a06`. Test correction `f4c0ce73237062b2a28b2f0ee69731d72ab6853b` exercises duplicate-provider rejection at the observation boundary rather than masking the constructor error through a helper unwrap.

## Same-generation extraction rule

For every exact selected converter function OID, the PostgreSQL 18 differential must read `pg_seclabel` from the same bounded source generation used for the converter `pg_proc`, `pg_depend`, and `pg_transform` facts. Capture requires `classoid=pg_proc`, exact `objoid`, and `objsubid=0`; it resolves the complete provider/label set without relying on ACL, function names, extension state, operating-system configuration, or another snapshot. An absent row set means no observed security labels. Provider policy itself remains outside ConceptWeave.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: SECURITY LABEL*. https://www.postgresql.org/docs/18/sql-security-label.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.46. pg_seclabel*. https://www.postgresql.org/docs/18/catalog-pg-seclabel.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: F.40. sepgsql — SELinux-, label-based mandatory access control (MAC) security module*. https://www.postgresql.org/docs/18/sepgsql.html

## Acceptance and residual risk

Source repair is not execution GREEN. The final exact head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and a bounded PostgreSQL 18 same-generation live differential including security-label capture. No predecessor execution result transfers after source or documentation movement.

No subsequent converter successor is authorized merely by locating another catalog field. A new successor requires a separately demonstrated buyer, semantic, security, lifecycle, recovery, or provenance distinction not already represented by the current chain.