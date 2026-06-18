from typing import List, Dict, Any, Optional
from hbp100.core.engine import HBP100
from hbp100.core.metadata import metadata_vault

_engine = None


def _get_engine() -> HBP100:
    """Get or create global engine instance."""
    global _engine
    if _engine is None:
        _engine = HBP100()
    return _engine


def mask(text: str, intent: str = "unknown") -> str:
    """
    Mask sensitive information in text.

    Args:
        text: Input text to process
        intent: User intent for context-aware decisions

    Returns:
        Text with sensitive information masked

    Examples:
        >>> mask("Patient: John Doe, Phone: (555) 123-4567")
        'Patient: [NAME_1], Phone: [PHONE_1]'
    """
    engine = _get_engine()
    result = engine.process(text, intent=intent)
    return result.masked_text


def restore(text: str) -> str:
    """
    Restore placeholders in text using metadata vault.

    Args:
        text: Text with placeholders to restore

    Returns:
        Text with placeholders restored to original values

    Examples:
        >>> restore("Patient [NAME_1] was discharged")
        'Patient John Doe was discharged'
    """
    engine = _get_engine()
    return engine.restore(text)


def process(text: str, intent: str = "unknown") -> Dict[str, Any]:
    """
    Process text and return full result.

    Args:
        text: Input text to process
        intent: User intent for context-aware decisions

    Returns:
        Dictionary with masked_text, metadata, has_pii, entities

    Examples:
        >>> result = process("Patient: John Doe")
        >>> result["masked_text"]
        'Patient: [NAME_1]'
        >>> result["metadata"]
        {'[NAME_1]': 'John Doe'}
    """
    engine = _get_engine()
    result = engine.process(text, intent=intent)
    return {
        "masked_text": result.masked_text,
        "metadata": result.metadata,
        "has_pii": result.has_pii,
        "entities": result.entities,
        "decisions": result.decisions,
    }


def batch_mask(texts: List[str], intent: str = "unknown") -> List[str]:
    """
    Mask multiple texts in batch.

    Args:
        texts: List of input texts
        intent: User intent for context-aware decisions

    Returns:
        List of masked texts

    Examples:
        >>> batch_mask(["Patient: John Doe", "Patient: Jane Smith"])
        ['Patient: [NAME_1]', 'Patient: [NAME_2]']
    """
    engine = _get_engine()
    results = engine.batch_process(texts, intent=intent)
    return [r.masked_text for r in results]


def show_metadata() -> Dict[str, str]:
    """Show current metadata mapping."""
    return metadata_vault.show()


def clear_metadata():
    """Clear metadata vault."""
    metadata_vault.clear()


def reset():
    """Reset global engine and metadata vault."""
    global _engine
    _engine = None
    metadata_vault.clear()