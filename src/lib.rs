use core::fmt;
use std::{io::BufRead, path::Path};

use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use oxigraph::{
    io::GraphFormat,
    model::GraphNameRef,
};
use skim::prelude::*;

struct TermMatcher {
    terms: Vec<Term>,
}

impl TermMatcher{

    fn new(terms: Vec<Term>) -> Self {
        Self { terms }
    }

    fn match_terms(&self, query: String) {
        let (tx, rx) = bounded(self.terms.len()); // <- this should take a vec
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(true)
            .preview(Some(""))
            .build()
            .unwrap();
        self.terms.iter().filter(|t| t.label.contains(&query))
            .for_each(|t| tx.send(t).unwrap());
        drop(tx);
        Skim::run_with(&options, Some(rx));
            
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

impl SkimItem for Term {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.label)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(format!("{} ({})", self.label, self.uri))
    }
}

pub fn filter_terms(query: String, terms: impl Iterator<Item = Term>) {
    terms.filter(|t| t.label.contains(&query))
        .for_each(|t| println!("{}", t));
}


// Build in-memory kg, load all sources and query for uris and labels.
pub fn query(readers: Vec<impl BufRead>) -> impl Iterator<Item = Term> {
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
