use std::{
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use anyhow::Result;
use reqwest::{blocking::Client, Url};

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

    return Ok(format!("{}-{}-{}", url, etag, last_modified));
}

/// Crafts a file metadata to create a stamp consisting of the file path,
/// size and last modified date.
pub fn get_file_stamp(path: &str) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();
    let modified = metadata.modified()?;

    return Ok(format!("{}-{}-{:?}", path, size, modified));
}

/// Generate a fixed cache key based on a collection of source paths.
/// Each path is converted to a stamp in the format "{path}-{fingerprint}-{modified-date}".
/// Stamps are then concatenated and hash of this concatenation is returned.
pub fn get_cache_key(sources: &Vec<&str>) -> Result<String> {
    let mut paths = sources.clone();
    paths.sort();

    // Craft all stamps and concatenate them
    let mut concat = String::new();
    for path in paths.into_iter() {
        if !PathBuf::from(path).exists() && Url::parse(path).is_err() {
            return Err(anyhow::anyhow!("Invalid path: {}", path));
        }

        let stamp = if let Ok(_) = Url::parse(path) {
            get_url_stamp(&path)?
        } else {
            get_file_stamp(&path)?
        };
        concat.push_str(&stamp);
    }

    // Hash the concatenated stamps
    let mut state = DefaultHasher::new();
    concat.hash(&mut state);
    let key = state.finish();

    return Ok(key.to_string());
}

/// Get the full cross-platform cache path for a collection of source paths.
pub fn get_cache_path(sources: &Vec<&str>) -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir().unwrap().join("fuzon");
    let cache_key = get_cache_key(&sources);
    let cache_path = cache_dir.join(&cache_key?);

    return Ok(cache_path);
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
        let sources = vec!["Cargo.toml", "https://www.rust-lang.org/"];
        let path = get_cache_path(&sources).unwrap();
        let key = get_cache_key(&sources).unwrap();
        assert!(path.ends_with(key));
    }
}
