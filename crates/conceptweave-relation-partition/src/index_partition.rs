//! Immutable PostgreSQL index-partition topology and versioned definition-equivalence evidence.

#[path = "index_partition_base.rs"]
mod base;
pub use base::*;

#[path = "collation_identity.rs"]
mod collation_identity;
pub use collation_identity::*;

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

#[path = "expression_var.rs"]
mod expression_var;
pub use expression_var::*;

#[path = "expression_relation_var_schema.rs"]
mod expression_relation_var_schema;
pub use expression_relation_var_schema::*;

#[path = "type_modifier.rs"]
mod type_modifier;
pub use type_modifier::*;
