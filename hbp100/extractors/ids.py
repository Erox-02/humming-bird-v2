import re
from typing import List, Dict

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class IDExtractor(BaseExtractor):
    """
    Extracts labeled identifiers.

    Supports:
    - MRN: 12345
    - Patient ID: P-67890
    - Case Number: C-11111
    - Policy Number: INS-12345
    """

    # Configuration for each ID type
    ID_CONFIGS: Dict[EntityType, Dict] = {
        EntityType.MRN: {
            'labels': [r'\b(?:MRN|Medical Record Number)[:\s]+'],
            'min_length': 4,
            'pattern': r'([A-Z0-9]{4,20})\b',
            'confidence': 0.90,
        },
        EntityType.PATIENT_ID: {
            'labels': [r'\b(?:Patient ID|PID|Patient Identifier)[:\s]+'],
            'min_length': 4,
            'pattern': r'([A-Z0-9][-]?[A-Z0-9]{3,20})\b',
            'confidence': 0.90,
        },
        EntityType.CASE_ID: {
            'labels': [r'\b(?:Case Number|Case No|Case ID)[:\s]+'],
            'min_length': 4,
            'pattern': r'([A-Z0-9][-]?[A-Z0-9]{3,20})\b',
            'confidence': 0.90,
        },
        EntityType.POLICY_NUMBER: {
            'labels': [r'\b(?:Policy Number|Policy No|Policy ID|Insurance Policy)[:\s]+'],
            'min_length': 5,
            'pattern': r'([A-Z0-9][-]?[A-Z0-9]{4,20})\b',
            'confidence': 0.90,
        },
    }

    def _compile_patterns(self):
        """Compile regex patterns for ID extraction."""
        self._patterns = {}
        for entity_type, config in self.ID_CONFIGS.items():
            self._patterns[entity_type] = []
            for label in config['labels']:
                full_pattern = label + config['pattern']
                self._patterns[entity_type].append(re.compile(full_pattern, re.IGNORECASE))

    @property
    def supported_types(self) -> List[EntityType]:
        """List of entity types this extractor supports."""
        return list(self.ID_CONFIGS.keys())

    def extract(self, text: str) -> List[Entity]:
        """Extract ID entities from text."""
        self._validate_text(text)

        entities = []
        detected_values = set()

        for entity_type, patterns in self._patterns.items():
            config = self.ID_CONFIGS[entity_type]
            for pattern in patterns:
                for match in pattern.finditer(text):
                    try:
                        value = match.group(1)
                    except IndexError:
                        continue

                    if value in detected_values:
                        continue

                    if len(value) >= config['min_length']:
                        start, end = match.span(1)
                        entity = Entity(
                            type=entity_type,
                            value=value,
                            start=start,
                            end=end,
                            confidence=config['confidence'],
                        )
                        entities.append(entity)
                        detected_values.add(value)

        return entities