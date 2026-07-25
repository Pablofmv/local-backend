use axum::extract::Path;

pub async fn home() -> &'static str {
    "WELCOME"
}

pub async fn list_questions() -> &'static str {
    "Listing all questions"
}

pub async fn get_question(Path(id): Path<u64>) -> String {
    format!("Show question {id}")
}

pub async fn create_question() -> &'static str {
    "Question created"
}

pub async fn update_question(Path(id): Path<u64>) -> String {
    format!("Question {id} updated")
}

pub async fn delete_question(Path(id): Path<u64>) -> String {
    format!("Question {id} deleted")
}