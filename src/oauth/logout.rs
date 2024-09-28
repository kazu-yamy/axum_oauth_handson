use anyhow::Context;
use async_session::{MemoryStore, SessionStore};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_extra::{headers::Cookie, TypedHeader};

use crate::structure::app_error::AppError;

static COOKIE_NAME: &str = "SESSION";

pub async fn logout(
    State(store): State<MemoryStore>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error gettinng cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        None => return Ok(Redirect::to("/")),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}
