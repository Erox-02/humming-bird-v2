use serde::{Deserialize, Serialize};
use crate::schemas::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyDecision {
    pub entity: Entity,
    pub decision: DecisionType,
    pub confidence: f32,
    pub context_string: Option<String>,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionType {
    Mask,
    Keep,
}

impl PrivacyDecision {
    pub fn should_mask(&self) -> bool {
        self.decision == DecisionType::Mask
    }
}