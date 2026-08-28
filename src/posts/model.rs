use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow,Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
}
#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: Option<String>,
}
#[derive(Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}
