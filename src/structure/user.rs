use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    RequestPartsExt,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{header, request::Parts};
use serde::{Deserialize, Serialize};

use super::auth_redirect::AuthRedirect;

static COOKIE_NAME: &str = "SESSION";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub avater: Option<String>,
    pub username: String,
    pub discriminator: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;

        let session_cookie = cookies
            .get(COOKIE_NAME)
            .ok_or(AuthRedirect)?
            .trim_matches('"');

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}
