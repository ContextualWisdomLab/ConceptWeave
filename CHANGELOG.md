# Changelog

All notable changes to ConceptWeave are documented here. The complete changelog through exact `08847df6124f8834ed8b8ec33c9451a2a1474d3e` is preserved losslessly at `docs/archive/CHANGELOG-through-08847df6.md`.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves exact `pg_constraint.conenforced` in a domain-separated `IndexExclusionConstraintEnforcementSnapshot` layered over the exact EXCLUDE timing predecessor. Every governed ordinary exclusion constraint requires one explicit enforcement observation; PostgreSQL 18 source-unreachable `conenforced=false` fails closed, the raw bit remains in the successor digest and provenance receipt, temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains owned by the key-constraint family, and no issued predecessor digest domain is rewritten.
