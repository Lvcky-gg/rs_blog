use axum::{routing::{get,put}, Router};

use crate::state::app_state::AppState;
use crate::posts::controller::{get_all_posts, create_post,update_post,delete_post,get_post};

pub fn  post_router() -> Router<AppState>{
    Router::new()
        .route("/post",get(get_all_posts).post(create_post))
        .route("/post/:id", put(update_post).delete(delete_post).get(get_post))
    }
