import joblib
from pathlib import Path
from typing import List, Optional, Dict, Any
from importlib import resources

from hbp100.schemas.entity import Entity
from hbp100.schemas.decision import PrivacyDecision, DecisionType
from hbp100.policy_engine.context_builder import ContextBuilder
from hbp100.utils.logger import get_logger

from hbp100.assets import get_asset_path
import joblib

MODEL = joblib.load(
    get_asset_path("hbp100-v2.pkl")
)

VECTORIZER = joblib.load(
    get_asset_path("vectorizer.pkl")
)

logger = get_logger(__name__)


class PrivacyPredictor:
    """
    Predicts privacy decisions for entities.

    Uses hbp100-v2 LightGBM model with TF-IDF vectorization.
    """

    def __init__(
        self,
        model_path: Optional[str] = None,
        vectorizer_path: Optional[str] = None,
        context_builder: Optional[ContextBuilder] = None,
    ):
        """
        Initialize the predictor.

        Args:
            model_path: Optional path to model file
            vectorizer_path: Optional path to vectorizer file
            context_builder: Optional context builder instance
        """
        self.model_path = model_path
        self.vectorizer_path = vectorizer_path
        self.context_builder = context_builder or ContextBuilder()

        self._model = None
        self._vectorizer = None
        self._loaded = False

        # Don't load by default - use lazy loading
        logger.info("Privacy predictor initialized (assets not loaded)")

    def load_assets(self) -> bool:
        """Load model and vectorizer from disk."""
        try:
            # Try to load from provided paths first
            if self.model_path and self.vectorizer_path:
                model_path = Path(self.model_path)
                vectorizer_path = Path(self.vectorizer_path)

                if model_path.exists() and vectorizer_path.exists():
                    self._model = joblib.load(model_path)
                    self._vectorizer = joblib.load(vectorizer_path)
                    self._loaded = True
                    logger.info("Assets loaded from custom paths")
                    return True

            # Fall back to package assets
            try:
                with resources.files("hbp100.assets") as assets_dir:
                    model_file = assets_dir / "hbp100-v2.pkl"
                    vectorizer_file = assets_dir / "vectorizer.pkl"

                    if model_file.exists() and vectorizer_file.exists():
                        self._model = joblib.load(model_file)
                        self._vectorizer = joblib.load(vectorizer_file)
                        self._loaded = True
                        logger.info("Assets loaded from package")
                        return True
            except (ImportError, AttributeError, FileNotFoundError):
                pass

            # Try relative path as fallback
            for path in [
                Path("assets/hbp100-v2.pkl"),
                Path("../assets/hbp100-v2.pkl"),
                Path("hbp100/assets/hbp100-v2.pkl"),
            ]:
                if path.exists():
                    vectorizer_path = path.parent / "vectorizer.pkl"
                    if vectorizer_path.exists():
                        self._model = joblib.load(path)
                        self._vectorizer = joblib.load(vectorizer_path)
                        self._loaded = True
                        logger.info(f"Assets loaded from {path}")
                        return True

            logger.warning("No assets found, running in fallback mode")
            self._loaded = False
            return False

        except Exception as e:
            logger.error(f"Failed to load assets: {e}")
            self._loaded = False
            return False

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
        # Build context string
        context_string = self.context_builder.build_context(
            entity, original_text, intent
        )

        # Get prediction
        decision, confidence = self._predict_single(context_string)

        return PrivacyDecision(
            entity=entity,
            decision=decision,
            confidence=confidence,
            context_string=context_string,
        )

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
        if not entities:
            return []

        # Build context strings
        context_strings = self.context_builder.batch_build_contexts(
            entities, original_text, intent
        )

        # Get predictions
        decisions, confidences = self._predict_batch(context_strings)

        # Create decision objects
        return [
            PrivacyDecision(
                entity=entity,
                decision=decision,
                confidence=confidence,
                context_string=context_string,
            )
            for entity, decision, confidence, context_string in zip(
                entities, decisions, confidences, context_strings
            )
        ]

    def _predict_single(self, context_string: str) -> tuple:
        """Predict for a single context string."""
        if not self._loaded:
            # Fallback mode: always mask
            return DecisionType.MASK, 0.75

        try:
            # Vectorize
            X = self._vectorizer.transform([context_string])

            # Predict
            prediction = self._model.predict(X)[0]
            probabilities = self._model.predict_proba(X)[0]
            confidence = float(max(probabilities))

            decision = DecisionType.MASK if prediction == 1 else DecisionType.KEEP
            return decision, confidence

        except Exception as e:
            logger.error(f"Prediction error: {e}")
            return DecisionType.MASK, 0.50

    def _predict_batch(self, context_strings: List[str]) -> tuple:
        """Predict for multiple context strings."""
        if not self._loaded:
            return [DecisionType.MASK] * len(context_strings), [0.75] * len(context_strings)

        try:
            # Vectorize
            X = self._vectorizer.transform(context_strings)

            # Predict
            predictions = self._model.predict(X)
            probabilities = self._model.predict_proba(X)
            confidences = [float(max(probs)) for probs in probabilities]

            decisions = [
                DecisionType.MASK if pred == 1 else DecisionType.KEEP
                for pred in predictions
            ]

            return decisions, confidences

        except Exception as e:
            logger.error(f"Batch prediction error: {e}")
            return [DecisionType.MASK] * len(context_strings), [0.50] * len(context_strings)

    @property
    def is_loaded(self) -> bool:
        """Check if model is loaded."""
        return self._loaded

    def reload(self):
        """Reload model assets."""
        self.load_assets()