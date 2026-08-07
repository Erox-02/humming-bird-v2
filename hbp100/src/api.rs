use crate::core::{Engine, EngineResult};
use crate::schemas::ProcessResult;
use std::collections::HashMap;

#[cfg(feature = "python-bridge")]
use crate::python_bridge::MLPredictor;

pub struct HBP100 {
    engine: Engine,
}

impl HBP100 {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }
    
    #[cfg(feature = "python-bridge")]
    pub fn with_predictor(mut self) -> Self {
        let predictor = MLPredictor::new();
        self.engine = self.engine.with_predictor(predictor);
        self
    }
    
    pub fn process(&mut self, text: &str, intent: Option<&str>) -> ProcessResult {
        self.engine.process(text, intent)
    }
    
    pub fn restore(&self, text: &str) -> String {
        self.engine.restore(text)
    }
    
    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        self.engine.restore_with_metadata(text, metadata)
    }
    
    pub fn validate_response(&self, response: &str) -> (bool, Option<String>) {
        self.engine.validate_response(response)
    }
    
    pub fn reset(&mut self) {
        self.engine.reset();
    }
    
    pub fn metadata(&self) -> HashMap<String, String> {
        self.engine.metadata()
    }
    
    pub fn version(&self) -> &'static str {
        self.engine.version()
    }
}

impl Default for HBP100 {
    fn default() -> Self {
        Self::new()
    }
}
