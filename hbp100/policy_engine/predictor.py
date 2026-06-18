import joblib
from typing import List, Optional

from hbp100.assets import get_asset_path
from hbp100.policy_engine.context_builder import ContextBuilder
from hbp100.schemas.decision import DecisionType, PrivacyDecision
from hbp100.schemas.entity import Entity
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


class PrivacyPredictor:
    """
    Predicts privacy decisions using hbp100-v2.
    """

    def __init__(
        self,
        model_path: Optional[str] = None,
        vectorizer_path: Optional[str] = None,
        context_builder: Optional[ContextBuilder] = None,
    ):
        self.model_path = model_path
        self.vectorizer_path = vectorizer_path
        self.context_builder = context_builder or ContextBuilder()

        self._model = None
        self._vectorizer = None
        self._loaded = False

        logger.info("Privacy predictor initialized")

    def load_assets(self) -> bool:
        """Load model and vectorizer lazily."""
        if self._loaded:
            return True

        try:
            model_file = (
                self.model_path
                if self.model_path
                else get_asset_path("hbp100-v2.pkl")
            )

            vectorizer_file = (
                self.vectorizer_path
                if self.vectorizer_path
                else get_asset_path("vectorizer.pkl")
            )

            if model_file is None or vectorizer_file is None:
                logger.error("Model assets not found")
                return False

            self._model = joblib.load(model_file)
            self._vectorizer = joblib.load(vectorizer_file)

            self._loaded = True
            logger.info("Assets loaded successfully")

            return True

        except Exception as e:
            logger.error(f"Failed to load assets: {e}")
            self._loaded = False
            return False

    def _predict_single(self, context_string: str):
        if not self._loaded:
            self.load_assets()

        if not self._loaded:
            return DecisionType.MASK, 0.50

        try:
            X = self._vectorizer.transform([context_string])

            pred = self._model.predict(X)[0]
            probs = self._model.predict_proba(X)[0]

            decision = (
                DecisionType.MASK
                if pred == 1
                else DecisionType.KEEP
            )

            confidence = float(max(probs))

            return decision, confidence

        except Exception as e:
            logger.error(f"Prediction error: {e}")
            return DecisionType.MASK, 0.50

    def _predict_batch(self, context_strings: List[str]):
        if not self._loaded:
            self.load_assets()

        if not self._loaded:
            return (
                [DecisionType.MASK] * len(context_strings),
                [0.50] * len(context_strings),
            )

        try:
            X = self._vectorizer.transform(context_strings)

            preds = self._model.predict(X)
            probs = self._model.predict_proba(X)

            decisions = [
                DecisionType.MASK if p == 1 else DecisionType.KEEP
                for p in preds
            ]

            confidences = [
                float(max(prob))
                for prob in probs
            ]

            return decisions, confidences

        except Exception as e:
            logger.error(f"Batch prediction error: {e}")

            return (
                [DecisionType.MASK] * len(context_strings),
                [0.50] * len(context_strings),
            )

    def predict(
        self,
        entity: Entity,
        original_text: str,
        intent: Optional[str] = None,
    ) -> PrivacyDecision:

        context_string = self.context_builder.build_context(
            entity,
            original_text,
            intent,
        )

        decision, confidence = self._predict_single(
            context_string
        )

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

        context_strings = self.context_builder.batch_build_contexts(
            entities,
            original_text,
            intent,
        )

        decisions, confidences = self._predict_batch(
            context_strings
        )

        return [
            PrivacyDecision(
                entity=e,
                decision=d,
                confidence=c,
                context_string=s,
            )
            for e, d, c, s in zip(
                entities,
                decisions,
                confidences,
                context_strings,
            )
        ]

    @property
    def is_loaded(self) -> bool:
        return self._loaded

    def reload(self):
        self._loaded = False
        self.load_assets()
