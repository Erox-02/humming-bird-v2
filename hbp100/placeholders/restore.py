import re
from typing import Dict, Optional, List
from hbp100.placeholders.metadata_vault import metadata_vault
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)

class PlaceholderRestorer:
    def __init__(self):
        self._vault = metadata_vault
        self._pattern = re.compile(r'\[[A-Z_]+_\d+\]')
        self.metadata = {}
        logger.debug("Placeholder restorer initialized")

    def restore(self, text: str, metadata: Optional[Dict[str, str]] = None) -> str:
        if not text:
            return text

        placeholders = self._pattern.findall(text)
        if not placeholders:
            return text

        if metadata is not None:
            mapping = metadata
        else:
            mapping = self._vault.get_all()

        if not mapping:
            logger.warning("No metadata available for restoration")
            return text

        restored = text
        for placeholder in sorted(mapping.keys(), key=len, reverse=True):
            if placeholder in restored:
                restored = restored.replace(placeholder, mapping[placeholder])

        return restored

    def restore_partial(
        self,
        text: str,
        entity_type: Optional[str] = None,
        metadata: Optional[Dict[str, str]] = None,
    ) -> str:
        if not text:
            return text

        if metadata is not None:
            mapping = metadata
        else:
            mapping = self._vault.get_all()

        if not mapping:
            return text

        if entity_type:
            pattern = re.compile(rf'\[{entity_type}_\d+\]')
            filtered = {k: v for k, v in mapping.items() if pattern.match(k)}
        else:
            filtered = mapping

        if not filtered:
            return text

        restored = text
        for placeholder in sorted(filtered.keys(), key=len, reverse=True):
            if placeholder in restored:
                restored = restored.replace(placeholder, filtered[placeholder])

        return restored

    def get_remaining_placeholders(self, text: str) -> List[str]:
        return self._pattern.findall(text)

    def count_placeholders(self, text: str) -> int:
        return len(self._pattern.findall(text))

    def has_placeholders(self, text: str) -> bool:
        return bool(self._pattern.search(text))

    def restore_custom(
        self,
        text: str,
        placeholder_mapping: Dict[str, str],
        preserve_unknown: bool = True,
    ) -> str:
        if not text or not placeholder_mapping:
            return text

        restored = text
        for placeholder, value in sorted(placeholder_mapping.items(), key=lambda x: len(x[0]), reverse=True):
            if placeholder in restored:
                restored = restored.replace(placeholder, value)

        if not preserve_unknown:
            restored = self._pattern.sub("", restored)

        return restored

    def peek(self, text: str) -> Dict[str, str]:
        placeholders = self._pattern.findall(text)
        mapping = self._vault.get_all()

        result = {}
        for ph in placeholders:
            if ph in mapping:
                result[ph] = mapping[ph]
            else:
                result[ph] = None

        return result

    def update_metadata(self, metadata: dict):
        """
        Update metadata mappings.
        """
        if hasattr(self, "metadata"):
            self.metadata.update(metadata)
        else:
            self.metadata = dict(metadata)