use axum::{
    Json,
    extract::Path,
};

use crate::models::{CreateQuestion, Question};

pub async fn home() -> &'static str {
    "WELCOME"
}

pub async fn list_questions() -> &'static str {
    "Listing all questions"
}

pub async fn get_question(Path(id): Path<u64>) -> String {
    format!("Show question {id}")
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

pub async fn update_question(Path(id): Path<u64>) -> String {
    format!("Question {id} updated")
}

pub async fn delete_question(Path(id): Path<u64>) -> String {
    format!("Question {id} deleted")
}