use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use clap::Parser;
use fuzon::{TermMatcher};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use std::fs::File;

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
    host: String,
    port: u16,
    collections: HashMap<String, Vec<String>>,
}

// Shared app state built from config and used by services
#[derive(Clone, Debug)]
struct AppState {
    collections: Arc<HashMap<String, TermMatcher>>,
}

impl AppState {
    fn from_config(data: Config) -> Self {
        let collections = data.clone()
            .collections
            .into_iter()
            .inspect(|(k, _)| println!("Loading collection: {}...", k))
            .map(|(k, v)| (
                k, 
                TermMatcher::from_paths(
                    v.iter().map(|s| &**s).collect()).unwrap()
                )
            )
            .collect();

        println!("Initialized with: {:?}", &data);
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

/// http server to serve the fuzon terminology matching api
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file.
    #[clap(short, long)]
    config: String,
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let config_path = args.config;

    let config: Config = serde_json::from_reader(
        File::open(config_path).expect("Failed to open config file.")
    ).expect("Failed to parse config.");
    let host = config.host.clone();
    let port = config.port as u16;

    let data = web::block(move || 
        AppState::from_config(config)
    )
        .await
        .expect("Failed to initialize state from config.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(list)
            .service(top)
    })
    .bind((host, port))?
    .run()
    .await
}
