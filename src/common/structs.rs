use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub user_id: String,
    pub created_at: i64,
    pub hashed_password: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UsersQuery {
    pub user_id: Option<String>,
    pub created_at: Option<i64>,
    pub hashed_password: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Sessions {
    pub session_id: String,
    pub user_id: String,
    pub created_at: i64,
    pub expire_at: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionsQuery {
    pub session_id: Option<String>,
    pub user_id: Option<String>,
    pub created_at: Option<i64>,
    pub expire_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}