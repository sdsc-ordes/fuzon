use std::{
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use anyhow::Result;
use reqwest::{blocking::Client, Url};

use crate::TermMatcher;

/// Requests headers with redirection to create a stamp for the URL
/// consisting of the last modified date and/or ETag.
pub fn get_url_stamp(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client.head(url).send()?;
    let headers = response.headers();
    let etag = headers.get("ETag").map_or("", |v| v.to_str().unwrap());
    let last_modified = headers
        .get("Last-Modified")
        .map_or("", |v| v.to_str().unwrap());

    Ok(format!("{}-{}-{}", url, etag, last_modified))
}

/// Crafts a file metadata to create a stamp consisting of the file path,
/// size and last modified date.
pub fn get_file_stamp(path: &str) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();
    let modified = metadata.modified()?;

    Ok(format!("{}-{}-{:?}", path, size, modified))
}

/// Generate a fixed cache key based on a collection of source paths.
/// Each path is converted to a stamp in the format "{path}-{fingerprint}-{modified-date}".
/// Stamps are then concatenated and hash of this concatenation is returned.
pub fn get_cache_key(paths: &mut Vec<&str>) -> Result<String> {
    paths.sort();

    // Craft all stamps and concatenate them into the hasher
    let mut state = DefaultHasher::new();
    for path in paths.iter() {
        let stamp = if let Ok(_) = Url::parse(path) {
            get_url_stamp(&path)?
        } else if PathBuf::from(path).exists() {
            get_file_stamp(&path)?
        } else {
            return Err(anyhow::anyhow!("Invalid path: {}", path));
        };
        stamp.hash(&mut state);
    }

    // Hash the concatenated stamps
    let key = state.finish();

    Ok(key.to_string())
}

/// Get the full cross-platform cache path for a collection of source paths.
pub fn get_cache_path(sources: &mut Vec<&str>) -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir().unwrap().join("fuzon");
    let cache_key = get_cache_key(sources)?;
    let cache_path = cache_dir.join(&cache_key);

    Ok(cache_path)
}

/// Save each source into an independent TermMatcher cache file.
pub fn cache_by_source(sources: Vec<&str>) -> Result<()> {
    for source in sources {
        let matcher = TermMatcher::from_paths(vec![source])?;
        let cache_path = get_cache_path(&mut vec![source])?;
        matcher.dump(&cache_path)?;
    }

    Ok(())
}

/// Load and combine single-source cache entries into a combined TermMatcher.
pub fn load_by_source(sources: Vec<&str>) -> Result<TermMatcher> {
    let mut matcher: TermMatcher = TermMatcher { terms: Vec::new() };

    for source in sources {
        let cache_path = get_cache_path(&mut vec![source])?;
        matcher = matcher + TermMatcher::load(&cache_path)?;
    }

    Ok(matcher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_stamp() {
        let path = "Cargo.toml";
        let stamp = get_file_stamp(path).unwrap();
        assert!(stamp.starts_with(path));
    }

    #[test]
    fn url_no_headers() {
        let url = "https://google.com";
        let stamp = get_url_stamp(url).unwrap();
        assert_eq!(stamp, format!("{}--", url));
    }

    #[test]
    fn cache_path() {
        let mut sources = vec!["Cargo.toml", "https://www.rust-lang.org/"];
        let path = get_cache_path(&mut sources.clone()).unwrap();
        let key = get_cache_key(&mut sources).unwrap();
        assert!(path.ends_with(key));
    }
}
