use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct NameExtractor {
    patient_patterns: Vec<Regex>,
    physician_patterns: Vec<Regex>,
    title_patterns: Vec<Regex>,
    called_patterns: Vec<Regex>,
    generic_patterns: Vec<Regex>,
}

impl NameExtractor {
    const NAME_WORD: &'static str = concat!(
        r"(?:",
        r"\p{Lu}[\p{Ll}]+(?:[-'][\p{Lu}][\p{Ll}]+)*",
        r"|\p{Lu}\.",
        r")"
    );

    const NAME: &'static str = concat!(
        r"(",
        r"(?:\p{Lu}[\p{Ll}]+(?:[-'][\p{Lu}][\p{Ll}]+)*|\p{Lu}\.)",
        r"(?:\s+",
        r"(?:\p{Lu}[\p{Ll}]+(?:[-'][\p{Lu}][\p{Ll}]+)*|\p{Lu}\.)",
        r"){0,3}",
        r")"
    );

    const STOP_WORDS: &'static [&'static str] = &[
        "THE", "AND", "OR", "OF", "FOR", "WITH", "FROM", "THIS", "THAT",
        "THESE", "THOSE", "WAS", "WERE", "IS", "ARE", "AM", "BE", "BEEN",
        "BEING", "HAS", "HAD", "HAVE", "DO", "DID", "DOES", "PATIENT",
        "NAME", "FULL", "PHYSICIAN", "DOCTOR", "ATTENDING", "REFERRING",
        "CONSULTING", "PRIMARY", "CARE", "RESIDENT", "CONSULTANT", "REPORT",
        "HISTORY", "DIAGNOSIS", "MEDICATION", "PRESCRIPTION", "ADMISSION",
        "DISCHARGE", "EMERGENCY", "FOLLOW", "UP", "HOSPITAL", "CLINIC",
        "MEDICAL", "HEALTH", "CENTER", "DEPARTMENT", "LABORATORY", "RESULT",
        "TEST", "SCAN", "MRI", "CT", "XRAY", "ULTRASOUND",
    ];

    pub fn new() -> Self {
        let mut extractor = Self {
            patient_patterns: Vec::new(),
            physician_patterns: Vec::new(),
            title_patterns: Vec::new(),
            called_patterns: Vec::new(),
            generic_patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }

    fn compile_patterns(&mut self) {
        self.patient_patterns = vec![
            Regex::new(&format!(
                r"(?i)\b(?:patient(?:'s)?\s+name|full\s+name)\s*[:\-]?\s*{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\bpatient\s*[:\-]?\s*{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\bcontact\s+patient\s+{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\bdischarge\s+note\s+for\s+patient\s+{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\bname\s*[:\-]\s*{}\b",
                Self::NAME
            )).unwrap(),
        ];

        self.physician_patterns = vec![
            Regex::new(&format!(
                r"(?i)\b(?:attending|referring|consulting)\s+physician\s*[:\-]?\s*{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\bprimary\s+care\s+physician\s*[:\-]?\s*{}\b",
                Self::NAME
            )).unwrap(),
            Regex::new(&format!(
                r"(?i)\b(?:physician|doctor|consultant|resident|pcp)\s*[:\-]?\s*{}\b",
                Self::NAME
            )).unwrap(),
        ];

        self.title_patterns = vec![
            Regex::new(&format!(
                r"(?i)\b(?:Dr|Mr|Mrs|Ms|Prof|Professor|Sir)\.?\s+{}\b",
                Self::NAME
            )).unwrap(),
        ];

        self.called_patterns = vec![
            Regex::new(&format!(
                r"(?i)\b(?:called|named)\s+{}\b",
                Self::NAME
            )).unwrap(),
        ];

        self.generic_patterns = vec![
            Regex::new(&format!(r"\b{}\b", Self::NAME)).unwrap(),
        ];
    }

    fn is_person_name(&self, value: &str) -> bool {
        let value = value.trim();
        if value.len() < 2 || value.len() > 80 {
            return false;
        }
        if value.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.is_empty() || parts.len() > 4 {
            return false;
        }
        for part in &parts {
            let normalized = part
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_uppercase();
            if Self::STOP_WORDS.contains(&normalized.as_str()) {
                return false;
            }
        }
        true
    }

    fn add_matches(
        &self,
        regex: &Regex,
        text: &str,
        confidence: f32,
        entities: &mut Vec<Entity>,
        detected: &mut HashSet<String>,
    ) {
        for caps in regex.captures_iter(text) {
            let Some(matched) = caps.get(1) else {
                continue;
            };
            let value = matched.as_str().trim();
            if !self.is_person_name(value) {
                continue;
            }
            let key = value.to_lowercase();
            if !detected.insert(key) {
                continue;
            }
            entities.push(Entity {
                entity_type: EntityType::Name,
                value: value.to_string(),
                start: matched.start(),
                end: matched.end(),
                confidence,
                placeholder: None,
                metadata: HashMap::new(),
            });
        }
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
        vec![EntityType::Name]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();

        for pattern in &self.patient_patterns {
            self.add_matches(pattern, text, 0.95, &mut entities, &mut detected);
        }
        for pattern in &self.physician_patterns {
            self.add_matches(pattern, text, 0.96, &mut entities, &mut detected);
        }
        for pattern in &self.title_patterns {
            self.add_matches(pattern, text, 0.98, &mut entities, &mut detected);
        }
        for pattern in &self.called_patterns {
            self.add_matches(pattern, text, 0.85, &mut entities, &mut detected);
        }
        for pattern in &self.generic_patterns {
            self.add_matches(pattern, text, 0.60, &mut entities, &mut detected);
        }

        entities.sort_by_key(|entity| entity.start);
        entities
    }
}