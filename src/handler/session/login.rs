use axum::{
    http::{
        header::{self, SET_COOKIE}, Response, StatusCode
    }, Extension, Json
};
use chrono::Duration;
use cookie::Cookie;
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::common::error::return_response_error;
use crate::common::structs::*;
use crate::common::generate_token;

pub async fn login_handler(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Json(payload): Json<LoginRequest>,
) -> Response<String> {
    // do nothing if the payload contains empty fields
    if payload.username.is_empty() || payload.password.is_empty() {
        return_response_error("All fields required.".to_string(), StatusCode::BAD_REQUEST);
    }

    // get user from database and handle errors
    let user = match sqlx::query_as!(
        UsersQuery,
        "SELECT * FROM users WHERE username = ?",
        payload.username
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(user) => user,
        Err(_) => return return_response_error("Invalid username or password".to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    };

    // verify password through bcrypt
    if bcrypt::verify(payload.password, &user.hashed_password.unwrap()).unwrap() == false {
        return return_response_error("Invalid username or password.".to_string(), StatusCode::UNAUTHORIZED);
    }

    // create session struct
    let session = Sessions {
        session_id: generate_token(),
        user_id: user.user_id.unwrap().clone(),
        created_at: chrono::Utc::now().timestamp(),
        expire_at: chrono::Utc::now().timestamp() + Duration::days(2).num_seconds(),
    };

    // insert session into db and handle errors
    if let Err(err) = sqlx::query!(
        "INSERT INTO sessions (session_id, user_id, created_at, expire_at) VALUES (?, ?, ?, ?)",
        session.session_id,
        session.user_id,
        session.created_at,
        session.expire_at
    )
    .execute(&db_pool)
    .await
    {
        return_response_error(format!("Failed to execute query: {}", err), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // insert cookie into headers
    let cookie = Cookie::build(("session", session.session_id))
        .path("/session").to_string();

    // return confirmation message
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(SET_COOKIE, cookie)
        .body(
            json!({
                "success": true,
                "data": {
                    "message": "User logged in."
                },
            })
            .to_string()
            .into(),
        )
        .unwrap()
}