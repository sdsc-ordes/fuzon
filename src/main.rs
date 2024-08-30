use clap::Parser;
use std::fs::File;
use std::io::BufReader;


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

fn main() {
    let args = Args::parse();
    let mut readers = Vec::new();
    for path in args.source {
        readers.push(BufReader::new(File::open(path).unwrap()));
    }
    
    let matcher = fuzon::TermMatcher::from_readers(readers);
    let mut results = matcher.rank_terms(args.query);
    if let Some(n) = args.top {
        results = results[..n].to_vec();
    }

    for (term, score) in results {
        println!("[{}] {}", score, term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_urls() {}

}
