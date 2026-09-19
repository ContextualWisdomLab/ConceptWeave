# PostgreSQL transform-converter initial-privilege integrity

## Decision

ConceptWeave Source Observation preserves converter-function `pg_init_privs` independently from current `pg_proc.proacl`, extension membership/dependency, security labels, function definition, and `pg_transform` binding.

The admitted distinction is recovery semantics, not catalog enumeration. PostgreSQL records non-default initial privileges for an object in `pg_init_privs`. Initial privileges can originate from `initdb` (`privtype='i'`) or from GRANT/REVOKE executed while an extension is created (`privtype='e'`). For extension objects, `pg_dump` emits `CREATE EXTENSION` and then the GRANT/REVOKE operations needed to reproduce the object's current privilege state relative to that stored initial baseline. Two converter functions can therefore have the same current ACL and every previously observed converter fact while requiring different dump/restore privilege reconstruction.

PostgreSQL 18 also makes ACL array order recovery-significant. `src/include/utils/acl.h` states that client utilities, particularly `pg_dump`, expect to issue GRANTs in ACL-array order because grants with grant option must precede grants that depend on them. Source Observation therefore preserves the exact `pg_init_privs.initprivs` array sequence; it must not sort ACL entries merely to obtain deterministic hashing.

ACL multiplicity is likewise part of the observed array. PostgreSQL 18 `check_acl()` validates that an ACL is a one-dimensional `aclitem[]` with no nulls; it does not impose a uniqueness invariant. `aclnewowner()` explicitly documents that an ACL can contain duplicate entries after owner substitution and contains dedicated merge/removal logic for that state. ConceptWeave therefore must not reject a source row solely because two observed ACLITEMs are identical. It preserves repeated entries and their positions in the immutable material/digest while keeping only the derived dangling-role remediation OID sets canonicalized.

Initial finding review `5254405649` pins the original missing `pg_init_privs` fact at exact pre-finding #46 head `71c86b196ab1fa2e66143d8f0f708a2606ace724`. Source-order integrity finding review `5255912747` pins the later normalization defect at exact pre-finding head `4967f12cb90ac41ec42263bfed47ba6bcbb49735`. ACL-multiplicity finding review `5256059436` pins the later lossless-observation defect at exact pre-finding head `50fff94eb567fd9b1c2150d4570ca1422acc9a60`.

## Owner and bounded context

ConceptWeave owns observation of this PostgreSQL source fact because it changes the immutable semantic evidence used by the `observe -> discover -> propose -> align -> validate -> review -> publish` chain. It does not own extension package files, role-membership policy, enterprise authorization policy, or restoration orchestration.

The existing converter current-ACL successor remains authoritative for `pg_proc.proacl`. The initial-privilege successor does not infer or duplicate that current ACL. The two facts answer different questions:

- current ACL: what PostgreSQL currently authorizes at the function object boundary;
- initial privilege baseline: what PostgreSQL records as the baseline used for extension privilege dump/restore semantics.

## PostgreSQL source contract

For every exact FROM SQL / TO SQL converter function selected from the same source generation, the extractor must query the function object's `pg_init_privs` identity using:

- `classoid = pg_proc`;
- `objoid =` the exact converter function OID already selected by the transform binding;
- `objsubid = 0`.

Absence of a row is an observed fact and must remain distinct from a present row whose `initprivs` ACL is empty. A present row must preserve exact `privtype` (`i` or `e`) and the complete object-level EXECUTE ACL. Non-PUBLIC grantor/grantee OIDs are resolved to exact role names in the same source generation before entering the domain boundary. If catalog damage leaves a nonzero OID unresolved, Source Observation retains it as a typed unresolved identity for recovery governance rather than dropping or stringifying it.

ACL entry order and multiplicity are source identity. The exact `pg_init_privs.initprivs` sequence is preserved even when identical ACLITEMs repeat. Source Observation does not normalize, sort, merge, or reject those repeated entries. The sorted/deduplicated dangling-role OID collections used for remediation are derived diagnostic sets only; they do not replace, reorder, or deduplicate the source ACL.

Current `proacl`, extension name, `deptype='e'`, `deptype='x'`, package inventory, function name, or security label is not a valid proxy for `pg_init_privs`.

## Implementation traceability

