use axum::{
    http::{
        header::{self, SET_COOKIE}, Response, StatusCode
    }, Extension, Json
};
use bcrypt::hash_with_salt;
use chrono::Duration;
use cookie::Cookie;
use serde_json::json;
use sqlx::SqlitePool;

use crate::common::error::return_response_error;
use crate::common::structs::*;
use crate::common::{generate_random_bytes, generate_token, generate_uuid};

pub async fn register_handler(
    Extension(db_pool): Extension<SqlitePool>,
    Json(payload): Json<RegisterRequest>,
) -> Response<String> {
    // generate salt
    let salt = generate_random_bytes();

    // return error if either fields are empty
    if payload.username.is_empty()
        || payload.password.is_empty()
    {
        return_response_error("All fields required.".to_string(), StatusCode::BAD_REQUEST);
    }

    // hash password
    let hashed_password = hash_with_salt(payload.password, 12, salt)
        .unwrap()
        .to_string();

    // initialize users struct
    let user = Users {
        user_id: generate_uuid(),
        username: payload.username.clone(),
        hashed_password: hashed_password.clone(),
        created_at: chrono::Utc::now().timestamp(),
    };

    // initialize sessions struct
    let session = Sessions {
        session_id: generate_token(),
        user_id: user.user_id.clone(),
        created_at: chrono::Utc::now().timestamp(),
        expire_at: chrono::Utc::now().timestamp() + Duration::days(2).num_seconds(),
    };

    // create user, and handle errors
    if let Err(err) = sqlx::query!(
        "INSERT INTO users (user_id, created_at, hashed_password, username) VALUES (?, ?, ?, ?)",
        user.user_id,
        user.created_at,
        user.hashed_password,
        user.username,
    )
    .execute(&db_pool)
    .await
    {
        return_response_error(format!("Failed to execute query: {}", err), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // create session, and handle errors
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

    // make it so that the cookie works in /sessions/* endpoints
    // NOTE: users can change this directory, so set these permissions
    // properly in the router
    let cookie = Cookie::build(("session", session.session_id))
        .path("/session").to_string();

    // return success json response
    Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .header(SET_COOKIE, cookie)
        .body(
            json!({
                "success": true,
                "data": {
                    "message": "User created."
                },
                "userInfo": {
                    "username": user.username,
                    "id": user.user_id,
                }
            })
            .to_string()
            .into(),
        )
        .unwrap()
}
