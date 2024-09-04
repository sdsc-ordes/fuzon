from dataclasses import dataclass
from fuzon import Term, score_terms

@dataclass
class TermMatcher:
    terms: list[Term]
    
    def top(self, query: str, n: int=5) -> list[Term]:
        return self.rank(query)[:n]

    def rank(self, query: str) -> list[Term]:
        scores = self.score(query)
        ranks = [
            i[0] for i in 
            sorted(enumerate(scores), key=lambda x:x[1], reverse=True)
        ]
        return [self.terms[rank] for rank in ranks]

    def score(self, query: str) -> list[float]:
        return score_terms(query, self.terms)
