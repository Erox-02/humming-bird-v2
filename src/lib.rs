pub mod api;
pub mod core;
pub mod extractors;
pub mod interfaces;
pub mod placeholders;
pub mod policy_engine;
pub mod schemas;
pub mod utils;

use pyo3::prelude::*;

pub use api::{HBP100, HBP100Session};
pub use core::{Engine, Pipeline, SessionManager, MetadataVault};
pub use schemas::{Session, ProcessResult, PrivacyDecision, Entity};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn hbp100(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<api::HBP100>()?;
    m.add_class::<api::HBP100Session>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}