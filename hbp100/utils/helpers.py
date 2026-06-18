import re
from typing import Optional


def truncate_text(text: str, max_length: int = 1000, ellipsis: str = "...") -> str:
    """
    Truncate text to maximum length while preserving word boundaries.

    Args:
        text: Text to truncate
        max_length: Maximum length
        ellipsis: String to append when truncated

    Returns:
        Truncated text
    """
    if len(text) <= max_length:
        return text

    # Truncate at word boundary
    truncated = text[:max_length]
    last_space = truncated.rfind(' ')

    if last_space > 0:
        truncated = truncated[:last_space]

    return truncated + ellipsis


def safe_regex_escape(text: str) -> str:
    """
    Escape regex special characters in text.

    Args:
        text: Text to escape

    Returns:
        Escaped text safe for regex use
    """
    return re.escape(text)


def normalize_text(text: str) -> str:
    """
    Normalize text for processing.

    Args:
        text: Input text

    Returns:
        Normalized text
    """
    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text)
    # Strip leading/trailing whitespace
    text = text.strip()
    return text


def extract_with_word_boundary(text: str, pattern: str) -> list:
    """
    Extract matches with word boundaries.

    Args:
        text: Input text
        pattern: Pattern to match

    Returns:
        List of matches
    """
    full_pattern = r'\b' + pattern + r'\b'
    return re.findall(full_pattern, text, re.IGNORECASE)


def is_mostly_uppercase(text: str, threshold: float = 0.7) -> bool:
    """
    Check if text is mostly uppercase.

    Args:
        text: Input text
        threshold: Proportion of uppercase characters required

    Returns:
        True if mostly uppercase, False otherwise
    """
    if not text:
        return False

    letters = sum(1 for c in text if c.isalpha())
    if letters == 0:
        return False

    uppercase = sum(1 for c in text if c.isupper())
    return uppercase / letters >= threshold


def extract_numbers(text: str) -> list:
    """
    Extract all numbers from text.

    Args:
        text: Input text

    Returns:
        List of numbers as strings
    """
    return re.findall(r'\b\d+\b', text)


def is_valid_email(email: str) -> bool:
    """
    Validate email format.

    Args:
        email: Email string

    Returns:
        True if valid, False otherwise
    """
    pattern = r'^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'
    return bool(re.match(pattern, email))