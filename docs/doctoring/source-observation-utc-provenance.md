# Source Observation UTC provenance

Status: active PR evidence for Source Observation.

## Decision

`PostgresSchemaSnapshot::observed_at_utc` is provenance, not a display timestamp. ConceptWeave therefore accepts only an explicit UTC form with uppercase `T`/`Z`, a four-digit Gregorian date, complete hour/minute/second fields, and optional decimal fractional seconds. Numeric or local offsets are rejected rather than silently normalized because normalization would replace the adapter-supplied evidence string with a derived representation.

The validator checks Gregorian month/day bounds and the RFC 3339 clock range. A syntactic `:60` second is accepted only at `23:59`; that preserves the RFC 3339 leap-second syntax boundary without claiming that ConceptWeave has independently verified an IERS leap-second announcement for the supplied date. Historical/operational leap-second authority remains source-clock evidence outside this value-object validator.

PostgreSQL accepts a deliberately broad family of date/time inputs and converts `timestamp with time zone` values to UTC internally, while not retaining the originally supplied zone. That flexibility is useful at the database boundary but is too permissive for an immutable evidence coordinate. The ConceptWeave domain contract therefore uses a narrower canonical wire form instead of delegating provenance identity to PostgreSQL's parser or current `TimeZone`/`DateStyle` settings.

## Executable evidence

Product run `33696875090`, job `100467545647`, checked out exact PR #6 head `2817df62d0b7b41c0b0dd1bcbd34a444b8a5a092`, passed CI-contract validation, Rust 1.98.0 setup, formatting and Clippy, then failed in `crates/conceptweave-observation/tests/observed_at_utc.rs` because the literal `time` was accepted as `observed_at_utc`. This is the authoritative RED for the repair.

Production commit `e27ffaf4a40d746781b8012e9fe71467e7e6511f` replaces nonblank-only validation with the bounded UTC parser. Follow-up edge fixtures exercise missing/lowercase zone designators, numeric offsets, malformed fractional seconds, invalid separators, Gregorian month/day/leap-year boundaries, invalid clock fields, fractional seconds, and the RFC 3339 `23:59:60` syntax path. Exact-head hosted GREEN remains required before this lane is considered complete.

## Rejected alternatives

- Accept any PostgreSQL-parsable timestamp: rejected because `DateStyle`, `TimeZone`, shorthand values, and automatic zone conversion are broader than an immutable evidence identity requires.
- Accept arbitrary numeric offsets and normalize to UTC: rejected because ConceptWeave would then manufacture a replacement provenance representation instead of preserving an exact adapter-supplied UTC coordinate.
- Add a datetime dependency to the core observation value-object crate for this slice: rejected because the bounded validation contract is small, deterministic, network-free, and does not require timezone-database behavior. Revisit only if later contracts need offset conversion, calendar arithmetic, or IANA timezone semantics.

## References

Klyne, G., & Newman, C. (2002). *Date and time on the Internet: Timestamps* (RFC 3339). Internet Engineering Task Force. https://www.rfc-editor.org/rfc/rfc3339

Postel, J., et al. [RFC Editor record]. (2024). *Date and time on the Internet: Timestamps with additional information* (RFC 9557). Internet Engineering Task Force. https://www.rfc-editor.org/rfc/rfc9557

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Date/time types*. https://www.postgresql.org/docs/18/datatype-datetime.html
