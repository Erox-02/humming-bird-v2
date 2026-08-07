use serde::{Deserialize, Serialize};
use super::entity::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    Keep,
    Mask,
}

impl DecisionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DecisionType::Keep => "KEEP",
            DecisionType::Mask => "MASK",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyDecision {
    pub entity: Entity,
    pub decision: DecisionType,
    pub confidence: f32,
    pub context_string: String,
    pub reasoning: Option<String>,
}

impl PrivacyDecision {
    pub fn new(
        entity: Entity,
        decision: DecisionType,
        confidence: f32,
        context_string: impl Into<String>,
    ) -> Self {
        Self {
            entity,
            decision,
            confidence,
            context_string: context_string.into(),
            reasoning: None,
        }
    }

    pub fn should_mask(&self) -> bool {
        self.decision == DecisionType::Mask
    }

    pub fn should_keep(&self) -> bool {
        self.decision == DecisionType::Keep
    }

    pub fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "entity_type": self.entity.entity_type.as_str(),
            "entity_value": self.entity.value,
            "decision": self.decision.as_str(),
            "confidence": self.confidence,
            "should_mask": self.should_mask(),
        })
    }
}
