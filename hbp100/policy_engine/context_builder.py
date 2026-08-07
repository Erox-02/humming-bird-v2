from typing import Optional

from hbp100.schemas.entity import Entity


class ContextBuilder:
    """
    Builds context strings for privacy decisions.

    Format: entity_type [SEP] original_text
    """

    SEPARATOR = " [SEP] "

    def build_context(
        self,
        entity: Entity,
        original_text: str,
        intent: Optional[str] = None,
    ) -> str:
        """
        Build context string for a single entity.

        Args:
            entity: Entity to decide on
            original_text: Complete original text
            intent: User intent (included if provided)

        Returns:
            Context string for model prediction
        """
        parts = []

        # Add intent if provided
        if intent:
            parts.append(intent.strip())

        # Add entity type
        parts.append(entity.type.value)

        # Add entity value for context
        parts.append(entity.value)

        # Add surrounding context
        context = self._extract_surrounding_context(
            original_text,
            entity.start,
            entity.end,
            window=100
        )

        # Add the full text (truncated)
        max_len = 1000
        if len(original_text) > max_len:
            context_parts = []
            if entity.start > 100:
                context_parts.append("...")
            context_parts.append(original_text[max(0, entity.start - 100):min(len(original_text), entity.end + 100)])
            if entity.end < len(original_text) - 100:
                context_parts.append("...")
            full_text = "".join(context_parts)
        else:
            full_text = original_text

        parts.append(full_text)

        return self.SEPARATOR.join(parts)

    def _extract_surrounding_context(
        self,
        text: str,
        start: int,
        end: int,
        window: int = 100
    ) -> str:
        """Extract surrounding context around an entity."""
        context_start = max(0, start - window)
        context_end = min(len(text), end + window)

        context_parts = []
        if context_start > 0:
            context_parts.append("...")
        context_parts.append(text[context_start:context_end])
        if context_end < len(text):
            context_parts.append("...")

        return "".join(context_parts)

    def batch_build_contexts(
        self,
        entities: list,
        original_text: str,
        intent: Optional[str] = None,
    ) -> list:
        """
        Build context strings for multiple entities.

        Args:
            entities: List of Entity objects
            original_text: Complete original text
            intent: User intent

        Returns:
            List of context strings in same order as entities
        """
        return [
            self.build_context(entity, original_text, intent)
            for entity in entities
        ]