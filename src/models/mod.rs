use serde::{Deserialize,Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub body: String,
    pub category: String,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub category: String,
}