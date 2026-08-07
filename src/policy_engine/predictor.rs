use crate::schemas::{Entity, PrivacyDecision};
use crate::policy_engine::ContextBuilder;

pub struct PrivacyPredictor {
    context_builder: ContextBuilder,
    threshold: f32,
}

impl PrivacyPredictor {
    pub fn new() -> Self {
        Self {
            context_builder: ContextBuilder::new(),
            threshold: 0.5,
        }
    }
    
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }
    
    pub fn predict(&self, entity: &Entity, original_text: &str, intent: Option<&str>) -> PrivacyDecision {
        let context = self.context_builder.build(entity, original_text, intent);
        
        let should_mask = self.should_mask_fallback(entity);
        let confidence = if should_mask { 0.65 } else { 0.35 };
        
        PrivacyDecision::new(
            entity.clone(),
            if should_mask { crate::schemas::DecisionType::Mask } else { crate::schemas::DecisionType::Keep },
            confidence,
            context,
        )
    }
    
    pub fn predict_batch(&self, entities: &[Entity], original_text: &str, intent: Option<&str>) -> Vec<PrivacyDecision> {
        entities.iter()
            .map(|e| self.predict(e, original_text, intent))
            .collect()
    }
    
    fn should_mask_fallback(&self, entity: &Entity) -> bool {
        match entity.entity_type {
            crate::schemas::EntityType::SSN => true,
            crate::schemas::EntityType::Passport => true,
            crate::schemas::EntityType::PatientId => true,
            crate::schemas::EntityType::MRN => true,
            crate::schemas::EntityType::PolicyNumber => true,
            crate::schemas::EntityType::Phone => true,
            crate::schemas::EntityType::Email => true,
            crate::schemas::EntityType::Address => true,
            _ => {
                entity.confidence > 0.7
            }
        }
    }
}

impl Default for PrivacyPredictor {
    fn default() -> Self {
        Self::new()
    }
}
