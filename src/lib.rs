pub mod api;
pub mod core;
pub mod extractors;
pub mod interfaces;
pub mod placeholders;
pub mod policy_engine;
pub mod schemas;
pub mod utils;

use pyo3::prelude::*;

pub use api::HBP100;
pub use core::{Engine, Pipeline, EngineResult, PipelineResult};
pub use extractors::*;
pub use schemas::*;
pub use interfaces::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn hbp100(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<api::HBP100>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}