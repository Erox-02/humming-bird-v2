import re
from typing import List

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class PhoneExtractor(BaseExtractor):
    def _compile_patterns(self):
        self._patterns = [
            re.compile(r'\b\+\d{1,3}[-.\s]?\(?\d{1,4}\)?[-.\s]?\d{1,4}[-.\s]?\d{1,9}\b'),
            re.compile(r'\b\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}\b'),
            re.compile(r'\b\d{3}[-.\s]\d{3}[-.\s]\d{4}\b'),
            re.compile(r'\b\d{10}\b'),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        return [EntityType.PHONE]

    def extract(self, text: str) -> List[Entity]:
        self._validate_text(text)

        entities = []
        detected_digits = set()

        for pattern in self._patterns:
            for match in pattern.finditer(text):
                value = match.group(0)
                cleaned = re.sub(r'[^0-9]', '', value)

                if cleaned in detected_digits:
                    continue

                if 10 <= len(cleaned) <= 15:
                    start, end = match.span()
                    entity = Entity(
                        type=EntityType.PHONE,
                        value=value,
                        start=start,
                        end=end,
                        confidence=0.85,
                    )
                    entities.append(entity)
                    detected_digits.add(cleaned)

        return entities
