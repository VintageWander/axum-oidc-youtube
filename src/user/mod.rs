use axum::Router;

use crate::AppState;

pub use self::controller::profile::{__path_profile, profile};

pub mod controller;
pub mod model;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/user", Router::new().merge(profile()))
}
