use fuzon::ui::{interactive, search};
use std::fs;

use anyhow::Result;
use clap::Parser;
use fuzon::{cache::get_cache_path, TermMatcher};

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

    /// Do not load from cache.
    #[clap(short, long, default_value = "false")]
    no_cache: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut sources = args.source.iter().map(|s| s.as_str()).collect();

    // Attempt to load from cache
    let matcher: TermMatcher;
    if !args.no_cache {
        let cache_path = get_cache_path(&mut sources)?;
        let _ = fs::create_dir_all(cache_path.parent().unwrap());
        // Cache hit
        matcher = if let Ok(matcher) = TermMatcher::load(&cache_path) {
            matcher
        // Cache miss
        } else {
            let matcher = TermMatcher::from_paths(sources)?;
            matcher.dump(&cache_path)?;
            matcher
        };
    } else {
        matcher = TermMatcher::from_paths(sources)?;
    }

    // Search for query
    if let Some(query) = args.query {
        for (term, score) in search(&matcher, &query, args.top) {
            println!("[{}] {}", score, term)
        }
        Ok(())
    // Or interactively trigger search on keystrokes
    } else {
        interactive(&matcher, args.top)
    }
}

#[cfg(test)]
mod tests {}
