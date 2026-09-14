//! Immutable PostgreSQL index-partition topology and versioned definition-equivalence evidence.

#[path = "index_partition_base.rs"]
mod base;
pub use base::*;

mod operator_family;
pub use operator_family::*;
