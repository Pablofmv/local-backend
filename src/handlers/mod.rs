use axum::{
    extract::{Path, Json, State},
    http::StatusCode
};
use serde_json::error;

use crate::{
    state::AppState,
    models::{CreateQuestion, Question, User, CreateUser},
};

pub async fn home() -> &'static str {
    "WELCOME"
}

pub async fn list_questions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Question>>,StatusCode> {

    let questions = sqlx::query_as::<_, Question>(
        r#"
        SELECT
            id,
            user_id,
            title,
            body,
            category,
            community,
            region,
            state
        FROM questions
        ORDER BY id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("Failed to read PostgreSQL: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(questions))

}

pub async fn get_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Question>, StatusCode> {
    
    let question = sqlx::query_as::<_,Question> (
        r#"
        SELECT
            id,
            user_id,
            title,
            body,
            category,
            community,
            region,
            state
        FROM questions
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to ready question PostgreSQL:{error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match question {
        Some(question) => Ok(Json(question)),
        None => Err(StatusCode::NOT_FOUND),
    }
}


pub async fn create_question(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuestion>,
) -> Result<Json<Question>,StatusCode> {
    
    let question = sqlx::query_as::<_, Question>(
        r#"
        INSERT INTO questions(
            user_id,
            title,
            body,
            category,
            community,
            region,
            state
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            user_id,
            title,
            body,
            category,
            community,
            region,
            state
        "#,
    )
    .bind(1)
    .bind(payload.title)
    .bind(payload.body)
    .bind(payload.category)
    .bind("PERUVIAN")
    .bind("REGION1")
    .bind("NEW YORK")
    .fetch_one(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("Failted to create question in PostgreSQL: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<i32>, 
    Json(payload): Json<CreateQuestion>
    ) -> Result<Json<Question>,StatusCode> {

        let question = sqlx::query_as::<_, Question>(
            r#"
            UPDATE questions
            SET
                title = $1,
                body = $2,
                category = $3
            WHERE id = $4
            RETURNING
                id,
                user_id,
                title,
                body,
                category,
                community,
                region,
                state
            "#,
        )
        .bind(payload.title)
        .bind(payload.body)
        .bind(payload.category)
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to update question in PostgreSQL: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        match question {
            Some(question) => Ok(Json(question)),
            None => Err(StatusCode::NOT_FOUND)
        }
}

pub async fn delete_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {

    let result  = sqlx::query(
        r#"
        DELETE FROM questions
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to delete question from PostgreSQL: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
    
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>,StatusCode>
{
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (
            email,
            nickname,
            password_hash
        )
        VALUES($1,$2,$3)
        RETURNING
            id,
            email,
            nickname,
            password_hash
        "#,
    )
    .bind(payload.email)
    .bind(payload.nickname)
    .bind(payload.password)
    .fetch_one(&state.pool)
    .await
    .map_err(|error|{

        if let sqlx::Error::Database(database_error) = &error {
            if database_error.code().as_deref() == Some("23505") {
                return StatusCode::CONFLICT;
            }
        }
        

        eprintln!("failed to create user in PostgreSQL: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(user))
}