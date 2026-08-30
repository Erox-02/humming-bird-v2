use crate::schemas::{Entity, EntityType, PrivacyDecision, DecisionType};

pub struct PrivacyPredictor {
    model: Option<Model>,
}

impl PrivacyPredictor {
    pub fn new() -> Self {
        Self {
            model: None,
        }
    }
    pub fn predict_batch(
        &self,
        entities: &[Entity],
        text: &str,
        intent: Option<&str>,
    ) -> Vec<PrivacyDecision> {
        entities
            .iter()
            .map(|entity| self.predict(entity, text, intent))
            .collect()
    }
    pub fn predict(
        &self,
        entity: &Entity,
        _text: &str,
        _intent: Option<&str>,
    ) -> PrivacyDecision {
        let should_mask = self.should_mask_by_type(&entity.entity_type);   
        PrivacyDecision {
            entity: entity.clone(),
            decision: if should_mask { DecisionType::Mask } else { DecisionType::Keep },
            confidence: entity.confidence,
            context_string: None,
            reasoning: None,
        }
    }

    fn should_mask_by_type(&self, entity_type: &EntityType) -> bool {
        match entity_type {
            EntityType::Name => true,
            EntityType::Email => true,
            EntityType::Phone => true,
            EntityType::Date => false,
            EntityType::Address => true,
            EntityType::Id => true,
            EntityType::Medical => true,
        }
    }
}

impl Default for PrivacyPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Model;