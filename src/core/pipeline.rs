use crate::extractors::ExtractorManager;
use crate::placeholders::{
    PlaceholderGenerator,
    PlaceholderValidator,
    PlaceholderRestorer,
    SessionAwareGenerator,
};
use crate::policy_engine::PrivacyPredictor;
use crate::schemas::{Entity, PrivacyDecision, ProcessResult};
use crate::Session;
use log;
use std::collections::HashMap;
use std::collections::HashSet;

pub type PipelineResult = ProcessResult;

pub struct Pipeline {
    pub extractor_manager: ExtractorManager,
    pub generator: PlaceholderGenerator,
    pub validator: PlaceholderValidator,
    pub restorer: PlaceholderRestorer,
    pub predictor: PrivacyPredictor,
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
        log::debug!("Processing text (length: {} chars)", text.len());
        let entities = self.extractor_manager.extract_all(text);
        log::debug!("Extracted {} entities", entities.len());
        if entities.is_empty() {
            return ProcessResult::new(text, text);
        }

        let decisions = self.predictor.predict_batch(
            &entities,
            text,
            intent,
        );
        log::debug!("Predicted {} decisions", decisions.len());
        let (masked_text, metadata) = self.apply_masking(text, &entities, &decisions);
        log::debug!("Masked {} entities", metadata.len());
        let allowed: HashSet<String> = metadata.keys().cloned().collect();
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
    pub fn process_with_session(
        &mut self,
        text: &str,
        session: &mut Session,
        intent: Option<&str>,
    ) -> ProcessResult {
        if text.trim().is_empty() {
            return ProcessResult::new(text, text);
        }
        self.validator.reset();
        log::debug!("Processing text with session (length: {} chars)", text.len());
        let entities = self.extractor_manager.extract_all(text);
        log::debug!("Extracted {} entities", entities.len());
        if entities.is_empty() {
            return ProcessResult::new(text, text);
        }
        let decisions = self.predictor.predict_batch(
            &entities,
            text,
            intent,
        );
        log::debug!("Predicted {} decisions", decisions.len());
        let (masked_text, metadata) = self.apply_masking_with_session(
            text,
            &entities,
            &decisions,
            session,
        );
        log::debug!("Masked {} entities", metadata.len());
        let allowed: HashSet<String> = metadata.keys().cloned().collect();
        self.validator.update_allowed(allowed);
        let mut all_metadata = self.restorer.get_all_metadata();
        all_metadata.extend(metadata.clone());
        self.restorer.update_metadata(all_metadata);
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

    pub fn apply_masking(
        &self,
        text: &str,
        entities: &[Entity],
        decisions: &[PrivacyDecision],
    ) -> (String, HashMap<String, String>) {
        let mut decision_map = HashMap::new();
        for (i, decision) in decisions.iter().enumerate() {
            decision_map.insert(i, decision);
        }

        let mut sorted_indices: Vec<usize> = (0..entities.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            entities[b].end.cmp(&entities[a].end)
        });

        let mut masked = text.to_string();
        let mut metadata = HashMap::new();

        let mut generator = PlaceholderGenerator::new();

        for &idx in &sorted_indices {
            let entity = &entities[idx];
            if let Some(decision) = decision_map.get(&idx) {
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

    pub fn apply_masking_with_session(
        &self,
        text: &str,
        entities: &[Entity],
        decisions: &[PrivacyDecision],
        session: &mut Session,
    ) -> (String, HashMap<String, String>) {
        // Build map using index position instead of Entity reference
        let mut decision_map = HashMap::new();
        for (i, decision) in decisions.iter().enumerate() {
            decision_map.insert(i, decision);
        }

        let mut sorted_indices: Vec<usize> = (0..entities.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            entities[b].end.cmp(&entities[a].end)
        });

        let mut masked = text.to_string();
        let mut metadata = HashMap::new();

        let mut generator = SessionAwareGenerator::new(session);

        for &idx in &sorted_indices {
            let entity = &entities[idx];
            if let Some(decision) = decision_map.get(&idx) {
                if decision.should_mask() {
                    let placeholder = generator.generate(
                        entity.entity_type.as_str(),
                        &entity.value,
                    );
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
    
    pub fn metadata(&self) -> HashMap<String, String> {
        self.restorer.get_all_metadata()
    }

    pub fn add_config_extractor(&mut self, config_json: &str) -> Result<(), String> {
        self.extractor_manager.add_config_from_json(config_json)
    }

    pub fn add_config_extractor_from_file(&mut self, path: &str) -> Result<(), String> {
        self.extractor_manager.add_config_from_file(path)
    }

    pub fn enable_extractor(&mut self, name: &str) -> bool {
        self.extractor_manager.enable_extractor(name)
    }

    pub fn disable_extractor(&mut self, name: &str) -> bool {
        self.extractor_manager.disable_extractor(name)
    }

    pub fn list_extractors(&self) -> Vec<String> {
        self.extractor_manager.list_extractors()
    }

    pub fn list_enabled_extractors(&self) -> Vec<String> {
        self.extractor_manager.list_enabled()
    }

    pub fn reset_extractors(&mut self) {
        self.extractor_manager.reset_to_defaults();
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}