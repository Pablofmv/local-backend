mod routes;
mod handlers;
mod models;
mod state;

use sqlx::postgres::PgPoolOptions;

use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::state::AppState;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    println!("{database_url}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    println!("Connected to PostgreSQL successfully");

    let _ = &pool;

    let state = AppState {
        questions: Arc::new(Mutex::new(Vec::new())),
    };
    
    let app = routes::create_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("LOCAL server running at http//127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();

}
