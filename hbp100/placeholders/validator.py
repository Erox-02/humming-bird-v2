import re
from typing import Dict, Set, Optional, Tuple

from hbp100.placeholders.generator import PlaceholderGenerator
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


class PlaceholderValidator:
    """
    Validates that LLM responses only use allowed placeholders.
    """

    def __init__(self, allowed_placeholders: Optional[Set[str]] = None):
        """
        Initialize the validator.

        Args:
            allowed_placeholders: Set of allowed placeholder strings
        """
        self.allowed_placeholders = allowed_placeholders or set()
        self._pattern = re.compile(r'\[[A-Z_]+_\d+\]')
        logger.debug("Placeholder validator initialized")

    def validate(
        self,
        response: str,
        metadata: Dict[str, str],
        strict: bool = True
    ) -> Tuple[bool, Optional[str]]:
        """
        Validate placeholders in a response.

        Args:
            response: LLM response text
            metadata: Metadata mapping placeholders to values
            strict: If True, reject any unknown placeholders

        Returns:
            Tuple of (is_valid, error_message)
        """
        # Find all placeholders in response
        placeholders = self._pattern.findall(response)

        if not placeholders:
            return True, None

        # Check each placeholder
        unknown = []
        for ph in placeholders:
            if ph not in metadata:
                unknown.append(ph)

        if unknown:
            allowed = list(metadata.keys())[:10]  # Show first 10
            error = (
                f"Hallucinated placeholder(s) detected: {', '.join(unknown[:5])}. "
                f"Allowed: {', '.join(allowed)}"
            )
            logger.warning(error)
            return False, error

        return True, None

    def sanitize_response(
        self,
        response: str,
        metadata: Dict[str, str],
        replacement: str = "[ERROR: INVALID PLACEHOLDER]"
    ) -> str:
        """
        Sanitize a response by replacing unknown placeholders.

        Args:
            response: LLM response text
            metadata: Metadata mapping placeholders to values
            replacement: Replacement text for unknown placeholders

        Returns:
            Sanitized response
        """
        placeholders = self._pattern.findall(response)

        sanitized = response
        for ph in placeholders:
            if ph not in metadata:
                sanitized = sanitized.replace(ph, replacement)
                logger.debug(f"Replaced unknown placeholder {ph}")

        return sanitized

    def update_allowed(self, metadata: Dict[str, str]):
        """
        Update allowed placeholders from metadata.

        Args:
            metadata: Metadata mapping placeholders to values
        """
        self.allowed_placeholders.update(metadata.keys())

    def reset(self):
        """Reset allowed placeholders."""
        self.allowed_placeholders.clear()
        logger.debug("Placeholder validator reset")

    def get_allowed(self) -> Set[str]:
        """Get allowed placeholders."""
        return self.allowed_placeholders.copy()