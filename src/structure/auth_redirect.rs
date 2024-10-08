use axum::response::{IntoResponse, Redirect, Response};

#[derive(Debug)]
pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/discord").into_response()
    }
}
