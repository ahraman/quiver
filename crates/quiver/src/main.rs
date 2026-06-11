use std::{net::SocketAddr, sync::Arc};

use axum::{ServiceExt, extract::Request};
use quiver::{App, Error, app::AppConfig};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let app = Arc::new(App::new(AppConfig::new()?)?);
    let router = NormalizePathLayer::trim_trailing_slash().layer(app.build_router());

    let server_url = &app.config.env.server_url;
    let listener = TcpListener::bind(server_url).await?;
    println!("> listening at: {server_url}");
    Ok(axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(router),
    )
    .await?)
}
