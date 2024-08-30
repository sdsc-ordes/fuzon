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
    /// List of ontology files to search. Can be a file path or a URL.
    #[clap(short, long, required = true)]
    terminology: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut readers = Vec::new();
    for path in args.terminology {
        readers.push(BufReader::new(File::open(path).unwrap()));
    }
    
    let matcher = fuzon::TermMatcher::from_readers(readers);
    println!("query finished");
    let ranked = matcher.rank_terms(args.query);
    println!("matching finished");

    for (term, score) in ranked {
        println!("[{}] {}", score, term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_urls() {}

}
