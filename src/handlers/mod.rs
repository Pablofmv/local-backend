use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};


use axum::{
    extract::{Json, Path, State}, http::{HeaderMap, StatusCode}
};

use sha2::{Digest, Sha256};

use crate::{
    state::AppState,
    models::{CreateQuestion, CreateUser, LoginRequest, LoginResponse, Question, User, UserResponse},
};



fn extract_bearer_token(
    headers: &HeaderMap,
) -> Result <String,StatusCode> {

    let authorization = headers
        .get("authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let authorization = authorization
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if token.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(token.to_string())

}

fn hash_session_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}


async fn authenticate_session(
    state: &AppState,
    headers: &HeaderMap,
)-> Result<i32, StatusCode> {

    let token = extract_bearer_token(headers)?;

    let token_hash = hash_session_token(&token);

    let session = sqlx::query_as::<_, (i32,)>(
        r#"
            SELECT user_id
            FROM sessions
            WHERE token_hash = $1
            AND expires_at > NOW()
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("Failed to validate session: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match session{
        Some((user_id,)) => Ok(user_id),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}





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
    headers: HeaderMap,
    Json(payload): Json<CreateQuestion>,
) -> Result<Json<Question>,StatusCode> {

    let user_id = authenticate_session(
        &state, 
        &headers
    )
    .await?;
    
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
    .bind(user_id)
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
) -> Result<Json<UserResponse>,StatusCode>
{   

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
    .hash_password(payload.password.as_bytes(), &salt)
    .map_err(|error| {
        eprintln!("Failted to hash password: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .to_string();

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
    .bind(password_hash)
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

    let response = UserResponse {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    };

    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>,StatusCode> {

    let user = sqlx::query_as::<_,User>(
        r#"
        SELECT
            id,
            email,
            nickname,
            password_hash
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("Failed to read user during login: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = match user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED)
    };

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|error|{
            eprintln!("failed to parse stored password hash:{error}");
            StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash,)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut token_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut token_bytes);

    let token = hex::encode(token_bytes);

    let token_hash = hash_session_token(&token);

    sqlx::query(
        r#"
        INSERT INTO sessions(
            user_id,
            token_hash,
            expires_at
        )
        VALUES(
            $1,
            $2,
            NOW() + INTERVAL '30 days'
        )
        "#,
    )
    .bind(user.id)
    .bind(token_hash)
    .execute(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("failed to create session: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(LoginResponse { 
        token,
     }))

}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {

    let token = extract_bearer_token(&headers)?;

    let token_hash = hash_session_token(&token);

    let result = sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE token_hash  = $1
        "#,
    )
    .bind(token_hash)
    .execute(&state.pool)
    .await
    .map_err(|error|{
        eprintln!("failed to delete session: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(StatusCode::NO_CONTENT)

}