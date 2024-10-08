use std::collections::HashMap;
use actix_web::{get, middleware, web, App, HttpServer, Responder, Result};
use clap::Parser;
use fuzon::TermMatcher;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
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
pub struct CodeMatch {
    label: String,
    uri: String,
    score: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct MatchResponse {
    codes: Vec<CodeMatch>,
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
            .inspect(|(k, _)| info!("Loading collection: {}...", k))
            .map(|(k, v)| (
                k, 
                TermMatcher::from_paths(
                    v.iter().map(|s| &**s).collect()).unwrap()
                )
            )
            .collect();

        info!("Initialized with: {:?}", &data);
        AppState { collections: Arc::new(collections) }
    }
}


// list collections: /list
#[get("/list")]
async fn list(data: web::Data<AppState>) -> impl Responder {
    let mut response = HashMap::new();
    let collections : Vec<String> = data.collections.keys().cloned().collect();
    response.insert("collections".to_string(), collections);

    web::Json(response)

}

// Top matching codes from collection for query: /top?collection={collection}&query={foobar}&top={10}
#[get("/top")]
async fn top(data: web::Data<AppState>, req: web::Query<CodeRequest>) -> Result<impl Responder> {

    let top_terms: Vec<CodeMatch> = data.collections
        .get(&req.collection)
        .expect(&format!("Collection not found: {}", req.collection))
        .top_terms(&req.query, req.top)
        .into_iter()
        .map(|t| CodeMatch {
            label: t.label.clone(), uri: t.uri.clone(), score: None
        })
        .collect();

    Ok(web::Json(MatchResponse{ codes: top_terms }))
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
    env::set_var("RUST_LOG", "fuzon_http=info,actix_web=warn,actix_server=info");
    env_logger::init();
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
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(data.clone()))
            .service(list)
            .service(top)
    })
    .bind((host, port))?
    .run()
    .await
}
