//! Bounded Source Observation port contracts for ConceptWeave.
//!
//! The executable contract is introduced test-first in follow-up commits. This crate owns only
//! provider-independent access boundaries; PostgreSQL drivers and credentials remain behind an
//! adapter implementation.
#![forbid(unsafe_code)]
