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
    
    fn is_person_name(&self, value: &str) -> bool {
        let len = value.len();
        if len < 2 || len > 50 {
            return false;
        }
        if value.chars().any(|c| c.is_ascii_digit()) {
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
        parts.len() <= 4
    }
}

impl Default for NameExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityExtractor for NameExtractor {
    fn name(&self) -> &str {
        "NameExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::NAME]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in &self.patient_patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let value = matched.as_str().trim().to_string();
                    if !detected.contains(&value) && self.is_person_name(&value) {
                        detected.insert(value.clone());
                        entities.push(Entity {
                            entity_type: EntityType::NAME,
                            value,
                            start: matched.start(),
                            end: matched.end(),
                            confidence: 0.80,
                        });
                    }
                }
            }
        }
        
        for pattern in &self.physician_patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let value = matched.as_str().trim().to_string();
                    if !detected.contains(&value) && self.is_person_name(&value) {
                        detected.insert(value.clone());
                        entities.push(Entity {
                            entity_type: EntityType::NAME,
                            value,
                            start: matched.start(),
                            end: matched.end(),
                            confidence: 0.85,
                        });
                    }
                }
            }
        }
        
        for pattern in &self.title_patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let value = matched.as_str().trim().to_string();
                    if !detected.contains(&value) && self.is_person_name(&value) {
                        detected.insert(value.clone());
                        entities.push(Entity {
                            entity_type: EntityType::NAME,
                            value,
                            start: matched.start(),
                            end: matched.end(),
                            confidence: 0.80,
                        });
                    }
                }
            }
        }
        
        for pattern in &self.called_patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let value = matched.as_str().trim().to_string();
                    if !detected.contains(&value) && self.is_person_name(&value) {
                        detected.insert(value.clone());
                        entities.push(Entity {
                            entity_type: EntityType::NAME,
                            value,
                            start: matched.start(),
                            end: matched.end(),
                            confidence: 0.75,
                        });
                    }
                }
            }
        }
        
        entities
    }
}