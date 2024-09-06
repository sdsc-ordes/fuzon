use fuzon::ui::{interactive, search};

use anyhow::Result;
use clap::Parser;
use fuzon::TermMatcher;

/// fuzzy match terms from ontologies to get their uri
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The query to search for in the ontology.
    #[clap(short, long)]
    query: Option<String>,
    /// File to search. Can be a file path or a URL.
    #[clap(short, long, required = true)]
    source: Vec<String>,

    /// Only return the top N results.
    #[clap(short, long)]
    top: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sources = args.source
        .iter()
        .map(|s| s.as_str())
        .collect();
    let matcher = TermMatcher::from_paths(sources)?;

    if let Some(query) = args.query {
        for (term, score) in search(&matcher, &query, args.top) {
            println!("[{}] {}", score, term)
        }
        return Ok(());
    } else {
        return interactive(&matcher, args.top);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn match_urls() {}
}
