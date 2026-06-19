import re
from typing import List, Set

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class HospitalExtractor(BaseExtractor):
    def _compile_patterns(self):
        self._patterns = [
            re.compile(r'\b(?:Hospital|Medical Center|Clinic)[:\s]+([A-Z][a-zA-Z\s]+?)(?=\s+[A-Z]|$|[,.]|\n)', re.IGNORECASE),
            re.compile(r'\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2}\s+(?:Hospital|Medical Center|Clinic))\b'),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        return [EntityType.HOSPITAL]

    def extract(self, text: str) -> List[Entity]:
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
                if value.upper() in {"HOSPITAL", "MEDICAL CENTER", "CLINIC"}:
                    continue
                if len(value.split()) > 5:
                    continue

                start, end = match.span(1) if match.groups() else match.span()
                entity = Entity(
                    type=EntityType.HOSPITAL,
                    value=value,
                    start=start,
                    end=end,
                    confidence=0.80,
                )
                entities.append(entity)
                detected_values.add(value)

        return entities
