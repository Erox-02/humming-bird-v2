from typing import List, Dict, Any, Optional
from hbp100.core.engine import HBP100
from hbp100.core.metadata import metadata_vault

_engine = None


def _get_engine() -> HBP100:
    global _engine
    if _engine is None:
        _engine = HBP100()
    return _engine


def mask(text: str, intent: str = "unknown") -> str:
    engine = _get_engine()
    result = engine.process(text, intent=intent)
    return result.masked_text


def restore(text: str) -> str:
    engine = _get_engine()
    return engine.restore(text)


def process(text: str, intent: str = "unknown") -> Dict[str, Any]:
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
    engine = _get_engine()
    results = engine.batch_process(texts, intent=intent)
    return [r.masked_text for r in results]


def show_metadata() -> Dict[str, str]:
    return metadata_vault.show()


def clear_metadata():
    metadata_vault.clear()


def reset():
    global _engine
    _engine = None
    metadata_vault.clear()
