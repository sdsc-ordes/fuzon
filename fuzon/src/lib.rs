use core::fmt;
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    ops::Add,
    path::Path,
};

use anyhow::Result;
use lazy_static::lazy_static;
use oxrdfio::{RdfFormat, RdfParser};
use oxrdf::Subject;
use postcard;
use reqwest::{blocking::Client, Url};
use serde::{Deserialize, Serialize};

use rff;

pub mod cache;
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TermMatcher {
    pub terms: Vec<Term>,
}

impl Add for TermMatcher {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // union of terms
        let terms = self
            .terms
            .into_iter()
            .chain(rhs.terms.into_iter())
            .collect::<HashSet<Term>>()
            .into_iter()
            .collect();

        TermMatcher { terms }
    }
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
    pub fn from_readers(readers: Vec<(impl BufRead, RdfFormat)>) -> Self {
        let terms = gather_terms(readers).collect();

        TermMatcher { terms }
    }

    pub fn from_paths(paths: Vec<&str>) -> Result<Self> {
        let readers = paths.into_iter().map(|p| get_source(p).unwrap()).collect();
        let terms: Vec<Term> = gather_terms(readers).collect();

        Ok(TermMatcher { terms })
    }

    pub fn load(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        let matcher = postcard::from_bytes(&bytes)?;

        Ok(matcher)
    }

    pub fn dump(&self, path: &Path) -> Result<()> {
        let bytes = postcard::to_allocvec(&self).unwrap();
        std::fs::write(path, &bytes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Term {
    pub uri: String,
    pub label: String,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.uri)
    }
}

/// Get an rdf reader along with its format from a path
pub fn get_source(path: &str) -> Result<(Box<dyn BufRead>, RdfFormat)> {
    let file_ext = path.split('.').last().unwrap();
    let ext = match file_ext {
        "owl" => "xml",
        "rdf" => "xml",
        _ => file_ext,
    };
    let format = RdfFormat::from_extension(ext).expect("Unkown file extension");
    if let Ok(url) = Url::parse(path) {
        // Handle URL
        let client = Client::new();
        let response = client.get(url).send()?.error_for_status()?;
        let reader = BufReader::new(response);
        Ok((Box::new(reader), format)) // Return boxed reader for URL
    } else {
        // Handle file path
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok((Box::new(reader), format)) // Return boxed reader for file
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

    ranked
}

// Load URI-label pairs from all sources.
pub fn gather_terms(readers: Vec<(impl BufRead, RdfFormat)>) -> impl Iterator<Item = Term> {
    // NOTE: May want to use bulk loader for better performances
    let mut terms = Vec::new();
    for (reader, format) in readers {
        let parser = RdfParser::from_format(format).for_reader(reader);
        // Drop blank nodes and filter by common annotation properties
        let mut out = parser
            .map(|t| t.expect("Error parsing RDF"))
            .filter(|t| if let Subject::NamedNode(_) = t.subject {true} else {false})
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn matcher_from_source() {
        let source = vec!["../data/test_schema.ttl"];
        let matcher = TermMatcher::from_paths(source).unwrap();
        assert_eq!(matcher.terms.len(), 11);
    }

    #[test]
    fn rank_terms() {
        let source = vec!["../data/test_schema.ttl"];
        let matcher = TermMatcher::from_paths(source).unwrap();
        let query = "Person";
        let ranked = matcher.rank_terms(query);
        assert_eq!(ranked[0].0.label, "\"Person\"");
    }

    #[test]
    fn serde() {
        let source = vec!["../data/test_schema.ttl"];
        let matcher = TermMatcher::from_paths(source).unwrap();
        let out = tempfile::NamedTempFile::new().unwrap();
        let _ = matcher.dump(&out.path());
        let loaded = TermMatcher::load(&out.path()).unwrap();
        assert_eq!(matcher, loaded);
    }
}
