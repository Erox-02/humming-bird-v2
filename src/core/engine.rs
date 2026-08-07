use crate::core::{Pipeline, MetadataVault};
use crate::schemas::ProcessResult;
use std::collections::HashMap;
use log;
pub type EngineResult = ProcessResult;

#[cfg(feature = "python-bridge")]
use crate::python_bridge::MLPredictor;

pub struct Engine {
    pipeline: Pipeline,
    metadata_vault: MetadataVault,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
            metadata_vault: MetadataVault::new(),
        }
    }
    
    #[cfg(feature = "python-bridge")]
    pub fn with_predictor(mut self, predictor: MLPredictor) -> Self {
        self.pipeline = self.pipeline.with_predictor(predictor);
        self
    }
    
    pub fn process(&mut self, text: &str, intent: Option<&str>) -> ProcessResult {
        let result = self.pipeline.process(text, intent);
        self.metadata_vault.update(result.metadata.clone());
        result
    }
    
    pub fn restore(&self, text: &str) -> String {
        let metadata = self.metadata_vault.get_all();
        self.pipeline.restore_with_metadata(text, metadata)
    }
    
    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        self.pipeline.restore_with_metadata(text, metadata)
    }
    
    pub fn validate_response(&self, response: &str) -> (bool, Option<String>) {
        self.pipeline.validate_response(response)
    }
    
    pub fn reset(&mut self) {
        self.pipeline.reset();
        self.metadata_vault.clear();
        log::info!("Engine reset");
    }
    
    pub fn metadata(&self) -> HashMap<String, String> {
        self.metadata_vault.get_all()
    }
    
    pub fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
