use axum::{Router, routing::get};

use crate::post_views::controller::post;
use crate::state::app_state::AppState;

pub fn post_template_router() -> Router<AppState> {
    Router::new().route("/posts/{id}", get(post))
}
