//! Immutable PostgreSQL schema-observation contracts for ConceptWeave.
//!
//! The executable contract is intentionally introduced test-first. Source-system adapters remain
//! outside this crate and must provide bounded, read-only metadata to these domain-safe types.
#![forbid(unsafe_code)]
