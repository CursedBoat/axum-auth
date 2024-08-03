use axum::http::{header, Response, StatusCode};
use serde_json::json;

pub fn return_response_error(message: String, status_code: StatusCode) -> Response<String> {
    return Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!({
                "success": false,
                "data": {
                    "message": message
                },
            })
            .to_string()
            .into(),
        )
        .unwrap();
}
