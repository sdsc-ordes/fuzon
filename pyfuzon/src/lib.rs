use std::path::PathBuf;
use pyo3::prelude::*;
use core::fmt;

use fuzon::{get_source, gather_terms, TermMatcher};
use rff;

/// A struct to represent a term from an ontology.
/// This mirrors fuzon::Term while making it easier to use in Python.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Term {
    #[pyo3(get, set)]
    pub uri: String,
    #[pyo3(get, set)]
    pub label: String,
}

#[pymethods]
impl Term {
    #[new]
    pub fn new(uri: String, label: String) -> Self {
        Term { uri, label }
    }

    pub fn __str__(&self) -> String {
        format!("{} ({})", self.label, self.uri)
    }

    pub fn __repr__(&self) -> String {
        format!("{} ({})", self.label, self.uri)
        }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.uri)
    }
}


/// Returns a vector of similarity scores for each term to the query
#[pyfunction]
pub fn score_terms(query: String, terms: Vec<Term>) -> PyResult<Vec<f64>> {
    let scores: Vec<f64> = terms
        .into_iter()
        .map(|t| {
            rff::match_and_score(&query, &t.label.to_string())
                .and_then(|m| Some(m.1.to_owned()))
                .unwrap_or(0.0)
        })
        .collect();

    return Ok(scores);
}

/// Parse and filter RDF files to gather the union of all terms.
#[pyfunction]
pub fn parse_files(paths: Vec<String>) -> PyResult<Vec<Term>> {
        let readers = paths.iter().map(|p| get_source(p).unwrap()).collect();
        let terms = gather_terms(readers)
            .map(|t| Term::new(t.uri, t.label))
            .collect();
    
    return Ok(terms)
}

/// Extract terms from a serialized fuzon TermMatcher.
/// This is faster than parsing RDF files.
#[pyfunction]
pub fn load_terms(path: PathBuf) -> PyResult<Vec<Term>> {
    let terms: Vec<Term> = TermMatcher::load(&path)
        .unwrap()
        .terms
        .into_iter()
        .map(|t| Term::new(t.uri, t.label))
        .collect();

    return Ok(terms)
}

/// Serialize the provided terms as a fuzon TermMatcher.
#[pyfunction]
pub fn dump_terms(terms: Vec<Term>, path: PathBuf) -> PyResult<()> {
    let mut matcher = TermMatcher::new();
    matcher.terms = terms
        .into_iter()
        .map(|t| fuzon::Term{ uri: t.uri, label: t.label })
        .collect();
    matcher.dump(&path).unwrap();

    return Ok(())
}


#[pymodule]
fn pyfuzon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(score_terms, m)?)?;
    m.add_function(wrap_pyfunction!(parse_files, m)?)?;
    m.add_function(wrap_pyfunction!(load_terms, m)?)?;
    m.add_function(wrap_pyfunction!(dump_terms, m)?)?;
    m.add_class::<Term>()?;
    Ok(())
}
