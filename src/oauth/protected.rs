use axum::response::IntoResponse;

use crate::structure::user::User;

pub async fn protected(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
}
