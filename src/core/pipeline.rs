use crate::extractors::ExtractorManager;
use crate::placeholders::{
    PlaceholderGenerator,
    PlaceholderValidator,
    PlaceholderRestorer,
};
use crate::policy_engine::PrivacyPredictor;
use crate::schemas::{Entity, PrivacyDecision, ProcessResult};
use log;
use std::collections::HashMap;
use std::collections::HashSet;
pub type PipelineResult = ProcessResult;
pub struct Pipeline {
    extractor_manager: ExtractorManager,
    generator: PlaceholderGenerator,
    validator: PlaceholderValidator,
    restorer: PlaceholderRestorer,
    predictor: PrivacyPredictor,
}
impl Pipeline {
    pub fn new() -> Self {
        Self {
            extractor_manager: ExtractorManager::new(),
            generator: PlaceholderGenerator::new(),
            validator: PlaceholderValidator::new(),
            restorer: PlaceholderRestorer::new(),
            predictor: PrivacyPredictor::new(),
        }
    }
    pub fn process(&mut self, text: &str, intent: Option<&str>) -> ProcessResult {
        if text.trim().is_empty() {
            return ProcessResult::new(text, text);
        }   
        self.generator.reset();
        self.validator.reset();
        self.restorer.reset();
        log::info!("Processing text (length: {} chars)", text.len());
        let entities = self.extractor_manager.extract_all(text);
        log::info!("Extracted {} entities", entities.len());
        if entities.is_empty() {
            return ProcessResult::new(text, text);
        }
        let decisions=self.predictor.predict_batch(
            &entities,
            text,
            intent,
        );
        log::info!("Predicted {} decisions", decisions.len());
        let (masked_text, metadata) = self.apply_masking(text, &entities, &decisions);
        log::info!("Masked {} entities", metadata.len());
        let allowed:HashSet<String>=metadata.keys().cloned().collect();
        self.validator.update_allowed(allowed);
        self.restorer.update_metadata(metadata.clone());
        let has_pii = decisions.iter().any(|d| d.should_mask());
        ProcessResult {
            original_text: text.to_string(),
            masked_text,
            metadata,
            entities,
            decisions,
            has_pii,
        }
    }
    fn apply_masking(
        &self,
        text: &str,
        entities: &[Entity],
        decisions: &[PrivacyDecision],
    ) -> (String, HashMap<String, String>) {
        let decision_map: HashMap<&Entity, &PrivacyDecision> = decisions
            .iter()
            .map(|d| (&d.entity, d))
            .collect();
        let mut sorted_entities: Vec<&Entity> = entities.iter().collect();
        sorted_entities.sort_by(|a, b| b.start.cmp(&a.start));
        
        let mut masked = text.to_string();
        let mut metadata = HashMap::new();
        
        let mut generator = PlaceholderGenerator::new();
        
        for entity in sorted_entities {
            if let Some(decision) = decision_map.get(&entity) {
                if decision.should_mask() {
                    let placeholder = generator.generate(entity);
                    let start = entity.start;
                    let end = entity.end;
                    masked.replace_range(start..end, &placeholder);
                    metadata.insert(placeholder, entity.value.clone());
                }
            }
        }
        (masked, metadata)
    }
    pub fn restore_placeholders(&self, text: &str) -> String {
        self.restorer.restore(text)
    }
    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        self.restorer.restore_with_metadata(text, metadata)
    }
    pub fn validate_response(&self, response: &str) -> (bool, Option<String>) {
        self.validator.validate(response)
    }
    pub fn reset(&mut self) {
        self.generator.reset();
        self.validator.reset();
        self.restorer.reset();
        log::info!("Pipeline reset");
    }
}
impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
