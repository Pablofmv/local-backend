use serde::{Deserialize,Serialize};

#[derive(Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub body: String,
    pub category: String,
}


#[derive(Serialize)]
pub struct Question {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub category: String,
}