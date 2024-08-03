use axum::{
    http::{header, StatusCode},
    response::Response,
};
use serde_json::json;

// Returns a public message
pub async fn public_view_handler() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!({
                "success": true,
                "data": {
                    "message": "This message is available to all users.",
                }
            })
            .to_string(),
        )
        .unwrap_or_default()
}