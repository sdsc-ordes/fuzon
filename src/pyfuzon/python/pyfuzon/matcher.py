from dataclasses import dataclass
from typing import Self

from dataclasses import dataclass

from pyfuzon import Term, score_terms, parse_files, load_terms, dump_terms


@dataclass
class TermMatcher:
    """Fuzzy matches terms from RDF terminologies to input queries."""

    terms: list[Term]

    def top(self, query: str, n: int=5) -> list[Term]:
        """Return the n terms most similar to input query."""
        return self.rank(query)[:n]

    def rank(self, query: str) -> list[Term]:
        """Return all terms, ranked by query similarity."""
        scores = self.score(query)
        ranks = [
            i[0] for i in
            sorted(enumerate(scores), key=lambda x:x[1], reverse=True)
        ]
        return [self.terms[rank] for rank in ranks]

    def score(self, query: str) -> list[float]:
        """Return all terms with a similarity score to the query."""
        return score_terms(query, self.terms)

    @classmethod
    def from_files(cls, paths: list[str]) -> Self:
        """Create a TermMatcher from a list of paths to source ontologies.
        Both filepaths and URLs are supported.
        """
        terms = parse_files(paths)
        return cls(terms)

    @classmethod
    def load(cls, path):
        """Deserialize a TermMatcher object from disk."""
        terms = load_terms(path)
        return cls(terms)

    def dump(self, path):
        """Serialize to disk."""
        dump_terms(self.terms, path)
