import re
from typing import List

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class EmailExtractor(BaseExtractor):
    def _compile_patterns(self):
        self._patterns = [
            re.compile(r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b')
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        return [EntityType.EMAIL]

    def extract(self, text: str) -> List[Entity]:
        self._validate_text(text)
        return self._extract_matches(text, self._patterns, EntityType.EMAIL, confidence=0.95)
