import logging
import sys
from typing import Optional

_logger = None


def setup_logger(
    name: str = "hbp100",
    level: str = "INFO",
    log_format: Optional[str] = None,
) -> logging.Logger:
    """
    Set up a logger with console handler.

    Args:
        name: Logger name
        level: Log level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
        log_format: Custom log format string

    Returns:
        Configured logger instance
    """
    logger = logging.getLogger(name)

    if logger.handlers:
        return logger

    level_map = {
        "DEBUG": logging.DEBUG,
        "INFO": logging.INFO,
        "WARNING": logging.WARNING,
        "ERROR": logging.ERROR,
        "CRITICAL": logging.CRITICAL,
    }
    logger.setLevel(level_map.get(level.upper(), logging.INFO))

    if log_format is None:
        log_format = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"

    handler = logging.StreamHandler(sys.stdout)
    formatter = logging.Formatter(log_format)
    handler.setFormatter(formatter)
    logger.addHandler(handler)

    return logger


def get_logger(name: Optional[str] = None) -> logging.Logger:
    """
    Get a logger instance.

    Args:
        name: Optional logger name (uses hbp100 if None)

    Returns:
        Logger instance
    """
    global _logger
    if _logger is None:
        _logger = setup_logger("hbp100")
    if name:
        return _logger.getChild(name)
    return _logger