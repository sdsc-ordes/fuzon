use core::fmt;
use pyo3::prelude::*;
use std::path::PathBuf;

use fuzon::{cache, gather_terms, get_source, TermMatcher};
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

    Ok(scores)
}

/// Parse and filter RDF files to gather the union of all terms.
#[pyfunction]
pub fn parse_files(paths: Vec<String>) -> PyResult<Vec<Term>> {
    let readers = paths.iter().map(|p| get_source(p).unwrap()).collect();
    let terms = gather_terms(readers)
        .map(|t| Term::new(t.uri, t.label))
        .collect();

    Ok(terms)
}

/// Extract terms from a serialized fuzon TermMatcher.
/// This is faster than parsing RDF files.
#[pyfunction]
pub fn load_terms(path: PathBuf) -> PyResult<Vec<Term>> {
    let terms: Vec<Term> = TermMatcher::load(&path)?
        .terms
        .into_iter()
        .map(|t| Term::new(t.uri, t.label))
        .collect();

    Ok(terms)
}

/// Serialize the provided terms as a fuzon TermMatcher.
#[pyfunction]
pub fn dump_terms(terms: Vec<Term>, path: PathBuf) -> PyResult<()> {
    let mut matcher = TermMatcher::new();
    matcher.terms = terms
        .into_iter()
        .map(|t| fuzon::Term {
            uri: t.uri,
            label: t.label,
        })
        .collect();
    matcher.dump(&path)?;

    Ok(())
}

/// Get a full platform-specific cache path based on input collection of sources.
#[pyfunction]
pub fn get_cache_path(sources: Vec<String>) -> PyResult<String> {
    let mut src_ref = sources.iter().map(|s| s.as_str()).collect();
    let cache_path = cache::get_cache_path(&mut src_ref)?;

    Ok(cache_path.to_str().unwrap().to_owned())
}

/// Get a deterministic cache key based on input collection of sources
#[pyfunction]
pub fn get_cache_key(sources: Vec<String>) -> PyResult<String> {
    let mut src_ref = sources.iter().map(|s| s.as_str()).collect();

    Ok(cache::get_cache_key(&mut src_ref)?)
}

/// Save each source in a dedicated TermMatcher cache file.
#[pyfunction]
pub fn cache_by_source(sources: Vec<String>) -> PyResult<()> {
    let src_ref = sources.iter().map(|s| s.as_str()).collect();
    cache::cache_by_source(src_ref)?;

    Ok(())
}

/// Load terms from individual TermMatcher cache files for each source.
#[pyfunction]
pub fn load_by_source(sources: Vec<String>) -> PyResult<Vec<Term>> {
    let src_ref = sources.iter().map(|s| s.as_str()).collect();
    let terms = cache::load_by_source(src_ref)?
        .terms
        .into_iter()
        .map(|t| Term::new(t.uri, t.label))
        .collect();

    Ok(terms)
}

#[pymodule]
fn pyfuzon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(score_terms, m)?)?;
    m.add_function(wrap_pyfunction!(parse_files, m)?)?;
    m.add_function(wrap_pyfunction!(load_terms, m)?)?;
    m.add_function(wrap_pyfunction!(dump_terms, m)?)?;
    m.add_function(wrap_pyfunction!(get_cache_key, m)?)?;
    m.add_function(wrap_pyfunction!(get_cache_path, m)?)?;
    m.add_function(wrap_pyfunction!(cache_by_source, m)?)?;
    m.add_function(wrap_pyfunction!(load_by_source, m)?)?;
    m.add_class::<Term>()?;

    Ok(())
}
