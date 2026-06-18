import re
from typing import List

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class AddressExtractor(BaseExtractor):
    """
    Extracts addresses from labeled sections.

    Supports:
    - Address: 123 Main St, Boston, MA 02115
    - Mailing Address: 456 Oak Ave, New York, NY 10001
    - 123 Main St, Boston, MA 02115 (direct detection)
    """

    def _compile_patterns(self):
        """Compile regex patterns for address extraction."""
        self._patterns = [
            re.compile(
                r'\b(?:Address|Mailing Address|Home Address)[:\s]+([A-Za-z0-9\s,.#-]{10,100})\b',
                re.IGNORECASE
            ),
            re.compile(
                r'\b(\d{1,5}\s+[A-Za-z]+\s+(?:Street|St|Avenue|Ave|Road|Rd|Boulevard|Blvd|Lane|Ln|Drive|Dr|Way|Place|Pl|Court|Ct)[,\s]+[A-Za-z]+[\s,]+[A-Z]{2}\s+\d{5}(?:-\d{4})?)\b',
                re.IGNORECASE
            ),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        """List of entity types this extractor supports."""
        return [EntityType.ADDRESS]

    def extract(self, text: str) -> List[Entity]:
        """Extract address entities from text."""
        self._validate_text(text)

        entities = []
        detected_values = set()

        for pattern in self._patterns:
            for match in pattern.finditer(text):
                try:
                    value = match.group(1).strip()
                except IndexError:
                    continue

                if value in detected_values or len(value) < 10:
                    continue

                start, end = match.span(1)
                entity = Entity(
                    type=EntityType.ADDRESS,
                    value=value,
                    start=start,
                    end=end,
                    confidence=0.80,
                )
                entities.append(entity)
                detected_values.add(value)

        return entities