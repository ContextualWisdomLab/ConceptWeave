# PostgreSQL transform-converter initial-privilege integrity

## Decision

ConceptWeave Source Observation preserves converter-function `pg_init_privs` independently from current `pg_proc.proacl`, extension membership/dependency, security labels, function definition, and `pg_transform` binding.

The admitted distinction is recovery semantics, not catalog enumeration. PostgreSQL records non-default initial privileges for an object in `pg_init_privs`. Initial privileges can originate from `initdb` (`privtype='i'`) or from GRANT/REVOKE executed while an extension is created (`privtype='e'`). For extension objects, `pg_dump` emits `CREATE EXTENSION` and then the GRANT/REVOKE operations needed to reproduce the object's current privilege state relative to that stored initial baseline. Two converter functions can therefore have the same current ACL and every previously observed converter fact while requiring different dump/restore privilege reconstruction.

Finding review `5254405649` pins the lossiness at exact pre-finding #46 head `71c86b196ab1fa2e66143d8f0f708a2606ace724`.

## Owner and bounded context

ConceptWeave owns observation of this PostgreSQL source fact because it changes the immutable semantic evidence used by the `observe -> discover -> propose -> align -> validate -> review -> publish` chain. It does not own extension package files, role-membership policy, enterprise authorization policy, or restoration orchestration.

The existing converter current-ACL successor remains authoritative for `pg_proc.proacl`. The new initial-privilege successor does not infer or duplicate that current ACL. The two facts answer different questions:

- current ACL: what PostgreSQL currently authorizes at the function object boundary;
- initial privilege baseline: what PostgreSQL records as the baseline used for extension privilege dump/restore semantics.

## PostgreSQL source contract

For every exact FROM SQL / TO SQL converter function selected from the same source generation, the extractor must query the function object's `pg_init_privs` identity using:

- `classoid = pg_proc`;
- `objoid =` the exact converter function OID already selected by the transform binding;
- `objsubid = 0`.

Absence of a row is an observed fact and must remain distinct from a present row whose `initprivs` ACL is empty. A present row must preserve exact `privtype` (`i` or `e`) and the complete object-level EXECUTE ACL. Non-PUBLIC grantor/grantee OIDs are resolved to exact role names in the same source generation before entering the domain boundary. Grant ordering is not semantic and is canonicalized; duplicate ACL entries fail closed.

Current `proacl`, extension name, `deptype='e'`, `deptype='x'`, package inventory, function name, or security label is not a valid proxy for `pg_init_privs`.

## Implementation traceability

- Finding review: `5254405649`.
- Structural RED: `6a16b6c314e9ba6f9b8a2038055713c62e4cefab`. The integration contract referenced the initial-privilege public API before that API existed.
- Production successor: `c94caa8a1da7088c819b802a87588ba303edcf55` adds initial privilege type, canonical initial EXECUTE grant material, observation, immutable receipt, snapshot, exact binding/completeness checks, raw-root propagation, and domain-separated digests.
- Public composition: `1ba7f86bb62c399f334b823ed51bad651e398f24` exposes the successor from `index_partition`.
- Transform-object lifecycle restack: `50dddc811bacc301fc36f0d7b5bddd3a6a51fd18` makes initial privileges the digest predecessor while retaining the independently supplied raw transform-converter root check.
- Transform-object contract restack: `34c8611f0c75723bdebc82e0e84fa67a92856604` proves that a converter initial-privilege change survives into the transform-object lifecycle digest.
- Retained direction/function/raw-root lineage restack: `a1c35b715e56f5a62813ba60e65d0ba6a80ba418` keeps hostile same-generation direction, function-name, and function-definition drift checks on the current predecessor shape.

The structural RED is evidence of the missing contract at that historical commit only. It is not GREEN evidence for any successor head.

## Invariants

1. Every converter direction in the security-label predecessor has exactly one initial-privilege observation, including explicit row absence.
2. Coordinates, transform type, direction, converter schema, and converter function must match the predecessor exactly.
3. `converter_snapshot_digest` propagates unchanged from the immutable raw transform-converter root.
4. Present `pg_init_privs` material distinguishes `privtype='i'` from `privtype='e'` even when ACL entries are identical.
5. ACL entry order is canonicalized; PUBLIC versus a named role, exact grantor, and grant option remain identity-bearing.
6. Duplicate ACL entries, blank resolved role identities, missing/extra/duplicate coordinates, binding drift, zero key positions, unknown receipt locations, and provenance-location collisions fail closed.
7. Transform-object extension membership consumes this successor digest so the recovery distinction cannot disappear later in the chain.

## Alternatives considered

### Reuse current `proacl` only

Rejected. Identical current ACL does not imply identical `pg_init_privs`. PostgreSQL explicitly stores the latter as a separate initial baseline and uses it in extension dump/restore privilege reconstruction.

### Infer initial privileges from extension membership

Rejected. Extension membership says that an object belongs to an extension; it does not encode whether an extension script changed initial privileges or what those privileges were.

### Preserve raw `aclitem[]` text

Rejected at the domain boundary. Raw OID-bearing ACL text is source-format identity rather than stable semantic identity across equivalent source instances. The existing access-control pattern resolves role identities in the same source generation and hashes canonical object-level grants. The same pattern is used here while preserving row absence and `privtype` separately.

### Copy extension package/control/update scripts

Rejected as a canonical-owner violation. Those artifacts are extension-owned package truth, not ConceptWeave Source Observation truth.

## Risks and follow-up

The current source repair is not accepted until the exact final head has repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and a bounded PostgreSQL 18 live differential.

The differential must demonstrate at least one same-generation case in which the current function ACL is held constant while the initial privilege baseline differs, and must verify row absence, `privtype`, complete initial EXECUTE ACL resolution, immutable raw-root lineage, and downstream transform-object digest propagation. Synthetic data remains suitable for unit contracts only; acceptance needs live PostgreSQL catalog evidence.

## Primary sources

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 36.17. Packaging related objects into an extension*. https://www.postgresql.org/docs/18/extend-extensions.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: GRANT*. https://www.postgresql.org/docs/18/sql-grant.html

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: pg_dump*. https://www.postgresql.org/docs/18/app-pgdump.html
