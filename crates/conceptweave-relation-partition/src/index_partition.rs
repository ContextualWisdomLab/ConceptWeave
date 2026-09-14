//! Immutable PostgreSQL index-partition topology and versioned definition-equivalence evidence.

#[path = "index_partition_base.rs"]
mod base;
pub use base::*;

#[path = "operator_family.rs"]
mod operator_family;
pub use operator_family::*;

#[path = "exclusion.rs"]
mod exclusion;
pub use exclusion::*;
