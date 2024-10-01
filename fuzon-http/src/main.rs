use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use fuzon::{TermMatcher};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

// URL query parameters when requesting matching codes
#[derive(Debug, Deserialize)]
pub struct CodeRequest {
    query: String,
    collection: String,
    top: usize,
}

// Response model containing matching codes
#[derive(Debug, Serialize)]
pub struct MatchResponse {
    label: String,
    uri: String,
    score: Option<f64>,
}

// Config file structure
# [derive(Clone, Debug, Deserialize)]
struct Config {
    collections: HashMap<String, String>,
}

// Shared app state built from config and used by services
#[derive(Clone, Debug)]
struct AppState {
    collections: Arc<HashMap<String, TermMatcher>>,
}

impl AppState {
    fn from_config(data: Config) -> Self {
        let collections = data
            .collections
            .into_iter()
            .map(|(k, v)| (k, TermMatcher::from_paths(vec![&v]).unwrap())).collect();
        AppState { collections: Arc::new(collections) }
    }
}


// list collections: /list
#[get("/list")]
async fn list(data: web::Data<AppState>) -> impl Responder {

    let collections : Vec<String> = data.collections.keys().cloned().collect();

    web::Json(collections)

}

// Top matching codes from collection for query: /top?collection={collection}&query={foobar}&top={10}
#[get("/top")]
async fn top(data: web::Data<AppState>, req: web::Query<CodeRequest>) -> Result<impl Responder> {

    let top_terms: Vec<MatchResponse> = data.collections
        .get(&req.collection)
        .expect(&format!("Collection not found: {}", req.collection))
        .top_terms(&req.query, req.top)
        .into_iter()
        .map(|t| MatchResponse {
            label: t.label.clone(), uri: t.uri.clone(), score: None
        })
        .collect();

    Ok(web::Json(top_terms))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    // NOTE: config as inline json for debugging, later this will be a config file
    let config_data = r#"
        {
            "collections": {
                "cell_type": "https://purl.obolibrary.org/obo/cl.owl",
                "source_material": "https://purl.obolibrary.org/obo/uberon.owl",
                "taxon_id": "https://purl.obolibrary.org/obo/ncbitaxon/subsets/taxslim.owl"
            }
        }"#;
    let data = web::block(move || 
        AppState::from_config(serde_json::from_str::<Config>(config_data).unwrap())
    )
        .await
        .expect("Failed to parse config");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(list)
            .service(top)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
