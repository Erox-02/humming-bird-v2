import re
from typing import List

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class AddressExtractor(BaseExtractor):
    def _compile_patterns(self):
        self._patterns = [
            re.compile(
                r'\b(?:Address|Mailing Address|Home Address)[:\s]+([^.\n]{10,100}?)(?=\s+(?:and|phone|email|policy|ssn|mrn|passport|[A-Z]{2,}\d)|\.|\n|$)',
                re.IGNORECASE
            ),
            re.compile(
                r'\b(\d{1,5}\s+[A-Za-z]+\s+(?:Street|St|Avenue|Ave|Road|Rd|Boulevard|Blvd|Lane|Ln|Drive|Dr|Way|Place|Pl|Court|Ct)[,\s]+[A-Za-z]+[\s,]+[A-Z]{2}\s+\d{5}(?:-\d{4})?)(?=\s+(?:and|phone|email|policy|ssn|mrn|passport|[A-Z]{2,}\d)|\n|\.\s|\.$)',
                re.IGNORECASE
            ),
            re.compile(
                r'\b(\d{1,5}\s+[A-Za-z]+\s+(?:Street|St|Avenue|Ave|Road|Rd|Boulevard|Blvd|Lane|Ln|Drive|Dr|Way|Place|Pl|Court|Ct)[,\s]+[A-Za-z]+[\s,]+[A-Z]{2}\s+\d{5}(?:-\d{4})?)\b',
                re.IGNORECASE
            ),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        return [EntityType.ADDRESS]

    def extract(self, text: str) -> List[Entity]:
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

                value = re.sub(r'[.,]\s*$', '', value)

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
