import re
from typing import List, Set

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class NameExtractor(BaseExtractor):
    NAME_KEYWORDS = {'patient', 'name', 'full name', "patient's", 'dr', 'mr', 'mrs', 'ms'}

    PHYSICIAN_KEYWORDS = {
        'attending physician', 'referring physician',
        'consulting physician', 'resident', 'pcp',
        'primary care physician', 'consultant'
    }

    TITLE_WORDS = {"PATIENT", "NAME", "FULL", "DR", "MR", "MRS", "MS"}

    MEDICAL_WORDS = {
        "MRI", "XRAY", "XRAYS", "REPORT", "SCAN", "CHEST", "BLOOD",
        "TEST", "LAB", "RESULT", "CT", "ULTRASOUND", "SONOGRAM",
        "BIOPSY", "CULTURE", "PATHOLOGY", "HISTOLOGY", "CYTOLOGY",
        "EMERGENCY", "ADMISSION", "DISCHARGE", "DIAGNOSIS",
        "HISTORY", "PHYSICAL", "MEDICATION", "PRESCRIPTION",
        "DOSAGE", "FREQUENCY", "ROUTE", "INDICATION", "CONTRAINDICATION"
    }

    def __init__(self):
        super().__init__()

    def _compile_patterns(self):
        self._patient_patterns = [
            # Labeled patterns
            re.compile(r'\b(?:Patient Name|Patient\'s Name|Full Name)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:Patient Name|Patient\'s Name|Full Name)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Simple patient name pattern - catches "patient Sarah Johnson" anywhere
            re.compile(r'\bpatient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\bpatient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Contact patient pattern
            re.compile(r'\bContact\s+patient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\bContact\s+patient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Name label pattern
            re.compile(r'\bName[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\bName[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Discharge note pattern
            re.compile(r'\bdischarge note for patient\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\bdischarge note for patient\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
        ]

        self._physician_patterns = [
            # Attending/Referring/Consulting Physician
            re.compile(r'\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Resident, PCP, Primary Care Physician
            re.compile(r'\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Consultant
            re.compile(r'\bConsultant[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\bConsultant[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
            
            # Physician/Doctor label
            re.compile(r'\b(?:Physician|Doctor)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:Physician|Doctor)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
        ]

        self._title_patterns = [
            # Dr., Mr., Mrs., Ms. with optional period
            re.compile(r'\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
        ]

        self._called_patterns = [
            # called/named patterns
            re.compile(r'\b(?:called|named)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b', re.IGNORECASE),
            re.compile(r'\b(?:called|named)\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b', re.IGNORECASE),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        return [EntityType.NAME, EntityType.PHYSICIAN]

    def extract(self, text: str) -> List[Entity]:
        self._validate_text(text)

        entities = []
        detected_values: Set[str] = set()

        # Extract patient names
        patient_entities = self._extract_group_matches(
            text, self._patient_patterns, EntityType.NAME, group=1, confidence=0.80
        )
        for entity in patient_entities:
            if self._is_patient_name(entity, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract physician names
        physician_entities = self._extract_group_matches(
            text, self._physician_patterns, EntityType.PHYSICIAN, group=1, confidence=0.85
        )
        for entity in physician_entities:
            if self._is_physician_name(entity, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract title-based names
        title_entities = self._extract_group_matches(
            text, self._title_patterns, EntityType.NAME, group=1, confidence=0.80
        )
        for entity in title_entities:
            if self._is_patient_name(entity, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract called/named patterns
        called_entities = self._extract_group_matches(
            text, self._called_patterns, EntityType.NAME, group=1, confidence=0.75
        )
        for entity in called_entities:
            if self._is_patient_name(entity, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        return entities

    def _is_patient_name(self, entity: Entity, full_context: str) -> bool:
        return self._is_person_name(entity, full_context, self.NAME_KEYWORDS)

    def _is_physician_name(self, entity: Entity, full_context: str) -> bool:
        return self._is_person_name(entity, full_context, self.PHYSICIAN_KEYWORDS)

    def _is_person_name(self, entity: Entity, full_context: str, keywords: set) -> bool:
        word = entity.value
        if len(word) < 2 or len(word) > 50:
            return False

        if re.search(r'\d', word):
            return False

        if not (self._is_valid_name_format(word)):
            return False

        parts = word.strip().split()
        for part in parts:
            upper_part = part.upper()
            if upper_part in self.TITLE_WORDS:
                return False
            if upper_part in self.MEDICAL_WORDS:
                return False
        start = max(0, entity.start - 200)
        end = min(len(full_context), entity.end + 200)
        window = full_context[start:end].lower()

        if not any(kw in window for kw in keywords):
            return False

        return 1 <= len(parts) <= 4 
