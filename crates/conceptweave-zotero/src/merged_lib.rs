#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Read-only Zotero research classification plus evidence-bound golden-set evaluation.
//!
//! Research Intake remains the owner of the trusted classification aggregate. Golden-set
//! evaluation is layered on its public, read-only contract and therefore cannot reopen or
//! remint `ClassificationReport` internals.

#[path = "lib.rs"]
mod research_intake;
pub use research_intake::*;

mod golden_set;
pub use golden_set::*;
