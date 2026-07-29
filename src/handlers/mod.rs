use axum::{
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
};

use crate::models::{CreateQuestion, Question};

pub async fn home() -> &'static str {
    "WELCOME"
}

pub async fn list_questions() -> &'static str {
    "Listing all questions"
}

pub async fn get_question(Path(id): Path<u64>) -> Result<String, StatusCode> {
    if id == 1 {
        Ok(format!("Showing question {}", id))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_question(
    Json(payload): Json<CreateQuestion>
) -> Json<Question> {
    let question = Question {
        id: 1,
        title: payload.title,
        body: payload.body,
        category: payload.category
    };

    Json(question)
}

pub async fn update_question(
    Path(id): Path<u64>, 
    Json(payload): Json<CreateQuestion>
    ) -> Json<Question> {
    
    let question = Question {
        id,
        title: payload.title,
        body: payload.body,
        category: payload.category,
    };

    Json(question)
}

pub async fn delete_question(Path(id): Path<u64>) -> String {
    format!("Question {id} deleted")
}