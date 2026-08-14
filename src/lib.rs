pub mod api;
pub mod core;
pub mod extractors;
pub mod interfaces;
pub mod placeholders;
pub mod policy_engine;
pub mod schemas;
pub mod utils;

pub use api::HBP100;
pub use core::{Engine, Pipeline, EngineResult, PipelineResult};
pub use extractors::{
    BaseExtractor, AddressExtractor, DateExtractor, EmailExtractor,
    PhoneExtractor, IDExtractor, NameExtractor, HospitalExtractor,
    ExtractorManager,
};
pub use schemas::{
    Entity, EntityType, PrivacyDecision, DecisionType, Placeholder, ProcessResult,
};
pub use interfaces::{EntityExtractor, PrivacyPredictor, PlaceholderEngine};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
