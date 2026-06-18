import re
from typing import Dict, Set, Optional, Tuple
from hbp100.placeholders.metadata_vault import metadata_vault
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)

class PlaceholderValidator:
    def __init__(self, allowed_placeholders: Optional[Set[str]] = None):
        self.allowed_placeholders = allowed_placeholders or set()
        self._vault = metadata_vault
        self._pattern = re.compile(r'\[[A-Z_]+_\d+\]')
        logger.debug("Placeholder validator initialized")

    def validate(
        self,
        response: str,
        metadata: Optional[Dict[str, str]] = None,
        strict: bool = True,
    ) -> Tuple[bool, Optional[str]]:
        if not response:
            return True, None

        placeholders = self._pattern.findall(response)

        if not placeholders:
            return True, None

        if metadata is not None:
            valid_placeholders = set(metadata.keys())
        else:
            valid_placeholders = set(self._vault.get_all().keys())

        if not strict:
            invalid = [ph for ph in placeholders if ph not in valid_placeholders]
            if invalid:
                error = f"Invalid placeholder(s) detected: {', '.join(invalid)}"
                logger.warning(error)
                return False, error
            return True, None

        invalid = [ph for ph in placeholders if ph not in valid_placeholders]
        if invalid:
            allowed = list(valid_placeholders)[:10]
            error = (
                f"Hallucinated placeholder(s) detected: {', '.join(invalid[:5])}. "
                f"Allowed: {', '.join(allowed)}"
            )
            logger.warning(error)
            return False, error

        return True, None

    def sanitize_response(
        self,
        response: str,
        metadata: Optional[Dict[str, str]] = None,
        replacement: str = "[ERROR: INVALID PLACEHOLDER]",
    ) -> str:
        if not response:
            return response

        if metadata is not None:
            valid_placeholders = set(metadata.keys())
        else:
            valid_placeholders = set(self._vault.get_all().keys())

        placeholders = self._pattern.findall(response)
        sanitized = response

        for ph in placeholders:
            if ph not in valid_placeholders:
                sanitized = sanitized.replace(ph, replacement)
                logger.debug(f"Replaced unknown placeholder {ph}")

        return sanitized

    def update_allowed(self, placeholders: Set[str]):
        self.allowed_placeholders.update(placeholders)

    def reset(self):
        self.allowed_placeholders.clear()
        logger.debug("Placeholder validator reset")

    def get_allowed(self) -> Set[str]:
        return self.allowed_placeholders.copy()

    def is_allowed(self, placeholder: str) -> bool:
        return placeholder in self.allowed_placeholders
