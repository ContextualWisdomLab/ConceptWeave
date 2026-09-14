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

#[path = "expression_semantics.rs"]
mod expression_semantics;
pub use expression_semantics::*;

#[path = "expression_schema.rs"]
mod expression_schema;
pub use expression_schema::*;

#[path = "type_modifier.rs"]
mod type_modifier;
pub use type_modifier::*;
