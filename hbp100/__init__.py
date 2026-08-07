"""
Main entry points:
    - mask(): Quick masking convenience function
    - restore(): Quick restoration convenience function
    - metadata_vault: Global metadata storage
    - HBP100: Advanced processing class

Examples:
    >>> from hbp100 import mask, restore
    >>> masked = mask("Patient: John Doe")
    >>> print(masked)
    Patient: [NAME_1]
    >>> restored = restore("Patient [NAME_1] discharged")
    >>> print(restored)
    Patient John Doe discharged
"""

from hbp100.version import __version__
from hbp100.core.engine import HBP100
from hbp100.core.metadata import metadata_vault
from hbp100.api import mask, restore, process, batch_mask, show_metadata, clear_metadata, reset

__all__ = [
    "__version__",
    "HBP100",
    "mask",
    "restore",
    "process",
    "batch_mask",
    "metadata_vault",
    "show_metadata",
    "clear_metadata",
    "reset",
]