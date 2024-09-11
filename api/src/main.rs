#![feature(duration_constructors)]

use axum::Router;
use axum::routing::post;
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

mod login;
mod join_server;
mod jwt;
mod open_session;
mod error;

static PRIVATE_KEY: &[u8] = include_bytes!("../../jwt.private.pem");
static PUBLIC_KEY: &[u8] = include_bytes!("../../jwt.public.pem");

#[tokio::main]
async fn main() {

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
        .route("/login", post(login))
        .route("/session", post(open_session))
        .route("/join", post(join_server))
        .layer(
            TraceLayer::new_for_http()
        )
        .layer(cors);

    println!("Listening on http://localhost:3000/");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
