import re
from typing import List, Set

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class HospitalExtractor(BaseExtractor):
    """
    Extracts hospital and facility names.
    """

    def _compile_patterns(self):
        """Compile regex patterns for hospital extraction."""
        self._patterns = [
            re.compile(r'\b(?:Hospital|Medical Center|Clinic)[:\s]+([A-Z][a-zA-Z\s]+)\b', re.IGNORECASE),
            re.compile(r'\b([A-Z][a-zA-Z\s]+(?:Hospital|Medical Center|Clinic))\b'),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        """List of entity types this extractor supports."""
        return [EntityType.HOSPITAL]

    def extract(self, text: str) -> List[Entity]:
        """Extract hospital entities from text."""
        self._validate_text(text)

        entities = []
        detected_values: Set[str] = set()

        for pattern in self._patterns:
            for match in pattern.finditer(text):
                try:
                    value = match.group(1).strip()
                except IndexError:
                    value = match.group(0).strip()

                if value in detected_values or len(value) < 5:
                    continue

                # Filter out common false positives
                if value.upper() in {"HOSPITAL", "MEDICAL CENTER", "CLINIC"}:
                    continue

                start, end = match.span()
                entity = Entity(
                    type=EntityType.HOSPITAL,
                    value=value,
                    start=start,
                    end=end,
                    confidence=0.75,
                )
                entities.append(entity)
                detected_values.add(value)

        return entities