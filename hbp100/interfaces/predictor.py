from abc import ABC, abstractmethod
from typing import List, Optional

from hbp100.schemas.entity import Entity
from hbp100.schemas.decision import PrivacyDecision


class PrivacyPredictorInterface(ABC):
    """
    Interface for privacy policy engines.

    All privacy predictors must implement this interface.
    """

    @abstractmethod
    def predict(
        self,
        entity: Entity,
        original_text: str,
        intent: Optional[str] = None,
    ) -> PrivacyDecision:
        """
        Predict privacy decision for a single entity.

        Args:
            entity: Entity to predict on
            original_text: Complete original text
            intent: Optional user intent

        Returns:
            PrivacyDecision object
        """
        pass

    @abstractmethod
    def predict_batch(
        self,
        entities: List[Entity],
        original_text: str,
        intent: Optional[str] = None,
    ) -> List[PrivacyDecision]:
        """
        Predict privacy decisions for multiple entities.

        Args:
            entities: List of entities to predict on
            original_text: Complete original text
            intent: Optional user intent

        Returns:
            List of PrivacyDecision objects
        """
        pass

    @abstractmethod
    def load_assets(self) -> bool:
        """
        Load model and vectorizer assets.

        Returns:
            True if loaded successfully, False otherwise
        """
        pass

    @property
    @abstractmethod
    def is_loaded(self) -> bool:
        """Check if predictor is loaded."""
        pass