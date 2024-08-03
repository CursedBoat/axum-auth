use axum::{
    body::Body, extract::Request, http::{
        header::{self, SET_COOKIE}, Response, StatusCode
    }, Extension
};
use cookie::Cookie;
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::common::error::return_response_error;

pub async fn logout_handler(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    req: Request<Body>,
) -> Response<String> {
    // extract cookie from header
    let cookies = req.headers().get("cookie").unwrap();
    let cookies = cookies.to_str().unwrap_or("");

    // execute logic if cookie exists
    if let Some(session_cookie) = cookies
        .split(';')
        .find(|c| c.trim().starts_with("session="))
    {
        // extract id from session cookie
        let session_token = session_cookie.trim().strip_prefix("session=").unwrap_or("");

        // query database
        let token_query = sqlx::query_as!(
            SessionsQuery,
            "DELETE FROM sessions WHERE session_id = ?",
            session_token
        )
        .fetch_one(&db_pool)
        .await;

        // handle errors
        match token_query {
            Ok(_) => {}
            Err(_) => {
                return_response_error("Could not log out.".to_string(), StatusCode::UNAUTHORIZED);
            }
        }
    }

    // reset cookies clientside
    let cookie = Cookie::build(("session", ""))
        .path("/session").to_string();

    // return confirmation message
    return Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(SET_COOKIE, cookie)
        .body(
            json!({
                "success": true,
                "data": {
                    "message": "User logged out."
                },
            })
            .to_string()
            .into(),
        )
        .unwrap();
}