from typing import Optional, Dict, Any, List
from dataclasses import dataclass, field

from hbp100.core.pipeline import PrivacyPipeline, PipelineResult
from hbp100.core.metadata import MetadataVault, metadata_vault as default_vault
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


@dataclass
class EngineResult:
    """
    Result from HBP100 engine processing.

    Provides a clean public API for processing results.
    """

    original_text: str
    masked_text: str
    metadata: Dict[str, str] = field(default_factory=dict)
    has_pii: bool = False
    entities: List[Dict[str, Any]] = field(default_factory=list)
    decisions: List[Dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization."""
        return {
            "original_text": self.original_text,
            "masked_text": self.masked_text,
            "metadata": self.metadata,
            "has_pii": self.has_pii,
            "entities": self.entities,
            "decisions": self.decisions,
        }


class HBP100:
    
    def __init__(
        self,
        model_path: Optional[str] = None,
        vectorizer_path: Optional[str] = None,
        metadata_vault: Optional[MetadataVault] = None,
        lazy_load: bool = True,
    ):
        """
        Initialize HBP100 engine.

        Args:
            model_path: Optional path to model file
            vectorizer_path: Optional path to vectorizer file
            metadata_vault: Optional metadata vault instance
            lazy_load: Whether to lazy-load model on first use
        """
        self._pipeline = PrivacyPipeline(
            model_path=model_path,
            vectorizer_path=vectorizer_path,
            lazy_load=lazy_load,
        )
        self._metadata_vault = metadata_vault or default_vault
        self._loaded = not lazy_load

        if not lazy_load:
            self._ensure_loaded()

        logger.info("HBP100 engine initialized")

    def _ensure_loaded(self):
        """Ensure model assets are loaded."""
        if not self._loaded:
            self._pipeline.load_assets()
            self._loaded = self._pipeline.is_loaded
            if self._loaded:
                logger.info("Assets loaded successfully")
            else:
                logger.warning("Assets failed to load, running in fallback mode")

    def process(
        self,
        text: str,
        intent: str = "unknown",
        return_entities: bool = False,
    ) -> EngineResult:
        """
        Process text through the privacy pipeline.

        Args:
            text: Input text to process
            intent: User intent for context-aware decisions
            return_entities: Whether to include entities and decisions in result

        Returns:
            EngineResult with masked text and metadata

        Raises:
            ValueError: If text is empty
        """
        self._ensure_loaded()

        pipeline_result = self._pipeline.process(
            text=text,
            intent=intent,
            return_decisions=return_entities,
        )

        # Update metadata vault
        self._metadata_vault.update(pipeline_result.metadata)

        return EngineResult(
            original_text=pipeline_result.original_text,
            masked_text=pipeline_result.masked_text,
            metadata=pipeline_result.metadata,
            has_pii=pipeline_result.has_pii,
            entities=[e.to_dict() for e in pipeline_result.entities] if return_entities else [],
            decisions=[d.to_dict() for d in pipeline_result.decisions] if return_entities else [],
        )

    def restore(self, text: str) -> str:
        """
        Restore placeholders in text using metadata vault.

        Args:
            text: Text with placeholders to restore

        Returns:
            Text with placeholders restored

        Examples:
            >>> engine.restore("Patient [NAME_1] was discharged")
            'Patient John Doe was discharged'
        """
        metadata = self._metadata_vault.get_all()
        return self._pipeline.restore_placeholders(text, metadata)

    def restore_with_metadata(self, text: str, metadata: Dict[str, str]) -> str:
        """
        Restore placeholders in text using provided metadata.

        Args:
            text: Text with placeholders to restore
            metadata: Metadata mapping placeholders to original values

        Returns:
            Text with placeholders restored
        """
        return self._pipeline.restore_placeholders(text, metadata)

    def validate_response(self, response: str) -> tuple:
        """
        Validate placeholders in an LLM response.

        Args:
            response: LLM response text

        Returns:
            Tuple of (is_valid, error_message, sanitized_response)
        """
        metadata = self._metadata_vault.get_all()
        is_valid, error = self._pipeline.validate_response(response, metadata)

        if is_valid:
            return True, None, response
        else:
            sanitized = self._pipeline.validator.sanitize_response(response, metadata)
            return False, error, sanitized

    def batch_process(
        self,
        texts: List[str],
        intent: str = "unknown",
    ) -> List[EngineResult]:
        """
        Process multiple texts in batch.

        Args:
            texts: List of input texts
            intent: User intent for context-aware decisions

        Returns:
            List of EngineResult objects
        """
        self._ensure_loaded()

        results = []
        for text in texts:
            try:
                result = self.process(text, intent)
                results.append(result)
            except Exception as e:
                logger.error(f"Failed to process text: {e}")
                results.append(
                    EngineResult(
                        original_text=text,
                        masked_text=text,
                        metadata={},
                        has_pii=False,
                    )
                )
        return results

    def reset(self):
        """Reset engine state."""
        self._pipeline.reset()
        self._loaded = False
        logger.info("Engine reset")

    @property
    def is_loaded(self) -> bool:
        """Check if model assets are loaded."""
        return self._loaded

    @property
    def metadata(self) -> Dict[str, str]:
        """Get current metadata."""
        return self._metadata_vault.get_all()

    @property
    def version(self) -> str:
        """Get package version."""
        from hbp100.version import __version__
        return __version__

    def __enter__(self):
        """Support context manager."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Clean up on context exit."""
        self.reset()