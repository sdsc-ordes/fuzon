use clap::Parser;

use anyhow::Result;


/// fuzzy match terms from ontologies to get their uri
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The query to search for in the ontology.
    #[clap(short, long)]
    query: String,
    /// File to search. Can be a file path or a URL.
    #[clap(short, long, required = true)]
    source: Vec<String>,

    /// Only return the top N results.
    #[clap(short, long)]
    top: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sources = args.source.iter().map(|s| s.as_str()).collect();
    let matcher = fuzon::TermMatcher::from_paths(sources)?;
    let mut results = matcher.rank_terms(args.query);

    if let Some(top_n) = args.top {
        let take_n = top_n.min(results.len());
        results = results[..take_n].to_vec();
    }

    for (term, score) in results {
        println!("[{}] {}", score, term)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_urls() {}

}
