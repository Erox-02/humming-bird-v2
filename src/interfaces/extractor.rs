use crate::schemas::{Entity, EntityType};
use std::collections::HashSet;

pub trait EntityExtractor: Send + Sync {
    fn name(&self) -> &str;
    fn supported_types(&self) -> Vec<EntityType>;
    fn extract(&self, text: &str) -> Vec<Entity>;
    fn confidence(&self, entity: &Entity) -> f32 {
        entity.confidence
    }
}