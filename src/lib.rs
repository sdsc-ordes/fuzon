use std::{io::BufRead, path::Path};

use oxigraph::store::Store;
use oxigraph::{
    io::GraphFormat,
    model::GraphNameRef,
};
use skim::prelude::*;

struct TermMatcher {
    terms: Vec<Term>,
    tx: SkimItemSender,
    rx: SkimItemReceiver,
}

impl TermMatcher{

    fn new(terms: Vec<Term>) -> Self {
        let (tx, rx) = unbounded(); // <- this should take a vec
        Self { terms, tx, rx }
    }
}

struct Term {
    uri: String,
    label: String,
}

impl SkimItem for Term {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.label)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(format!("{} ({})", self.label, self.uri))
    }
}

pub fn filter_terms(query: String, terms: Vec<Term>) {
    println!("Hello, world!");
}


// Build in-memory kg, load all sources and query for uris and labels.
fn query(readers: Vec<impl BufRead>) -> Vec<Term>{
    let store = Store::new().unwrap();
    readers.iter().for_each(|r| {
        store.load_graph(
            r,
            GraphFormat::Turtle,
            GraphNameRef::DefaultGraph,
            None,
        ).unwrap();
    });
    let results = store.query("
        SELECT ?uri ?label 
        WHERE { 
            ?uri <http://www.w3.org/2000/01/rdf-schema#label> ?label 
        }"
    ).unwrap();
    results
        .iter()
        .map(|r| Term {
            uri: r.get("uri"), label: r.get("label")
        })
        .collect()
        .into()
}
