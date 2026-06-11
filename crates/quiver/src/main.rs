use std::net::SocketAddr;

use axum::{Router, ServiceExt, extract::Request, response::IntoResponse, routing::get};
use quiver::Error;
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let router =
        NormalizePathLayer::trim_trailing_slash().layer(Router::new().route("/", get(root)));

    let server_url =
        &std::env::var("SERVER_URL").map_err(|e| Error::Env("SERVER_URL".to_string(), e))?;
    let listener = TcpListener::bind(server_url).await?;
    println!("> listening at: {server_url}");
    Ok(axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(router),
    )
    .await?)
}

async fn root() -> impl IntoResponse {
    "Hello, world!"
}
