use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;

use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::Url;


/// Requests headers with redirection to create a stamp for the URL
/// consisting of the last modified date and/or ETag.
pub fn get_url_stamp(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client.head(url).send().unwrap();
    let headers = response.headers();
    let etag = headers
        .get("ETag")
        .map_or("", |v| v.to_str().unwrap());
    let last_modified = headers
        .get("Last-Modified")
        .map_or("", |v| v.to_str().unwrap());
    return Ok(format!("{}-{}-{}", url, etag, last_modified));
}

/// Crafts a file metadata to create a stamp consisting of the file path,
/// size and last modified date.
pub fn get_file_stamp(path: &str) -> Result<String> {
    let metadata = fs::metadata(path).unwrap();
    let size = metadata.len();
    let modified = metadata.modified().unwrap();
    return Ok(format!("{}-{}-{:?}", path, size, modified));
}

/// Generate a fixed cache key based on a collection of source paths.
/// Each path is converted to a stamp in the format "{path}-{fingerprint}-{modified-date}".
/// Stamps are then concatenated and hash of this concatenation is returned.
pub fn get_cache_key(sources: &Vec<&str>) -> String {
    let mut paths = sources.clone();
    paths.sort();
    let concat = paths
        .into_iter()
        .map(|s|
            if let Ok(_) = Url::parse(s) {
                get_url_stamp(&s).unwrap()
            } else {
                get_file_stamp(&s).unwrap()
            }
        )
        .collect::<Vec<String>>()
        .join(" ");
    let mut state = DefaultHasher::new();
    concat.hash(&mut state);
    let key = state.finish();

    return key.to_string()
}

/// Get the full cross-platform cache path for a collection of source paths.
pub fn get_cache_path(sources: &Vec<&str>) -> PathBuf {

    let cache_dir = dirs::cache_dir().unwrap().join("fuzon");
    let cache_key = get_cache_key(
        &sources
    );

    return cache_dir.join(&cache_key)

}
