use axum::Router;

use crate::AppState;

pub use self::controller::{callback::callback, login::login};

pub mod controller;
pub mod model;
pub mod utils;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/auth", Router::new().merge(login()).merge(callback()))
}
