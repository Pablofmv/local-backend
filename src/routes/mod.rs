use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::*,
    state::AppState,
};

pub fn create_router() -> Router<AppState> {

    Router::new()
        .route("/", get(home))
        .route("/questions", get(list_questions))
        .route("/questions", post(create_question))
        .route("/questions/{id}", get(get_question))
        .route("/questions/{id}", put(update_question))
        .route("/questions/{id}", delete(delete_question))
        .route("/users", post(create_user))
        .route("/login",post(login))
        .route("/logout",post(logout))
    }