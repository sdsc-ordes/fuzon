use core::fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_static::lazy_static;
use oxttl::TurtleParser;
use reqwest::blocking::Client;
use reqwest::Url;

use rff;


pub mod ui;

// HashMap of common annotation properties
lazy_static! {
    static ref ANNOTATIONS: HashSet<String> = {
        HashSet::from_iter(
            vec![
                "http://www.w3.org/2000/01/rdf-schema#label".to_string(),
                "http://schema.org/name".to_string(),
                "http://www.w3.org/2004/02/skos/core#prefLabel".to_string(),
                "http://www.w3.org/2004/02/skos/core#altLabel".to_string(),
                "http://xmlns.com/foaf/0.1/name".to_string(),
                "http://purl.org/dc/elements/1.1/title".to_string(),
                "http://xmlns.com/foaf/0.1/name".to_string(),
            ]
            .iter()
            .cloned(),
        )
    };
}

pub struct TermMatcher {
    pub terms: Vec<Term>,
}

impl TermMatcher {
    pub fn new() -> Self {
        TermMatcher { terms: Vec::new() }
    }
    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }
    pub fn rank_terms(&self, query: &str) -> Vec<(&Term, f64)> {
        rank_terms(query, self.terms.iter().collect())
    }
    pub fn top_terms(&self, query: &str, n: usize) -> Vec<&Term> {
        self.rank_terms(query)
            .into_iter()
            .take(n)
            .map(|t| t.0)
            .collect()
    }
    pub fn from_readers(readers: Vec<impl BufRead>) -> Self {
        let terms = gather_terms(readers).collect();
        TermMatcher { terms }
    }

    pub fn from_paths(paths: Vec<&str>) -> Result<Self> {
        let readers = paths.into_iter().map(|p| get_source(p).unwrap()).collect();
        let terms: Vec<Term> = gather_terms(readers).collect();
        Ok(TermMatcher { terms })
    }
}

#[derive(Debug, Clone)]
pub struct Term {
    pub uri: String,
    pub label: String,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.uri)
    }
}

pub fn get_source(path: &str) -> Result<Box<dyn BufRead>> {
    if let Ok(url) = Url::parse(path) {
        // Handle URL
        let client = Client::new();
        let response = client.get(url).send()?.error_for_status()?;
        let reader = BufReader::new(response);
        Ok(Box::new(reader)) // Return boxed reader for URL
    } else {
        // Handle file path
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Box::new(reader)) // Return boxed reader for file
    }
}
/// Returns the input term vector sorted by match score (best first),
/// along with the individual matching scores.
pub fn rank_terms<'a>(query: &str, terms: Vec<&'a Term>) -> Vec<(&'a Term, f64)> {
    let mut ranked: Vec<(&Term, f64)> = terms
        .into_iter()
        .map(|t| {
            (
                t,
                rff::match_and_score(query, &t.label.to_string())
                    .and_then(|m| Some(m.1.to_owned()))
                    .unwrap_or(0.0),
            )
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    return ranked;
}

// Load URI-label pairs from all source.
pub fn gather_terms(readers: Vec<impl BufRead>) -> impl Iterator<Item = Term> {
    // NOTE: May want to use bulk loader for better performances
    let mut terms = Vec::new();
    for reader in readers {
        let parser = TurtleParser::new().for_reader(reader);
        let mut out = parser
            .map(|t| t.expect("Error parsing RDF"))
            .filter(|t| ANNOTATIONS.contains(t.predicate.as_str()))
            .map(|t| Term {
                uri: t.subject.to_string(),
                label: t.object.to_string(),
            })
            .collect();
        terms.append(&mut out);
    }
    terms.into_iter()
}

