# Immutable release record v1

The local publication store issues one release ID at most once in a caller-provisioned Unix directory with no group/other access. New staging files start with owner-only permissions. The final filename is the lowercase SHA-256 hex digest of the UTF-8 release ID followed by `.release`; filenames never contain caller-supplied path components.

A record contains an unsigned 64-bit big-endian header length, the header bytes, then exact governed artifact bytes. The header uses `text(x)`: unsigned 64-bit big-endian UTF-8 byte length followed by exact bytes, and `count(n)`: unsigned 64-bit big-endian count. Fields are:

1. `text("conceptweave.immutable_release_record.v1")`.
2. Release ID, contract version, ontology version, artifact digest, and complete release-manifest digest as text.
3. Truth-state byte `3` (Authoritative) and publication-state byte `4` (Published).
4. Provenance count, then source ID, source digest, and location as text for each release provenance reference sorted by the exact `(source_id, source_digest, location)` UTF-8 tuple. Duplicate references remain separate.
5. Concept-ID count, then each concept ID as text sorted by exact UTF-8 bytes.

The store writes a new staging file, makes it read-only, syncs it, atomically hard-links it to the final name without replacement, removes the staging name, and syncs the directory before returning success. An existing final name fails even when the new bytes match. A directory-sync error after linking is reported as an uncertain commit; callers must inspect stored state before any recovery decision. Orphan staging files are never release records.

Readback requires a `SemanticRelease` admitted by a client configured with an independently distributed trusted manifest pin. The store compares every header byte with that release and verifies the artifact digest before returning artifact bytes. The record's own manifest digest never grants client trust. This local implementation does not provide cross-host replication, protected pin distribution, steward authentication, or a production deployment/backup policy.
