use pyo3::prelude::*;
use core::fmt;

use fuzon::{get_source, gather_terms};
use rff;

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

#[pyfunction]
pub fn parse_files(paths: Vec<String>) -> PyResult<Vec<Term>> {
        let readers = paths.iter().map(|p| get_source(p).unwrap()).collect();
        let terms = gather_terms(readers)
            .map(|t| Term::new(t.uri, t.label))
            .collect();
        Ok(terms)
}


#[pymodule]
fn pyfuzon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(score_terms, m)?)?;
    m.add_function(wrap_pyfunction!(parse_files, m)?)?;
    m.add_class::<Term>()?;
    Ok(())
}
