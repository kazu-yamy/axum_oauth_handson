use anyhow::Context;
use async_session::MemoryStore;
use axum::{routing::get, Router};
use oauth::{
    auth::discord_auth, client, index::index, login::login_authorized, logout::logout,
    protected::protected,
};
use structure::app_state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod oauth;
mod structure;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let store = MemoryStore::new();
    let oauth_client = client::oauth_client().unwrap();
    let app_state = AppState {
        store,
        oauth_client,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/auth/discord", get(discord_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    tracing::debug!(
        "listen on {}",
        listener
            .local_addr()
            .context("failed to return address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
