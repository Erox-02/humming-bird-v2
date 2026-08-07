use crate::schemas::{Entity, PrivacyDecision};
use std::collections::HashMap;

pub trait PrivacyPredictor: Send + Sync {
    fn predict(
        &self,
        entity: &Entity,
        original_text: &str,
        intent: Option<&str>,
    ) -> PrivacyDecision;
    
    fn predict_batch(
        &self,
        entities: &[Entity],
        original_text: &str,
        intent: Option<&str>,
    ) -> Vec<PrivacyDecision>;
    
    fn load_assets(&mut self) -> bool;
    
    fn is_loaded(&self) -> bool;
    
    fn get_metadata(&self) -> HashMap<String, String>;
}
