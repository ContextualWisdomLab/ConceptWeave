# Changelog

All notable changes to ConceptWeave are documented here. The complete changelog through exact `08847df6124f8834ed8b8ec33c9451a2a1474d3e` is preserved losslessly at `docs/archive/CHANGELOG-through-08847df6.md`.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves exact `pg_constraint.conenforced` in a domain-separated `IndexExclusionConstraintEnforcementSnapshot` layered over the exact EXCLUDE timing predecessor. Every governed ordinary exclusion constraint requires one explicit enforcement observation; PostgreSQL 18 source-unreachable `conenforced=false` fails closed, the raw bit remains in the successor digest and provenance receipt, temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains owned by the key-constraint family, and no issued predecessor digest domain is rewritten.
- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves exact `pg_constraint.convalidated` in a domain-separated `IndexExclusionConstraintValidationSnapshot` layered over the exact enforcement predecessor. Every governed ordinary exclusion constraint requires one explicit validation observation; the PostgreSQL 18 index-constraint creation path's contradictory `convalidated=false` state fails closed, the raw bit remains in the successor digest and provenance receipt, and no issued predecessor digest domain is rewritten.
- PostgreSQL ordinary `EXCLUDE` constraint evidence now preserves exact `pg_constraint.connoinherit` in a domain-separated `IndexExclusionConstraintNoInheritSnapshot` bound to the exact EXCLUDE identity predecessor. Root constraints require `connoinherit=true`, partition-child constraints require `false`, every predecessor constraint requires one explicit observation, and the raw bit is retained in a separate digest/provenance receipt without rewriting issued predecessor domains.
