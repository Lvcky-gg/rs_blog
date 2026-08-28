use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::AppState;
use crate::posts::model::{Post, CreatePost, UpdatePost};

pub async fn create_post(
    State(state):State<AppState>,
    Json(payload):Json<CreatePost>,
    ) -> Result<(StatusCode, Json<Post>),StatusCode>{

        let post = sqlx::query_as::<_,Post>(
            "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content"
            )
            .bind(payload.title)
            .bind(payload.content)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok((StatusCode::CREATED, Json(post)))
    } 
pub async fn get_all_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>,StatusCode>{
    let posts = sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>
    )-> Result<Json<Post>, StatusCode>{
    let post = sqlx::query_as::<_,Post>("SELECT id, title, content FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}
pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePost>
    ) -> Result<Json<Post>, StatusCode> {
    let cur_post = sqlx::query_as::<_,Post>("SELECT id, title, content FROM posts where id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let fallback = cur_post.content.clone().unwrap_or_default();
    let title = payload.title.as_ref().unwrap_or(&cur_post.title);
    let content = payload.content.as_ref().unwrap_or(&fallback);



    let post = sqlx::query_as::<_,Post>("UPDATE posts SET title = $1, content = $2 WHERE id = $3 RETURNING id, title, content")
        .bind(title)
        .bind(content)
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}
pub async fn delete_post (
    State(state): State<AppState>,
    Path(id): Path<i32>,
    ) -> Result<StatusCode, StatusCode> {
    let res = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if res.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
