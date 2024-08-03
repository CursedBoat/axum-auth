use crate::common::error::return_response_error;
use crate::common::structs::SessionsQuery;
use axum::{
    extract::Request,
    http::{header::SET_COOKIE, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use cookie::Cookie;
use sqlx::SqlitePool;
use tracing::info;
use axum::http::header;
use serde_json::json;

pub async fn require_session(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response<String>> {
    // get database pool
    let db_pool = match req.extensions().get::<SqlitePool>() {
        Some(pool) => pool,
        None => {
            info!("Database connection not found.");
            return Err(return_response_error(
                "Database connection not found.".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    // extract cookies from header
    let cookies = req.headers().get("cookie");

    // execute if cookies exist
    if let Some(cookies) = cookies {
        let cookies = cookies.to_str().unwrap_or("");

        // split cookies and find session information
        if let Some(session_cookie) = cookies
            .split(';')
            .find(|c| c.trim().starts_with("session=")) {   
            // grab session id from the cookie
            let session_id = session_cookie.trim().strip_prefix("session=").unwrap_or("");

            // database query to ensure that the session exists
            let token_query = sqlx::query_as!(
                SessionsQuery,
                "SELECT * FROM sessions WHERE session_id = ?",
                session_id
            )
            .fetch_one(db_pool)
            .await;

            match token_query {
                Ok(result) => {
                    // verify that the tokens have not expired
                    let time_now = chrono::Utc::now().timestamp();
                    info!(result.expire_at);
                    info!(time_now);
                    if result.expire_at < Some(time_now) {
                        // delete cookie from server
                        let _token_query = sqlx::query_as!(
                            SessionsQuery,
                            "DELETE FROM sessions WHERE session_id = ?",
                            session_id
                        )
                        .fetch_one(&*db_pool)
                        .await;

                        // reset cookies clientside
                        let cookie = Cookie::build(("session", ""))
                            .path("/session").to_string();
                        
                        // return error response
                        return Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .header(header::CONTENT_TYPE, "application/json")
                            .header(SET_COOKIE, cookie)
                            .body::<String>(
                                json!({
                                    "success": false,
                                    "data": {
                                        "message": format!("Session {} expired", session_id)
                                    },
                                })
                                .to_string()
                                .into(),
                            )
                            .unwrap())
                    }

                    // insert session information as an extension
                    req.extensions_mut().insert(result);

                    // go to the endpoint if session is valid
                    return Ok(next.run(req).await);
                }
                Err(_) => {
                    // return error if the user is not logged in
                    return_response_error(
                        "You are not logged in.".to_string(),
                        StatusCode::UNAUTHORIZED,
                    );
                }
            }
        }
    }

    // return error if cookie doesn't exist
    Err(return_response_error(
        "You are not authorized to perform this action.".to_string(),
        StatusCode::UNAUTHORIZED,
    ))
}
