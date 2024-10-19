#![feature(duration_constructors)]

use std::cell::OnceCell;
use std::fs;
use std::sync::OnceLock;
use axum::Router;
use axum::routing::{post, get};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::login::login;
use crate::join_server::join_server;
use crate::open_session::open_session;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::filter::Targets;
use http::header::{CONTENT_TYPE};
use http::Method;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::from_filename;

mod login;
mod join_server;
mod jwt;
mod open_session;
mod error;

static PRIVATE_KEY: OnceLock<Vec<u8>> = OnceLock::new();
static PUBLIC_KEY: OnceLock<Vec<u8>> = OnceLock::new();

#[tokio::main]
async fn main() {
    from_filename("../.env").unwrap();

    // Include jwt keys
    PRIVATE_KEY.set(std::env::var("PRIVATE_JWT_KEY").expect("JWT_PRIVATE_KEY not set").into_bytes()).unwrap();
    PUBLIC_KEY.set(std::env::var("PUBLIC_JWT_KEY").expect("JWT_PUBLIC_KEY not set").into_bytes()).unwrap();

    let targets = Targets::new()
        .with_default(Level::TRACE);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(targets)
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/session", post(open_session))
        .route("/join", post(join_server))
        .layer(
            TraceLayer::new_for_http()
        )
        .layer(cors);

    println!("Listening on http://localhost:3001/");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn root() -> String {
    format!("Active")
}
