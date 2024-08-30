use core::fmt;
use std::io::BufRead;

use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use oxigraph::{
    io::GraphFormat,
    model::GraphNameRef,
};

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
            rff::match_and_score(&query, &t.to_string())
                .and_then(|m| Some(m.1.to_owned()))
                .unwrap_or(0.0)
        )
    }).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    return ranked
}


// Build in-memory kg, load all sources and query for uris and labels.
pub fn gather_terms(readers: Vec<impl BufRead>) -> impl Iterator<Item = Term> {
    let store = Store::new().unwrap();
    // NOTE: May want to use bulk loader for better performances
    for reader in readers {
        store.load_graph(
            reader,
            GraphFormat::Turtle,
            GraphNameRef::DefaultGraph,
            None,
        ).unwrap();
    }
    let results = store.query("
        SELECT ?uri ?label 
        WHERE { 
            ?uri <http://www.w3.org/2000/01/rdf-schema#label> ?label 
        }"
    ).unwrap();
    if let QueryResults::Solutions(sol) = results {
        sol.map(|r| r.unwrap())
            .map(|r| Term {
                uri: r.get("uri").unwrap().to_string(),
                label: r.get("label").unwrap().to_string(),
            })
    } else {
        panic!("Unexpected");
    }
}
