use axum::{
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    state::AppState,
    models::{CreateQuestion, Question},
};

pub async fn home() -> &'static str {
    "WELCOME"
}

pub async fn list_questions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Question>>,StatusCode> {
    

    let questions = state
        .questions
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(questions.clone()))

}

pub async fn get_question(Path(id): Path<u64>) -> Result<String, StatusCode> {
    if id == 1 {
        Ok(format!("Showing question {}", id))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_question(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuestion>,
) -> Result<Json<Question>,StatusCode> {
    
    let mut questions = state
        .questions
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    let question = Question {
        id: questions.len() as u64 + 1,
        title: payload.title,
        body: payload.body,
        category: payload.category
    };

    questions.push(question.clone());

    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<u64>, 
    Json(payload): Json<CreateQuestion>
    ) -> Result<Json<Question>,StatusCode> {

    let mut questions = state
        .questions
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let question = questions
        .iter_mut()
        .find(|q| q.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    question.title = payload.title;
    question.body = payload.body;
    question.category = payload.category;

    Ok(Json(question.clone()))
}

pub async fn delete_question(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, StatusCode> {

    let mut questions = state
        .questions
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let original_len = questions.len();

    questions.retain(|q| q.id != id);

    if questions.len() == original_len {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
    
}