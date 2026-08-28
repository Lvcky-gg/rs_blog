use axum::{routing::get, Router};
use sqlx::{PgPool};

mod state;
mod posts;

use posts::view::post_router;
use state::app_state::AppState;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("failed to init database");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    let state = AppState {pool};
    let app = Router::new()
        .route("/", get(|| async { "HELLO" }))
        .merge(post_router())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

