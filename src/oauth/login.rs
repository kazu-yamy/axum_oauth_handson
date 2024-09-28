use anyhow::Context;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use http::{header::SET_COOKIE, HeaderMap};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};

use crate::structure::{app_error::AppError, auth_request::AuthRequest, user::User};

static COOKIE_NAME: &str = "SESSION";

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient>,
) -> Result<impl IntoResponse, AppError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request to authorization server")?;

    let client = reqwest::Client::new();
    let user_data: User = client
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to deserialize response as JSON")?;

    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")
        .context("unexpected error retrieving cookie value")?
        .unwrap();

    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}
