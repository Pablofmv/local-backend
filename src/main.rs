mod routes;
mod handlers;
mod models;

use handlers::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tokio::net::TcpListener;

use crate::handlers::list_questions;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/", get(home))
        .route("/questions", get(list_questions))
        .route("/questions", post(create_question))
        .route("/questions/{id}", get(get_question))
        .route("/questions/{id}", put(update_question))
        .route("/questions/{id}", delete(delete_question));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

}
