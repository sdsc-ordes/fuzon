use crate::api::{AppState, list_collections, top_codes};
use std::collections::HashMap;
use actix_web::{
    middleware::Logger,
    web::{block, Data},
    App,
    HttpServer,
};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{
    get,
    resource,
    scope,
};
use apistos::ScalarConfig;
use clap::Parser;
use fuzon::TermMatcher;
use log::info;
use serde::Deserialize;
use serde_json;
use std::env;
use std::sync::Arc;
use std::fs::File;

mod api;

// Config file structure
# [derive(Clone, Debug, Deserialize)]
struct Config {
    host: String,
    port: u16,
    collections: HashMap<String, Vec<String>>,
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

    let data = block(move || 
        AppState::from_config(config)
    )
        .await
        .expect("Failed to initialize state from config.");

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "Fuzon API".to_string(),
                version: "0.3.0".to_string(),
                description: Some("API for fuzzy terminology matching.".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        App::new()
            .document(spec)
            .wrap(Logger::default())
            .app_data(Data::new(data.clone()))
            .service(resource("/collections").route(get().to(list_collections)))
            .service(
                scope("/codes")
                    .service(resource("/top").route(get().to(top_codes)))
            )
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(ScalarConfig::new(&"/")),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
