mod routes;
mod handlers;
mod models;
mod state;

use sqlx::postgres::PgPoolOptions;

use std::{
    env
};

use crate::state::AppState;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{database_url}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Connected to PostgreSQL successfully");

    let state = AppState {
        pool,
    };
    
    let app = routes::create_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Faailed to bind server address");

    println!("LOCAL server running at http//127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server Failed");

}
