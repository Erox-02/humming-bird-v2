import re
from typing import List

from hbp100.extractors.base import BaseExtractor
from hbp100.schemas.entity import Entity, EntityType


class DateExtractor(BaseExtractor):
    """
    Extracts dates in various formats.

    Supports:
    - MM/DD/YYYY, MM-DD-YYYY, MM.DD.YYYY
    - YYYY/MM/DD, YYYY-MM-DD
    - March 15, 1985
    - 15 March 1985
    - March 1985
    - MM/DD, MM-DD
    """

    def _compile_patterns(self):
        """Compile regex patterns for date extraction."""
        self._patterns = [
            re.compile(r'\b\d{1,2}[/-]\d{1,2}[/-]\d{2,4}\b'),
            re.compile(r'\b\d{4}[/-]\d{1,2}[/-]\d{1,2}\b'),
            re.compile(r'\b[A-Z][a-z]+ \d{1,2},? \d{4}\b'),
            re.compile(r'\b\d{1,2} [A-Z][a-z]+ \d{4}\b'),
            re.compile(r'\b[A-Z][a-z]+ \d{4}\b'),
            re.compile(r'\b\d{1,2}[/-]\d{1,2}\b'),
        ]

    @property
    def supported_types(self) -> List[EntityType]:
        """List of entity types this extractor supports."""
        return [EntityType.DATE]

    def extract(self, text: str) -> List[Entity]:
        """Extract date entities from text."""
        self._validate_text(text)

        return self._extract_matches(text, self._patterns, EntityType.DATE, confidence=0.90)