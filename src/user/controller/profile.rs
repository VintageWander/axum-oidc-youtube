use axum::{extract::State, routing::get, Router};
use crate::{user::model::LoggedInUser, web::Web, AppState, WebResult};

#[utoipa::path(
    get, 
    path = "/user/profile", 
    security(
        ("Open ID Connect" = [])
    )
)]
pub fn profile() -> Router<AppState> {
    async fn handler(
        State(AppState { db, oidc }): State<AppState>,
        LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        Ok(Web::ok("Get profile success", user))
    }
    Router::new().route("/profile", get(handler))
}
