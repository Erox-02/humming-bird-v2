from typing import Dict, Optional
from threading import Lock

class MetadataVault:
    def __init__(self):
        self._metadata: Dict[str, str] = {}
        self._lock = Lock()

    def set(self, placeholder: str, value: str) -> None:
        with self._lock:
            self._metadata[placeholder] = value

    def get(self, placeholder: str) -> Optional[str]:
        with self._lock:
            return self._metadata.get(placeholder)

    def update(self, mappings: Dict[str, str]) -> None:
        with self._lock:
            self._metadata.update(mappings)

    def get_all(self) -> Dict[str, str]:
        with self._lock:
            return self._metadata.copy()

    def show(self) -> Dict[str, str]:
        return self.get_all()

    def clear(self) -> None:
        with self._lock:
            self._metadata.clear()

    def remove(self, placeholder: str) -> None:
        with self._lock:
            self._metadata.pop(placeholder, None)

    def __len__(self) -> int:
        with self._lock:
            return len(self._metadata)

    def __contains__(self, placeholder: str) -> bool:
        with self._lock:
            return placeholder in self._metadata

    def __getitem__(self, placeholder: str) -> str:
        with self._lock:
            return self._metadata[placeholder]

metadata_vault = MetadataVault()
