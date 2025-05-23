# pyfuzon

Python bindings for the fuzon library.

## Installation

Pyfuzon is distributed on PyPI and can be installed with:

```shell
pip install pyfuzon
```

## Usage

`TermMatcher` is the central object in pyfuzon. It can be built from a list of RDF files, either locally or from URLs. It exposes methods to rank, score or search top N terms for their similarity with input text.

```python
from pyfuzon.matcher import TermMatcher

matcher = TermMatcher.from_files(["https://example.org/onto1.ttl", "/data/onto2.ttl"])
matcher.terms #accesses the list of terms loaded from input files
matcher.score("query") # returns the match score of each term for the input query.
matcher.rank("query") # returns the list of terms sorted by similarity with the query.
matcher.top("query", 5) # shows top 5 most similar results (sorted).
```

Fuzon's caching mechanism is also available from python via the `pyfuzon.cache`.

```python
from pathlib import Path
from pyfuzon import cache

sources = ["data/onto1.ttl", "data/onto2.ttl"]

# This initializes the fuzon cache dir, `~/.cache/fuzon` on linux.
Path(cache.get_cache_path(sources)).parent.mkdir(parents=True, exist_ok=True)
```

There are two way to use caching.

By source, where each ontology is cached/loaded indepdently:

> [!TIP]
> This is preferred if mix and matching many ontologies, as this reduces duplication in the caching folder.

```python
# each source cached under a separate key.
cache.cache_by_source(sources)

# multiple entries merged them into a matcher
matcher = cache.load_by_source(sources)
```

Or by matcher, where multiple ontologies are combined into a single cache entry:

> [!TIP]
> This is preferred if always reusing the same combination(s) of ontologies, as the loading process is faster.

```python
# Generate a single cache key from multiple ontologies
cache_path = cache.get_cache_path(sources)
# Dump the combined matcher to a file
matcher.dump(cache_path)
# Load combined matcher in one go
matcher = TermMatcher.load(cache_path)
```
