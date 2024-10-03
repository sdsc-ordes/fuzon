mod errors;

use actix_web::{get, middleware, rt::Runtime, web, App, HttpServer, Responder, Result};
use clap::Parser;
use errors::ApiError;
use fuzon::TermMatcher;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, env, fs::File, sync::Arc};

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
#[derive(Clone, Debug, Deserialize)]
struct Config {
    host: String,
    port: u16,
    // TODO: Add doc what these dict maps.
    collections: HashMap<String, Vec<String>>,
}

// Shared app state built from config and used by services
#[derive(Clone, Debug)]
struct AppState {
    // TODO: Add doc what these dict maps.
    collections: Arc<HashMap<String, TermMatcher>>,
}

impl AppState {
    fn from_config(data: Config) -> Self {
        let collections = data
            .collections
            .iter()
            .inspect(|(k, _)| info!("Loading collection: {}...", k))
            .map(|(k, v)| {
                let s = v.iter().map(|s| s.as_str()).collect();
                (k.clone(), TermMatcher::from_paths(s).unwrap())
            })
            .collect();

        info!("Initialized with: {:?}", &data);

        AppState {
            collections: Arc::new(collections),
        }
    }
}

// The handler which returns the list collections on endpoint `/list`.
#[get("/list")]
async fn list(data: web::Data<AppState>) -> impl Responder {
    //TODO:Define a proper type for this response
    let mut response = HashMap::new();
    let collections: Vec<String> = data.collections.keys().cloned().collect();
    response.insert("collections".to_string(), collections);

    web::Json(response)
}

// Top matching codes from
// collection for query: /top?collection={collection}&query={foobar}&top={10}
#[get("/top")]
async fn top(
    data: web::Data<AppState>,
    req: web::Query<CodeRequest>,
) -> Result<impl Responder, ApiError> {
    // Get the collection.
    let Some(matcher) = data.collections.get(&req.collection) else {
        return Err(ApiError::MissingCollection(req.collection.clone()));
    };

    // Match all terms.
    let top_terms: Vec<CodeMatch> = matcher
        .top_terms(&req.query, req.top)
        .iter()
        .map(|t| CodeMatch {
            label: t.label.clone(),
            uri: t.uri.clone(),
            score: None,
        })
        .collect();

    Ok(web::Json(MatchResponse { codes: top_terms }))
}

/// Http server to serve the fuzon terminology matching api.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file.
    #[clap(short, long)]
    config: String,
}

async fn async_main() -> std::io::Result<()> {
    let args = Args::parse();
    let config_path = args.config;

    let config: Config =
        serde_json::from_reader(File::open(config_path).expect("Failed to open config file."))
            .expect("Failed to parse config.");
    let host = config.host.clone();
    let port = config.port as u16;

    let data = AppState::from_config(config);

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

fn main() -> Result<()> {
    // TODO: This is unsafe: not sure what to do though -> maybe block the executor and set env
    // variables before this asyn function is started.
    // Strange that Rust compiled anyway, without the unsafe block, any idea?
    unsafe {
        env::set_var(
            "RUST_LOG",
            "fuzon_http=info,actix_web=warn,actix_server=info",
        );
    }

    env_logger::init();

    let rt = Runtime::new()?;
    rt.block_on(async_main())?;
    Ok(())
}
