use crate::schemas::{Entity, EntityType};

pub trait EntityExtractor: Send + Sync {
    fn extract(&self, text: &str) -> Vec<Entity>;
    
    fn supports(&self, entity_type: EntityType) -> bool;
    
    fn supported_types(&self) -> Vec<EntityType>;
    
    fn validate_text(&self, text: &str) -> Result<(), String> {
        if text.trim().is_empty() {
            return Err("Input text cannot be empty".to_string());
        }
        Ok(())
    }
}
