use crate::schemas::{Entity, EntityType};

pub trait Extractor: Send + Sync {
    fn name(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn extract(&self, text: &str) -> Vec<Entity>;
    fn confidence(&self, entity: &Entity) -> f64 {
        entity.confidence
    }
}