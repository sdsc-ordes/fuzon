extern crate apistos_schemars as schemars;
use actix_web::{
    web::{Data, Json, Query},
    Result,
};
use apistos::{api_operation, ApiComponent};
use fuzon::TermMatcher;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Request for codes similar to an input text query.
#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct CodeRequest {
    /// Input free text query.
    query: String,
    /// Target collection on which to perform matching.
    collection: String,
    /// Number of top matches to return.
    top: usize,
}

/// Response model containing a single matched code.
#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct CodeMatch {
    /// Human-readable label associated with the code.
    label: String,
    /// Unique identifier for the code.
    uri: String,
    /// Similarity score (0 to 1, higher is better).
    score: Option<f64>,
}

/// Response model containing a list of matched codes.
#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct MatchResponse {
    /// Collection of matched codes.
    codes: Vec<CodeMatch>,
}

/// Response model containing the names of available collections.
#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct CollectionList {
    /// Names of available collections.
    collections: Vec<String>,
}

/// Shared app state built from config and used by services
#[derive(Clone, Debug)]
pub struct AppState {
    pub collections: Arc<HashMap<String, TermMatcher>>,
}

#[api_operation(
    tag = "collections",
    summary = "List available collections",
    description = "Returns the names of available collections."
)]
pub(crate) async fn list_collections(data: Data<AppState>) -> Result<Json<CollectionList>> {
    let collections = CollectionList {
        collections: data.collections.keys().cloned().collect()
    };

    Ok(Json(collections))

}

// Top matching codes from collection for query: /top?collection={collection}&query={foobar}&top={10}
#[api_operation(
    tag = "codes",
    summary = "Top N codes.",
    description = r###"Fuzzy matches the input query against the description of codes in target collection.
    The top N closest matches are returned."###,

)]
pub(crate) async fn top_codes(data: Data<AppState>, req: Query<CodeRequest>) -> Result<Json<MatchResponse>> {

    let top_terms: Vec<CodeMatch> = data.collections
        .get(&req.collection)
        .expect(&format!("Collection not found: {}", req.collection))
        .top_terms(&req.query, req.top)
        .into_iter()
        .map(|t| CodeMatch {
            label: t.label.clone(), uri: t.uri.clone(), score: None
        })
        .collect();

    Ok(Json(MatchResponse{ codes: top_terms }))
}
