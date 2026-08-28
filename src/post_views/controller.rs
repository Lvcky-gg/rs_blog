use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::post_views::model::PostTemplate;
use crate::posts::model::Post;
use crate::state::app_state::AppState;

pub async fn post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<PostTemplate, StatusCode> {
    let post = sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(PostTemplate {
        title: post.title,
        content: post.content.unwrap_or_default(),
    })
}
