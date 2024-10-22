"""Caching utilities for pyfuzon.

This module provides functions to help manage the cache of TermMatchers.
Cache keys are built by fuzon using the sorted source paths. For each path,
a stamp is computed as follows (missing values are replaced by empty strings):
    + file path: {path}-{size}-{last-modified-datetime}
    + url: {url}-{etag-checksum}-{last-modified-datetime}
All stamps are then concatenated and hash of the result is used as the cache key.

Cache paths adhere to the following specifications:
    + Linux: XDG base / user directory
    + Windows: Known folder API
    + MacOS: Standard Directories guidelines
For more information, see: https://github.com/dirs-dev/dirs-rs
"""

from pathlib import Path

from .matcher import TermMatcher
from .pyfuzon import (
    get_cache_key as _get_cache_key,
    get_cache_path as _get_cache_path,
    cache_by_source as _cache_by_source,
    load_by_source as _load_by_source,
)

def get_cache_key(sources: list[str]) -> str:
    """Return a deterministic cache key based on a collection of source paths."""
    return _get_cache_key(sources)

def get_cache_path(sources: list[str]) -> Path:
    """Return a full platform-specific cache path based on a collection of source paths."""
    return Path(_get_cache_path(sources))

def cache_by_source(sources: list[str]):
    """Save each source into an independent TermMatcher cache file."""
    _cache_by_source(sources)

def load_by_source(sources: list[str]) -> TermMatcher:
    """Load and combine single-source cache entries into a combined TermMatcher."""
    terms = _load_by_source(sources)
    return TermMatcher(terms)
