use serde::{Deserialize,Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub body: String,
    pub category: String,
}


#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub category: String,
    pub community: String,
    pub region: String,
    pub state: String,
}


#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub nickname: String,
    pub password_hash:String,
}


#[derive(Clone,Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub nickname: String,
    pub password: String,
}