- Initial fact finding review: `5254405649`.
- Structural RED: `6a16b6c314e9ba6f9b8a2038055713c62e4cefab`. The integration contract referenced the initial-privilege public API before that API existed.
- Production successor: `c94caa8a1da7088c819b802a87588ba303edcf55` adds initial privilege type, initial EXECUTE grant material, observation, immutable receipt, snapshot, exact binding/completeness checks, raw-root propagation, and domain-separated digests.
- Public composition: `1ba7f86bb62c399f334b823ed51bad651e398f24` exposes the successor from `index_partition`.
- Transform-object lifecycle restack: `50dddc811bacc301fc36f0d7b5bddd3a6a51fd18` makes initial privileges the digest predecessor while retaining the independently supplied raw transform-converter root check.
- Transform-object contract restack: `34c8611f0c75723bdebc82e0e84fa67a92856604` proves that a converter initial-privilege change survives into the transform-object lifecycle digest.
- Retained direction/function/raw-root lineage restack: `a1c35b715e56f5a62813ba60e65d0ba6a80ba418` keeps hostile same-generation direction, function-name, and function-definition drift checks on the current predecessor shape.
- Source-order integrity finding review: `5255912747` at exact pre-finding head `4967f12cb90ac41ec42263bfed47ba6bcbb49735`.
- Source-order structural RED: `13e51561cac125f2a942e7ca0d0dd39433040426` requires swapped ACL arrays to produce distinct material identity and requires `grants()` to return the observed order. This is a source-level RED and is not claimed as an executed failing run.
- Source-order production repair: `966e2136c15aa049f72acfebf2c1a478d09c66ef` removes `grants.sort()` and hashes/retains grants in source order.
- ACL-multiplicity finding review: `5256059436` at exact pre-finding head `50fff94eb567fd9b1c2150d4570ca1422acc9a60`.
- Corrected ACL-multiplicity structural RED: `9747763c4b2370185bd18039b00e8ceaa9df206c` requires `[grant, grant]` to remain two source entries and have a different digest from `[grant]`. The earlier test-only `3ece5938...` contained a type-reference typo and is superseded; neither test-only commit is claimed as an executed failing run.
- ACL-multiplicity production repair: `55f828f6f6065fac89ae63f521d72d807f875c5b` removes the source-level uniqueness rejection while retaining sorted/deduplicated derived dangling-role remediation sets.

The structural REDs are evidence of missing contracts at their historical commits only. They are not GREEN evidence for any successor head.

## Invariants

1. Every converter direction in the security-label predecessor has exactly one initial-privilege observation, including explicit row absence.
2. Coordinates, transform type, direction, converter schema, and converter function must match the predecessor exactly.
3. `converter_snapshot_digest` propagates unchanged from the immutable raw transform-converter root.
4. Present `pg_init_privs` material distinguishes `privtype='i'` from `privtype='e'` even when ACL entries are otherwise identical.
5. ACL entry order and multiplicity are identity-bearing and remain exactly source-observed; PUBLIC versus a named role, unresolved role identity, exact grantor, and grant option are also identity-bearing.
6. Blank resolved role identities, missing/extra/duplicate converter coordinates, binding drift, zero key positions, unknown receipt locations, and provenance-location collisions fail closed without normalizing the source ACL. Repeated ACLITEM values inside one observed ACL do not fail Source Observation.
7. Transform-object extension membership consumes this successor digest so the recovery distinction cannot disappear later in the chain.

## Alternatives considered

### Reuse current `proacl` only

Rejected. Identical current ACL does not imply identical `pg_init_privs`. PostgreSQL explicitly stores the latter as a separate initial baseline and uses it in extension dump/restore privilege reconstruction.

### Infer initial privileges from extension membership

Rejected. Extension membership says that an object belongs to an extension; it does not encode whether an extension script changed initial privileges or what those privileges were.

### Sort ACL entries before hashing

Rejected. Determinism is not permission to erase source semantics. PostgreSQL's ACL implementation documents array order as significant to client utilities because dependent GRANT execution can require grant-option providers first. Hashing the observed vector is already deterministic for a fixed observation and preserves this recovery-relevant order.

### Reject or merge duplicate ACL entries at observation time

Rejected. PostgreSQL's own `check_acl()` does not define uniqueness as ACL validity, and upstream code explicitly contains duplicate-entry reconciliation for states that can arise during owner substitution. Observation is not remediation. Normalizing repeated ACLITEMs would erase source multiplicity before validation/recovery governance can inspect it.

### Preserve only raw `aclitem[]` text

Rejected at the domain boundary. Raw OID-bearing ACL text is not sufficient semantic identity across equivalent live-role resolutions and does not provide typed recovery governance for dangling OIDs. The domain model resolves live role identities in the same source generation, preserves unresolved nonzero OIDs as typed recovery evidence, and retains the exact entry sequence, multiplicity, and grant-option structure.

### Copy extension package/control/update scripts

Rejected as a canonical-owner violation. Those artifacts are extension-owned package truth, not ConceptWeave Source Observation truth.

## Risks and follow-up

The current source repair is not accepted until the exact final head has repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and a bounded PostgreSQL 18 live differential.

The differential must demonstrate at least one same-generation case in which the current function ACL is held constant while the initial privilege baseline differs, and must verify row absence, `privtype`, complete initial EXECUTE ACL resolution, immutable raw-root lineage, downstream transform-object digest propagation, source-array order, and multiplicity. It must include an injected or otherwise controlled PostgreSQL 18 catalog fixture proving a repeated ACLITEM can be observed without normalization; synthetic Rust-only data remains suitable for unit contracts but is not live-differential acceptance evidence. At least one fixture must also demonstrate that swapping the same ACL entries changes ConceptWeave material identity and readback order.

## Primary sources

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 36.17. Packaging related objects into an extension*. https://www.postgresql.org/docs/18/extend-extensions.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: GRANT*. https://www.postgresql.org/docs/18/sql-grant.html

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 source: ACL data structures (`src/include/utils/acl.h`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/utils/acl.h

PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 source: ACL validation, update, and owner-change behavior (`src/backend/utils/adt/acl.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/adt/acl.c

PostgreSQL Global Development Group. (2026f). *PostgreSQL 18 source: pg_dump initial-privilege capture (`src/bin/pg_dump/pg_dump.c`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/bin/pg_dump/pg_dump.c

PostgreSQL Global Development Group. (2026g). *PostgreSQL 18 documentation: pg_dump*. https://www.postgresql.org/docs/18/app-pgdump.html
