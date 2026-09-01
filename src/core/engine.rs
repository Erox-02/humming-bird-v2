use crate::core::Pipeline;
use crate::schemas::ProcessResult;
use crate::Session;
use std::collections::HashMap;
pub struct Engine {
    pipeline: Pipeline,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
        }
    }

    pub fn process(&mut self, text: &str, intent: Option<&str>) -> ProcessResult {
        self.pipeline.process(text, intent)
    }

    pub fn process_with_session(
        &mut self,
        text: &str,
        session: &mut Session,
        intent: Option<&str>,
    ) -> ProcessResult {
        self.pipeline.process_with_session(text, session, intent)
    }

    pub fn restore(&self, text: &str) -> String {
        self.pipeline.restore_placeholders(text)
    }

    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        self.pipeline.restore_with_metadata(text, metadata)
    }

    pub fn validate_response(&self, response: &str) -> (bool, Option<String>) {
        self.pipeline.validate_response(response)
    }

    pub fn reset(&mut self) {
        self.pipeline.reset();
    }

    pub fn metadata(&self) -> HashMap<String, String> {
        self.pipeline.metadata()
    }

    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    pub fn add_config_extractor(&mut self, config_json: &str) -> Result<(), String> {
        self.pipeline.add_config_extractor(config_json)
    }

    pub fn add_config_extractor_from_file(&mut self, path: &str) -> Result<(), String> {
        self.pipeline.add_config_extractor_from_file(path)
    }

    pub fn enable_extractor(&mut self, name: &str) -> bool {
        self.pipeline.enable_extractor(name)
    }

    pub fn disable_extractor(&mut self, name: &str) -> bool {
        self.pipeline.disable_extractor(name)
    }

    pub fn list_extractors(&self) -> Vec<String> {
        self.pipeline.list_extractors()
    }

    pub fn list_enabled_extractors(&self) -> Vec<String> {
        self.pipeline.list_enabled_extractors()
    }

    pub fn reset_extractors(&mut self) {
        self.pipeline.reset_extractors();
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}