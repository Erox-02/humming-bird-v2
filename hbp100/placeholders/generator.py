import re
from typing import Dict, Optional
from collections import defaultdict
from hbp100.placeholders.metadata_vault import metadata_vault
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)

class PlaceholderGenerator:
    PLACEHOLDER_PATTERN = re.compile(r'\[([A-Z_]+)_(\d+)\]')

    def __init__(self):
        self._counters: Dict[str, int] = defaultdict(int)
        self._vault = metadata_vault
        logger.debug("Placeholder generator initialized")

    def generate(self, entity) -> str:
        entity_type = entity.type.value
        self._counters[entity_type] += 1
        count = self._counters[entity_type]
        placeholder = f"[{entity_type}_{count}]"
        self._vault.set(placeholder, entity.value)
        entity.placeholder = placeholder
        logger.debug(f"Generated placeholder {placeholder} for {entity_type}")
        return placeholder

    def get_metadata(self) -> Dict[str, str]:
        return self._vault.get_all()

    def get_counter(self, entity_type: str) -> int:
        return self._counters.get(entity_type, 0)

    def reset(self):
        self._counters.clear()
        logger.debug("Placeholder generator counters reset")

    def reset_all(self):
        self._counters.clear()
        self._vault.clear()
        logger.debug("Placeholder generator and vault reset")

    @staticmethod
    def is_valid_placeholder(text: str) -> bool:
        return bool(PlaceholderGenerator.PLACEHOLDER_PATTERN.match(text))

    @staticmethod
    def extract_placeholder_type(placeholder: str) -> Optional[str]:
        match = PlaceholderGenerator.PLACEHOLDER_PATTERN.match(placeholder)
        if match:
            return match.group(1)
        return None

    @staticmethod
    def extract_placeholder_count(placeholder: str) -> Optional[int]:
        match = PlaceholderGenerator.PLACEHOLDER_PATTERN.match(placeholder)
        if match:
            return int(match.group(2))
        return None
