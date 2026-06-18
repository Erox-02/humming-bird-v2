from typing import Dict, Optional
from threading import Lock


class MetadataVault:
    """
    Thread-safe metadata storage for placeholder mappings.

    This vault stores the mapping between placeholders (e.g., [NAME_1])
    and their original values (e.g., "John Doe").

    Examples:
        >>> vault = MetadataVault()
        >>> vault.update({"[NAME_1]": "John Doe"})
        >>> vault.get("[NAME_1]")
        'John Doe'
        >>> vault.get_all()
        {'[NAME_1]': 'John Doe'}
        >>> vault.clear()
    """

    def __init__(self):
        """Initialize empty metadata vault."""
        self._metadata: Dict[str, str] = {}
        self._lock = Lock()

    def update(self, metadata: Dict[str, str]) -> None:
        """
        Update vault with new metadata.

        Args:
            metadata: Dictionary of placeholder-value mappings
        """
        with self._lock:
            self._metadata.update(metadata)

    def get(self, placeholder: str) -> Optional[str]:
        """
        Get original value for a placeholder.

        Args:
            placeholder: Placeholder string (e.g., "[NAME_1]")

        Returns:
            Original value or None if not found
        """
        with self._lock:
            return self._metadata.get(placeholder)

    def get_all(self) -> Dict[str, str]:
        """
        Get all metadata mappings.

        Returns:
            Copy of all placeholder-value mappings
        """
        with self._lock:
            return self._metadata.copy()

    def show(self) -> Dict[str, str]:
        """
        Alias for get_all().

        Returns:
            Copy of all placeholder-value mappings
        """
        return self.get_all()

    def clear(self) -> None:
        """Clear all metadata."""
        with self._lock:
            self._metadata.clear()

    def __len__(self) -> int:
        """Get number of metadata entries."""
        with self._lock:
            return len(self._metadata)

    def __contains__(self, placeholder: str) -> bool:
        """Check if placeholder exists in vault."""
        with self._lock:
            return placeholder in self._metadata

    def __getitem__(self, placeholder: str) -> str:
        """Get original value for a placeholder."""
        with self._lock:
            return self._metadata[placeholder]


# Global metadata vault instance
metadata_vault = MetadataVault()