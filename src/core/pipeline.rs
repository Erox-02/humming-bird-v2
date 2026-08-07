use crate::extractors::ExtractorManager;
use crate::placeholders::{PlaceholderGenerator, PlaceholderValidator, PlaceholderRestorer};
use crate::policy_engine::{ContextBuilder};
use crate::schemas::{Entity, PrivacyDecision, ProcessResult};
use std::collections::HashMap;
use log;

#[cfg(feature = "python-bridge")]
use crate::python_bridge::MLPredictor;

#[cfg(not(feature = "python-bridge"))]
pub struct MLPredictor {
    // Fallback when Python bridge is disabled
    threshold: f32,
}

#[cfg(not(feature = "python-bridge"))]
impl MLPredictor {
    pub fn new() -> Self {
        Self { threshold: 0.5 }
    }
    
    pub fn predict(&self, _context: &str) -> (bool, f32) {
        // Fallback: always mask with 0.5 confidence
        (true, 0.5)
    }
}

pub struct Pipeline {
    extractor_manager: ExtractorManager,
    generator: PlaceholderGenerator,
    validator: PlaceholderValidator,
    restorer: PlaceholderRestorer,
    context_builder: ContextBuilder,
    predictor: Option<MLPredictor>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            extractor_manager: ExtractorManager::new(),
            generator: PlaceholderGenerator::new(),
            validator: PlaceholderValidator::new(),
            restorer: PlaceholderRestorer::new(),
            context_builder: ContextBuilder::new(),
            predictor: None,
        }
    }
    
    #[cfg(feature = "python-bridge")]
    pub fn with_predictor(mut self, predictor: MLPredictor) -> Self {
        self.predictor = Some(predictor);
        self
    }
    
    #[cfg(not(feature = "python-bridge"))]
    pub fn with_predictor(mut self, _predictor: MLPredictor) -> Self {
        self.predictor = Some(MLPredictor::new());
        self
    }
    
    pub fn process(&mut self, text: &str, intent: Option<&str>) -> ProcessResult {
        if text.trim().is_empty() {
            return ProcessResult::new(text, text);
        }
        
        self.generator.reset();
        self.validator.reset();
        self.restorer.reset();
        
        log::info!("Processing text (length: {} chars)", text.len());
        
        // Extract entities
        let entities = self.extractor_manager.extract_all(text);
        log::info!("Extracted {} entities", entities.len());
        
        if entities.is_empty() {
            return ProcessResult::new(text, text);
        }
        
        // Predict decisions
        let decisions = self.predict_decisions(&entities, text, intent);
        log::info!("Predicted {} decisions", decisions.len());
        
        // Apply masking
        let (masked_text, metadata) = self.apply_masking(text, &entities, &decisions);
        log::info!("Masked {} entities", metadata.len());
        
        self.validator.update_allowed(metadata.keys().cloned().collect());
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
    
    fn predict_decisions(
        &self,
        entities: &[Entity],
        original_text: &str,
        intent: Option<&str>,
    ) -> Vec<PrivacyDecision> {
        if let Some(predictor) = &self.predictor {
            predictor.predict_batch(entities, original_text, intent)
        } else {
            // Fallback: mask everything
            entities.iter()
                .map(|e| {
                    let context = self.context_builder.build(e, original_text, intent);
                    PrivacyDecision::new(
                        e.clone(),
                        crate::schemas::DecisionType::Mask,
                        0.5,
                        context,
                    )
                })
                .collect()
        }
    }
    
    fn apply_masking(
        &self,
        text: &str,
        entities: &[Entity],
        decisions: &[PrivacyDecision],
    ) -> (String, HashMap<String, String>) {
        use std::collections::HashMap;
        
        // Build decision map
        let decision_map: HashMap<&Entity, &PrivacyDecision> = decisions
            .iter()
            .map(|d| (&d.entity, d))
            .collect();
        
        // Sort entities by start position descending for in-place replacement
        let mut sorted_entities: Vec<&Entity> = entities.iter().collect();
        sorted_entities.sort_by(|a, b| b.start.cmp(&a.start));
        
        let mut masked = text.to_string();
        let mut metadata = HashMap::new();
        
        for entity in sorted_entities {
            if let Some(decision) = decision_map.get(&entity) {
                if decision.should_mask() {
                    let placeholder = self.generator.generate(entity);
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
