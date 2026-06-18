import re
from typing import List, Set

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class NameExtractor(BaseExtractor):
    """
    Extracts person names and physician names from labeled sections.

    Supports:
    - Patient Name
    - Name (with context)
    - Dr/Mr/Mrs/Ms prefixes
    - Attending/Referring/Consulting Physician
    - Resident, PCP
    - All-caps OCR output
    """

    # Keywords that indicate name context
    NAME_KEYWORDS = {'patient', 'name', 'full name', "patient's", 'dr', 'mr', 'mrs', 'ms'}

    # Keywords that indicate physician context
    PHYSICIAN_KEYWORDS = {
        'attending physician', 'referring physician',
        'consulting physician', 'resident', 'pcp',
        'primary care physician', 'consultant'
    }

    # Words that should never be treated as names
    TITLE_WORDS = {"PATIENT", "NAME", "FULL", "DR", "MR", "MRS", "MS"}

    # Medical terms that can appear in all caps
    MEDICAL_WORDS = {
        "MRI", "XRAY", "XRAYS", "REPORT", "SCAN", "CHEST", "BLOOD",
        "TEST", "LAB", "RESULT", "CT", "ULTRASOUND", "SONOGRAM",
        "BIOPSY", "CULTURE", "PATHOLOGY", "HISTOLOGY", "CYTOLOGY",
        "EMERGENCY", "ADMISSION", "DISCHARGE", "DIAGNOSIS",
        "HISTORY", "PHYSICAL", "MEDICATION", "PRESCRIPTION",
        "DOSAGE", "FREQUENCY", "ROUTE", "INDICATION", "CONTRAINDICATION"
    }

    def __init__(self):
        """Initialize with compiled patterns."""
        super().__init__()

    def _compile_patterns(self):
        """Compile regex patterns for name extraction."""
        # Patient name patterns
        self._patient_patterns = [
            re.compile(r'\b(?:Patient Name|Patient\'s Name|Full Name)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Patient Name|Patient\'s Name|Full Name)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
            re.compile(r'\b(?:Patient)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Patient)\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
            re.compile(r'\b(?:Name)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Name)\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
        ]

        # Physician name patterns
        self._physician_patterns = [
            re.compile(r'\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Attending|Referring|Consulting)\s+Physician[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
            re.compile(r'\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Resident|PCP|Primary Care Physician)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
            re.compile(r'\b(?:Consultant)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Consultant)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
            re.compile(r'\b(?:Physician|Doctor)[:\s]+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Physician|Doctor)[:\s]+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
        ]

        # Title patterns
        self._title_patterns = [
            re.compile(r'\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:Dr|Mr|Mrs|Ms)\.?\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
        ]

        # Called/named patterns
        self._called_patterns = [
            re.compile(r'\b(?:called|named)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2})\b'),
            re.compile(r'\b(?:called|named)\s+([A-Z]+(?:\s+[A-Z]+){0,2})\b'),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        """List of entity types this extractor supports."""
        return [EntityType.NAME, EntityType.PHYSICIAN]

    def extract(self, text: str) -> List[Entity]:
        """Extract name entities from text."""
        self._validate_text(text)

        entities = []
        detected_values: Set[str] = set()

        # Extract patient names
        patient_entities = self._extract_group_matches(
            text, self._patient_patterns, EntityType.NAME, group=1, confidence=0.80
        )
        for entity in patient_entities:
            if self._is_patient_name(entity.value, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract physician names
        physician_entities = self._extract_group_matches(
            text, self._physician_patterns, EntityType.PHYSICIAN, group=1, confidence=0.85
        )
        for entity in physician_entities:
            if self._is_physician_name(entity.value, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract title-based names
        title_entities = self._extract_group_matches(
            text, self._title_patterns, EntityType.NAME, group=1, confidence=0.80
        )
        for entity in title_entities:
            if self._is_patient_name(entity.value, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        # Extract called/named patterns
        called_entities = self._extract_group_matches(
            text, self._called_patterns, EntityType.NAME, group=1, confidence=0.75
        )
        for entity in called_entities:
            if self._is_patient_name(entity.value, text):
                if entity.value not in detected_values:
                    entities.append(entity)
                    detected_values.add(entity.value)

        return entities

    def _is_patient_name(self, word: str, full_context: str) -> bool:
        """Validate if a word is a patient name."""
        return self._is_person_name(word, full_context, self.NAME_KEYWORDS)

    def _is_physician_name(self, word: str, full_context: str) -> bool:
        """Validate if a word is a physician name."""
        return self._is_person_name(word, full_context, self.PHYSICIAN_KEYWORDS)

    def _is_person_name(self, word: str, full_context: str, keywords: set) -> bool:
        """
        Validate if a word is a person name.

        Args:
            word: Word to validate
            full_context: Full text for context
            keywords: Set of context keywords

        Returns:
            True if valid name, False otherwise
        """
        if len(word) < 2 or len(word) > 50:
            return False

        # Must not contain digits
        if re.search(r'\d', word):
            return False

        # Allow Title Case or ALL CAPS
        if not (self._is_valid_name_format(word)):
            return False

        # Check each word part
        parts = word.strip().split()
        for part in parts:
            upper_part = part.upper()
            if upper_part in self.TITLE_WORDS:
                return False
            if upper_part in self.MEDICAL_WORDS:
                return False

        # Must have context keyword nearby
        if not any(kw in full_context.lower() for kw in keywords):
            return False

        return 1 <= len(parts) <= 3