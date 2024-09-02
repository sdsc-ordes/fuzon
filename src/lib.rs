use core::fmt;
use std::io::BufRead;

use oxrdf::vocab::rdfs;
use oxttl::TurtleParser;

use rff;

pub struct TermMatcher {
    terms: Vec<Term>,
}

impl TermMatcher {
    pub fn new() -> Self {
        TermMatcher {
            terms: Vec::new(),
        }
    }
    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }
    pub fn rank_terms(&self, query: String) -> Vec<(&Term, f64)> {
        rank_terms(query, self.terms.iter().collect())
    }
    pub fn top_terms(&self, query: String, n: usize) -> Vec<&Term> {
        self.rank_terms(query).into_iter().take(n).map(|t| t.0).collect()
    }
    pub fn from_readers(readers: Vec<impl BufRead>) -> Self {
        let terms = gather_terms(readers).collect();
        TermMatcher { terms }
    }
}

#[derive(Debug)]
pub struct Term {
    uri: String,
    label: String,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.uri)
    }
}

/// Returns the input term vector sorted by match score (best first),
/// along with the individual matching scores.
pub fn rank_terms<'a>(query: String, terms: Vec<&'a Term>) -> Vec<(&'a Term, f64)>{
    let mut ranked: Vec<(&Term, f64)> = terms
        .into_iter()
        .map(|t| {
        (
            t, 
            rff::match_and_score(&query, &t.label.to_string())
                .and_then(|m| Some(m.1.to_owned()))
                .unwrap_or(0.0)
        )
    }).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    return ranked
}


// Load URI-label pairs from all source.
pub fn gather_terms(readers: Vec<impl BufRead>) -> impl Iterator<Item = Term> {
    // NOTE: May want to use bulk loader for better performances
    let mut terms = Vec::new();
    for reader in readers {
        let parser = TurtleParser::new().parse_read(reader);
        let mut out = parser
            .map(|t| t.expect("Error parsing RDF"))
            .filter(|t| t.predicate.as_str() == rdfs::LABEL.as_str())
            .map(|t| Term {
                uri: t.subject.to_string(),
                label: t.object.to_string(),
            }).collect();
        terms.append(&mut out);
    }
    terms.into_iter()
}
