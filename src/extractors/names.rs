}use crate::extractors::base::BaseExtractor;
use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::HashSet;

pub struct NameExtractor {
    patient_patterns: Vec<Regex>,
    physician_patterns: Vec<Regex>,
    title_patterns: Vec<Regex>,
    called_patterns: Vec<Regex>,
}

impl NameExtractor {
    const NAME_KEYWORDS: &'static [&'static str] = &[
        "patient", "name", "full name", "patient's", "dr", "mr", "mrs", "ms",
    ];
    const PHYSICIAN_KEYWORDS: &'static [&'static str] = &[
        "attending physician", "referring physician",
        "consulting physician", "resident", "pcp",
        "primary care physician", "consultant",
    ];
    const TITLE_WORDS: &'static [&'static str] = &[
        "PATIENT", "NAME", "FULL", "DR", "MR", "MRS", "MS",
    ];
    const MEDICAL_WORDS: &'static [&'static str] = &[
        "MRI", "XRAY", "XRAYS", "REPORT", "SCAN", "CHEST", "BLOOD",
        "TEST", "LAB", "RESULT", "CT", "ULTRASOUND", "SONOGRAM",
        "BIOPSY", "CULTURE", "PATHOLOGY", "HISTOLOGY", "CYTOLOGY",
        "EMERGENCY", "ADMISSION", "DISCHARGE", "DIAGNOSIS",
        "HISTORY", "PHYSICAL", "MEDICATION", "PRESCRIPTION",
        "DOSAGE", "FREQUENCY", "ROUTE", "INDICATION", "CONTRAINDICATION",
    ];
    
    pub fn new() -> Self {
        let mut extractor = Self {
            patient_patterns: Vec::new(),
            physician_patterns: Vec::new(),
            title_patterns: Vec::new(),
            called_patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
    
    fn is_person_name(
        &self,
        value: &str,
        text: &str,
        start: usize,
        end: usize,
        keywords: &[&str],
    ) -> bool {
        let len = value.len();
        if len < 2 || len > 50 {
            return false;
        }
        
        if value.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }
        
        if !self.is_valid_name_format(value) && !self.is_all_caps(value) {
            return false;
        }
        
        let parts: Vec<&str> = value.split_whitespace().collect();
        for part in &parts {
            let upper = part.to_uppercase();
            if Self::TITLE_WORDS.contains(&upper.as_str()) {
                return false;
            }
            if Self::MEDICAL_WORDS.contains(&upper.as_str()) {
                return false;
            }
        }
        
        if !self.has_context_keyword(text, start, end, keywords) {
            return false;
        }
        
        parts.len() <= 4
    }
    
    fn extract_names(
        &self,
        text: &str,
        patterns: &[Regex],
        entity_type: EntityType,
        keywords: &[&str],
        confidence: f32,
    ) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let value = matched.as_str().trim().to_string();
                    
                    if !detected.contains(&value)
                        && self.is_person_name(&value, text, matched.start(), matched.end(), keywords)
                    {
                        detected.insert(value.clone());
                        entities.push(Entity::new(
                            entity_type,
                            value,
                            matched.start(),
                            matched.end(),
                            confidence,
                        ));
                    }
                }
            }
        }
        
        entities
    }
}

impl Default for NameExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityExtractor for NameExtractor {
    fn extract(&self, text: &str) -> Vec<Entity> {
        if let Err(e) = self.validate_text(text) {
            log::warn!("Validation failed: {}", e);
            return Vec::new();
        }
        
        let mut all_entities = Vec::new();
        let mut detected = HashSet::new();
        
        let patient_names = self.extract_names(
            text,
            &self.patient_patterns,
            EntityType::Name,
            Self::NAME_KEYWORDS,
            0.80,
        );
        for entity in patient_names {
            if !detected.contains(&entity.value) {
                detected.insert(entity.value.clone());
                all_entities.push(entity);
            }
        }
        
        let physician_names = self.extract_names(
            text,
            &self.physician_patterns,
            EntityType::Physician,
            Self::PHYSICIAN_KEYWORDS,
            0.85,
        );
        for entity in physician_names {
            if !detected.contains(&entity.value) {
                detected.insert(entity.value.clone());
                all_entities.push(entity);
            }
        }
        
        let title_names = self.extract_names(
            text,
            &self.title_patterns,
            EntityType::Name,
            Self::NAME_KEYWORDS,
            0.80,
        );
        for entity in title_names {
            if !detected.contains(&entity.value) {
                detected.insert(entity.value.clone());
                all_entities.push(entity);
            }
        }
        
        let called_names = self.extract_names(
            text,
            &self.called_patterns,
            EntityType::Name,
            Self::NAME_KEYWORDS,
            0.75,
        );
        for entity in called_names {
            if !detected.contains(&entity.value) {
                detected.insert(entity.value.clone());
                all_entities.push(entity);
            }
        }
        
        all_entities
    }
    
    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Name, EntityType::Physician]
    }
}

impl BaseExtractor for NameExtractor {
    fn compile_patterns(&mut self) {
        self.patient_patterns = vec![
            Regex::new(r"(?i)\b(?:Patient Name|Patient's Name|Full Name)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Patient Name|Patient's Name|Full Name)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bpatient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bpatient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bContact\s+patient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bContact\s+patient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bName[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bName[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bdischarge note for patient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bdischarge note for patient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
        ];
        
        self.physician_patterns = vec![
            Regex::new(r"(?i)\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bConsultant[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\bConsultant[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Physician|Doctor)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Physician|Doctor)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
        ];
        
        self.title_patterns = vec![
            Regex::new(r"(?i)\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
        ];
        
        self.called_patterns = vec![
            Regex::new(r"(?i)\b(?:called|named)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b").unwrap(),
            Regex::new(r"(?i)\b(?:called|named)\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b").unwrap(),
        ];
    }
}
